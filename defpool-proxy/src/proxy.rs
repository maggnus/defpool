use crate::config::Config;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{error, info, warn};
use serde::Deserialize;

// Stratum V2 crates
use stratum_core::{
    codec_sv2::{HandshakeRole, StandardEitherFrame},
    framing_sv2::framing::Frame,
    noise_sv2::{Initiator, Responder},
    parsers_sv2::AnyMessage,
};

// Local implementations based on stratum-apps
use secp256k1::XOnlyPublicKey;

#[derive(Debug, Copy, Clone)]
pub struct Secp256k1PublicKey(pub XOnlyPublicKey);

impl std::str::FromStr for Secp256k1PublicKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = bs58::decode(s).with_check(None).into_vec()?;
        if decoded.len() < 34 {
            return Err(anyhow::anyhow!("Invalid key length"));
        }
        let key_version = u16::from_le_bytes(decoded[..2].try_into().unwrap());
        if key_version != 1 {
            return Err(anyhow::anyhow!("Invalid key version"));
        }
        let public = XOnlyPublicKey::from_slice(&decoded[2..])?;
        Ok(Secp256k1PublicKey(public))
    }
}

// Type aliases
type Message = AnyMessage<'static>;

// Error type for networking operations
#[derive(Debug)]
pub enum Error {
    HandshakeRemoteInvalidMessage,
    CodecError(stratum_core::codec_sv2::Error),
    SocketClosed,
}

impl From<stratum_core::codec_sv2::Error> for Error {
    fn from(e: stratum_core::codec_sv2::Error) -> Self {
        Error::CodecError(e)
    }
}

// NoiseTcpStream implementation based on stratum-apps
use stratum_core::{
    binary_sv2::{Deserialize as BinaryDeserialize, GetSize, Serialize},
    codec_sv2::{NoiseEncoder, StandardNoiseDecoder, State},
    framing_sv2::framing::HandShakeFrame,
    noise_sv2::{ELLSWIFT_ENCODING_SIZE, INITIATOR_EXPECTED_HANDSHAKE_MESSAGE_SIZE},
};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub struct NoiseTcpStream<Message: Serialize + BinaryDeserialize<'static> + GetSize + Send + 'static> {
    reader: NoiseTcpReadHalf<Message>,
    writer: NoiseTcpWriteHalf<Message>,
}

pub struct NoiseTcpReadHalf<Message: Serialize + BinaryDeserialize<'static> + GetSize + Send + 'static> {
    reader: OwnedReadHalf,
    decoder: StandardNoiseDecoder<Message>,
    state: State,
    current_frame_buf: Vec<u8>,
    bytes_read: usize,
}

pub struct NoiseTcpWriteHalf<Message: Serialize + BinaryDeserialize<'static> + GetSize + Send + 'static> {
    writer: OwnedWriteHalf,
    encoder: NoiseEncoder<Message>,
    state: State,
}

impl<Message> NoiseTcpStream<Message>
where
    Message: Serialize + BinaryDeserialize<'static> + GetSize + Send + 'static,
{
    pub async fn new(stream: TcpStream, role: HandshakeRole) -> Result<Self, Error> {
        let (mut reader, mut writer) = stream.into_split();

        let mut decoder = StandardNoiseDecoder::<Message>::new();
        let mut encoder = NoiseEncoder::<Message>::new();
        let mut state = State::initialized(role.clone());

        match role {
            HandshakeRole::Initiator(_) => {
                let mut responder_state = State::not_initialized(&role);
                let first_msg = state.step_0()?;
                send_message(&mut writer, first_msg.into(), &mut state, &mut encoder).await?;

                loop {
                    match receive_message(&mut reader, &mut responder_state, &mut decoder).await {
                        Ok(second_msg) => {
                            let handshake_frame: HandShakeFrame = second_msg
                                .try_into()
                                .map_err(|_| Error::HandshakeRemoteInvalidMessage)?;
                            let payload: [u8; INITIATOR_EXPECTED_HANDSHAKE_MESSAGE_SIZE] =
                                handshake_frame
                                    .get_payload_when_handshaking()
                                    .try_into()
                                    .map_err(|_| Error::HandshakeRemoteInvalidMessage)?;
                            let transport_state = state.step_2(payload)?;
                            state = transport_state;
                            break;
                        }
                        Err(Error::CodecError(stratum_core::codec_sv2::Error::MissingBytes(_))) => {
                            tokio::task::yield_now().await;
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
            HandshakeRole::Responder(_) => {
                let mut initiator_state = State::not_initialized(&role);

                loop {
                    match receive_message(&mut reader, &mut initiator_state, &mut decoder).await {
                        Ok(first_msg) => {
                            let handshake_frame: HandShakeFrame = first_msg
                                .try_into()
                                .map_err(|_| Error::HandshakeRemoteInvalidMessage)?;
                            let payload: [u8; ELLSWIFT_ENCODING_SIZE] = handshake_frame
                                .get_payload_when_handshaking()
                                .try_into()
                                .map_err(|_| Error::HandshakeRemoteInvalidMessage)?;
                            let (second_msg, transport_state) = state.step_1(payload)?;
                            send_message(&mut writer, second_msg.into(), &mut state, &mut encoder)
                                .await?;
                            state = transport_state;
                            break;
                        }
                        Err(Error::CodecError(stratum_core::codec_sv2::Error::MissingBytes(_))) => {
                            tokio::task::yield_now().await;
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
        };

        Ok(Self {
            reader: NoiseTcpReadHalf {
                reader,
                decoder,
                state: state.clone(),
                current_frame_buf: vec![],
                bytes_read: 0,
            },
            writer: NoiseTcpWriteHalf {
                writer,
                encoder,
                state,
            },
        })
    }

    pub fn into_split(self) -> (NoiseTcpReadHalf<Message>, NoiseTcpWriteHalf<Message>) {
        (self.reader, self.writer)
    }
}

impl<Message> NoiseTcpReadHalf<Message>
where
    Message: Serialize + BinaryDeserialize<'static> + GetSize + Send + 'static,
{
    pub async fn read_frame(&mut self) -> Result<StandardEitherFrame<Message>, Error> {
        loop {
            let expected = self.decoder.writable_len();

            if self.current_frame_buf.len() != expected {
                self.current_frame_buf.resize(expected, 0);
                self.bytes_read = 0;
            }

            while self.bytes_read < expected {
                let n = self
                    .reader
                    .read(&mut self.current_frame_buf[self.bytes_read..])
                    .await
                    .map_err(|_| Error::SocketClosed)?;

                if n == 0 {
                    return Err(Error::SocketClosed);
                }

                self.bytes_read += n;
            }

            self.decoder
                .writable()
                .copy_from_slice(&self.current_frame_buf[..]);

            self.bytes_read = 0;

            match self.decoder.next_frame(&mut self.state) {
                Ok(frame) => return Ok(frame),
                Err(stratum_core::codec_sv2::Error::MissingBytes(_)) => {
                    tokio::task::yield_now().await;
                    continue;
                }
                Err(e) => return Err(Error::CodecError(e)),
            }
        }
    }
}

impl<Message> NoiseTcpWriteHalf<Message>
where
    Message: Serialize + BinaryDeserialize<'static> + GetSize + Send + 'static,
{
    pub async fn write_frame(&mut self, frame: StandardEitherFrame<Message>) -> Result<(), Error> {
        let buf = self.encoder.encode(frame, &mut self.state)?;
        self.writer
            .write_all(buf.as_ref())
            .await
            .map_err(|_| Error::SocketClosed)?;
        Ok(())
    }
}

async fn send_message<Message: Serialize + BinaryDeserialize<'static> + GetSize + Send + 'static>(
    writer: &mut OwnedWriteHalf,
    msg: StandardEitherFrame<Message>,
    state: &mut State,
    encoder: &mut NoiseEncoder<Message>,
) -> Result<(), Error> {
    let buffer = encoder.encode(msg, state)?;
    writer
        .write_all(buffer.as_ref())
        .await
        .map_err(|_| Error::SocketClosed)?;
    Ok(())
}

async fn receive_message<Message: Serialize + BinaryDeserialize<'static> + GetSize + Send + 'static>(
    reader: &mut OwnedReadHalf,
    state: &mut State,
    decoder: &mut StandardNoiseDecoder<Message>,
) -> Result<StandardEitherFrame<Message>, Error> {
    let mut buffer = vec![0u8; decoder.writable_len()];
    reader
        .read_exact(&mut buffer)
        .await
        .map_err(|_| Error::SocketClosed)?;
    decoder.writable().copy_from_slice(&buffer);
    decoder.next_frame(state).map_err(Error::CodecError)
}

/// Basic SV1 message types for parsing
#[derive(Debug)]
struct Sv1Message {
    msg_type: String,
    id: Option<u32>,
    result: Option<Value>,
    error: Option<Value>,
}

/// Parse basic SV1 JSON response
fn parse_sv1_response(line: &str) -> Result<Sv1Message, Box<dyn std::error::Error>> {
    let value: Value = serde_json::from_str(line)?;

    let msg_type = if value.get("method").is_some() {
        "notification".to_string()
    } else if value.get("result").is_some() {
        "response".to_string()
    } else {
        "unknown".to_string()
    };

    let id = value.get("id").and_then(|v| v.as_u64()).map(|v| v as u32);

    Ok(Sv1Message {
        msg_type,
        id,
        result: value.get("result").cloned(),
        error: value.get("error").cloned(),
    })
}



#[derive(Debug, Deserialize)]
struct Target {
    address: String,
    pubkey: Option<String>,
    #[serde(default = "default_protocol")]
    protocol: String,
}

fn default_protocol() -> String {
    "sv2".to_string()
}

pub async fn start(config: Config) -> Result<()> {
    let listener = TcpListener::bind(config.listen_address).await?;
    info!("Proxy listening for miners on: {}", config.listen_address);

    // Verify connection to server on startup
    info!("Verifying connection to server...");
    let initial_target = fetch_target(&config.server_endpoint).await
        .context("Failed to connect to server on startup")?;
    info!("Successfully connected to server. Current target: {} (Protocol: {})", initial_target.address, initial_target.protocol);

    let config = Arc::new(config);

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("Accepted connection from {}", addr);
        
        let config = config.clone();

        tokio::spawn(async move {
            // Try to detect protocol by reading first few bytes
            if let Err(e) = handle_connection_auto(socket, config).await {
                error!("Connection error with {}: {:?}", addr, e);
            }
        });
    }
}

async fn fetch_target(server_url: &str) -> Result<Target> {
    let url = format!("{}/target", server_url);
    let target = reqwest::get(&url)
        .await?
        .json::<Target>()
        .await?;
    Ok(target)
}

async fn handle_connection_auto(
    downstream_socket: TcpStream,
    config: Arc<Config>,
) -> Result<()> {
    // Peek at first byte to detect protocol
    // V1 (JSON-RPC): starts with '{' (0x7B)
    // V2 (Noise): starts with handshake (binary)
    
    let mut buf = [0u8; 1];
    downstream_socket.peek(&mut buf).await?;
    
    if buf[0] == b'{' {
        info!("Detected Stratum V1 downstream connection");
        handle_v1_passthrough(downstream_socket, config).await
    } else {
        info!("Detected Stratum V2 downstream connection");
        // Generate keypair for SV2 (outside of async context to avoid Send issues)
        let (proxy_pubkey_bytes, proxy_secret_key_bytes) = tokio::task::block_in_place(|| {
            let mut rng = rand::thread_rng();
            let proxy_secret_key = secp256k1::SecretKey::new(&mut rng);
            let proxy_public_key = secp256k1::PublicKey::from_secret_key(&secp256k1::Secp256k1::new(), &proxy_secret_key);
            
            let proxy_secret_key_bytes = proxy_secret_key.secret_bytes();
            let (x_only_pubkey, _) = proxy_public_key.x_only_public_key();
            let proxy_pubkey_bytes = x_only_pubkey.serialize();
            (proxy_pubkey_bytes, proxy_secret_key_bytes)
        });
        
        handle_sv2_connection(downstream_socket, config, proxy_pubkey_bytes, proxy_secret_key_bytes).await
    }
}

async fn handle_v1_passthrough(
    downstream_socket: TcpStream,
    config: Arc<Config>,
) -> Result<()> {
    // Fetch target from server
    info!("Fetching target from server: {}", config.server_endpoint);
    let target = fetch_target(&config.server_endpoint).await
        .context("Failed to fetch target from server")?;
    info!("Got target: {} (Protocol: {})", target.address, target.protocol);

    if target.protocol != "sv1" {
        return Err(anyhow::anyhow!("V1 miner requires V1 upstream, but got {}", target.protocol));
    }

    // Connect to upstream
    info!("Connecting to upstream (SV1): {}", target.address);
    let upstream_socket = TcpStream::connect(&target.address).await
        .context("Failed to connect to upstream")?;
    info!("Connected to upstream: {}", target.address);

    // Simple bidirectional passthrough
    let (mut d_read, mut d_write) = downstream_socket.into_split();
    let (mut u_read, mut u_write) = upstream_socket.into_split();

    let downstream_to_upstream = async {
        let mut buf = vec![0u8; 8192];
        loop {
            let n = d_read.read(&mut buf).await?;
            if n == 0 {
                return Ok::<(), anyhow::Error>(());
            }
            info!("V1 Downstream → Upstream: {} bytes", n);
            u_write.write_all(&buf[..n]).await?;
        }
    };

    let upstream_to_downstream = async {
        let mut buf = vec![0u8; 8192];
        loop {
            let n = u_read.read(&mut buf).await?;
            if n == 0 {
                return Ok::<(), anyhow::Error>(());
            }
            info!("V1 Upstream → Downstream: {} bytes", n);
            d_write.write_all(&buf[..n]).await?;
        }
    };

    tokio::select! {
        res = downstream_to_upstream => res,
        res = upstream_to_downstream => res,
    }
}

async fn handle_sv2_connection(
    downstream_socket: TcpStream, 
    config: Arc<Config>,
    proxy_pubkey_bytes: [u8; 32],
    proxy_secret_key_bytes: [u8; 32]
) -> Result<()> {
    // 1. Downstream Handshake (Responder)
    let responder = Responder::from_authority_kp(
        &proxy_pubkey_bytes,
        &proxy_secret_key_bytes,
        std::time::Duration::from_secs(3600), // Cert validity
    ).map_err(|e| anyhow::anyhow!("Failed to create responder: {:?}", e))?;

    let downstream_stream = NoiseTcpStream::<Message>::new(
        downstream_socket,
        HandshakeRole::Responder(responder),
    ).await.map_err(|e| anyhow::anyhow!("Downstream handshake failed: {:?}", e))?;

    info!("Downstream handshake complete");

    // 2. Fetch Target from Server
    info!("Fetching target from server: {}", config.server_endpoint);
    let target = fetch_target(&config.server_endpoint).await
        .context("Failed to fetch target from server")?;
    info!("Got target: {} (Protocol: {})", target.address, target.protocol);

    if target.protocol == "sv1" {
        handle_sv1_upstream(downstream_stream, target, &config).await
    } else {
        handle_sv2_upstream(downstream_stream, target).await
    }
}

async fn handle_sv2_upstream(
    downstream_stream: NoiseTcpStream<Message>,
    target: Target,
) -> Result<()> {
    // 3. Upstream Connection & Handshake (Initiator)
    info!("Connecting to upstream (SV2): {}", target.address);
    let upstream_socket = TcpStream::connect(&target.address).await
        .context("Failed to connect to upstream")?;
    
    let upstream_pubkey_str = target.pubkey.ok_or_else(|| anyhow::anyhow!("Missing pubkey for SV2 target"))?;
    let upstream_pubkey: Secp256k1PublicKey = upstream_pubkey_str.parse()
        .context("Invalid upstream pubkey")?;
        
    let initiator = Initiator::from_raw_k(upstream_pubkey.0.serialize())
        .map_err(|e| anyhow::anyhow!("Failed to create initiator: {:?}", e))?;
    
    let upstream_stream = NoiseTcpStream::<Message>::new(
        upstream_socket,
        HandshakeRole::Initiator(initiator),
    ).await.map_err(|e| anyhow::anyhow!("Upstream handshake failed: {:?}", e))?;

    info!("Connected to upstream: {}", target.address);
    info!("Upstream handshake complete");

    // 4. Bridge
    let (mut d_read, mut d_write) = downstream_stream.into_split();
    let (mut u_read, mut u_write) = upstream_stream.into_split();

    let client_to_server = async {
        loop {
            match d_read.read_frame().await {
                Ok(frame) => {
                    if let Err(e) = u_write.write_frame(frame).await {
                        return Err(anyhow::anyhow!("Failed to write to upstream: {:?}", e));
                    }
                }
                Err(e) => return Err(anyhow::anyhow!("Failed to read from downstream: {:?}", e)),
            }
        }
    };

    let server_to_client = async {
        loop {
            match u_read.read_frame().await {
                Ok(frame) => {
                    if let Err(e) = d_write.write_frame(frame).await {
                        return Err(anyhow::anyhow!("Failed to write to downstream: {:?}", e));
                    }
                }
                Err(e) => return Err(anyhow::anyhow!("Failed to read from upstream: {:?}", e)),
            }
        }
    };

    tokio::select! {
        res = client_to_server => res,
        res = server_to_client => res,
    }
}

// Simple SV1 JSON-RPC handling
use tokio_util::codec::{Framed, LinesCodec};
use futures::SinkExt;
use futures::StreamExt;
use serde_json::{json, Value};

async fn handle_sv1_upstream(
    downstream_stream: NoiseTcpStream<Message>,
    target: Target,
    config: &Config,
) -> Result<()> {
    info!("Connecting to upstream (SV1): {}", target.address);
    let upstream_socket = TcpStream::connect(&target.address).await
        .context("Failed to connect to upstream")?;
    
    let mut upstream_framed = Framed::new(upstream_socket, LinesCodec::new());

    // Basic SV1 Login (configurable wallet for development/testing)
    // TODO: In production, implement proper SV2->SV1 translation that extracts
    // wallet address from SV2 SetupConnection/OpenStandardMiningChannel messages
    let wallet = config.default_wallet.as_deref()
        .unwrap_or("44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBDDws8keQf66JxvVXuquhE3mAyUAL4f8cpAGzBVCTLG0P5sqDK17I3wcBiRT");

    let login_req = json!({
        "id": 1,
        "method": "login",
        "params": {
            "login": wallet,
            "pass": "x",
            "agent": "defpool-proxy/0.1"
        }
    });
    upstream_framed.send(login_req.to_string()).await?;

    let (mut d_read, _d_write) = downstream_stream.into_split();

    // Loop to bridge messages (Simplified: Just logging for now to prove connection)
    loop {
        tokio::select! {
            // Read from Upstream (SV1)
            Some(line_res) = upstream_framed.next() => {
                match line_res {
                    Ok(line) => {
                        info!("Received from upstream (SV1): {}", line);

                        // Basic SV1 response handling
                        // TODO: Implement full SV1->SV2 translation
                        match parse_sv1_response(&line) {
                            Ok(sv1_msg) => {
                                info!("Parsed SV1 message: {:?}", sv1_msg.msg_type);
                                // For now, just acknowledge receipt
                                // In full implementation: translate to SV2 and send via d_write
                            }
                            Err(e) => {
                                warn!("Failed to parse SV1 response: {}", e);
                            }
                        }
                    }
                    Err(e) => return Err(anyhow::anyhow!("Upstream read error: {:?}", e)),
                }
            }
            // Read from Downstream (SV2)
            res = d_read.read_frame() => {
                match res {
                    Ok(frame) => {
                        match frame {
                            Frame::Sv2(mut sv2_frame) => {
                                info!("Received from downstream (SV2). Payload size: {}", sv2_frame.payload().len());
                                // TODO: Parse SV2 message and translate to SV1 JSON commands
                                // For now, just acknowledge receipt
                            }
                            _ => {
                                info!("Received unexpected handshake frame from downstream");
                            }
                        }
                        // TODO: Translate SV2 message -> SV1 JSON -> upstream_framed
                    }
                    Err(e) => return Err(anyhow::anyhow!("Downstream read error: {:?}", e)),
                }
            }
        }
    }
}
