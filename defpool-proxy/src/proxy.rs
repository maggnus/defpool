use crate::config::Config;
use crate::share_recorder::{ShareRecorder, ShareSubmission};
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, error, info, warn};
use serde::Deserialize;

// Stratum V2 crates
use stratum_core::{
    codec_sv2::{HandshakeRole, StandardEitherFrame},
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HandshakeRemoteInvalidMessage => write!(f, "Handshake remote invalid message"),
            Error::CodecError(e) => write!(f, "Codec error: {:?}", e),
            Error::SocketClosed => write!(f, "Socket closed"),
        }
    }
}

impl std::error::Error for Error {}

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

// TODO: Define SV1 message structures
// #[derive(Debug)] struct Sv1Message { ... }

// TODO: Implement SV1 message parsing
// fn parse_sv1_response(line: &str) -> Result<Sv1Message, Box<dyn std::error::Error>> { ... }

// TODO: Implement SV2 message creation functions
// fn create_sv2_setup_connection() -> Result<...> { ... }
// fn create_sv2_open_channel(user_identity: &str) -> Result<...> { ... }

// TODO: Implement message translation functions
// async fn translate_sv2_to_sv1(...) -> Result<()> { ... }
// async fn translate_sv1_to_sv2(...) -> Result<()> { ... }

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
    use tokio::io::BufReader;
    use tokio::io::AsyncBufReadExt;
    use crate::stratum::Sv1Message;
    
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

    // Create share recorder
    let share_recorder = Arc::new(ShareRecorder::new(config.server_endpoint.clone()));
    let target_name = share_recorder.get_current_target().await
        .unwrap_or_else(|_| "unknown".to_string());

    // Split and wrap in buffered readers for line-based protocol
    let (d_read, mut d_write) = downstream_socket.into_split();
    let (u_read, mut u_write) = upstream_socket.into_split();
    
    let mut d_reader = BufReader::new(d_read);
    let mut u_reader = BufReader::new(u_read);

    let mut wallet_address = config.default_wallet.clone();
    let mut worker_name = String::from("worker1");
    let share_recorder_clone = share_recorder.clone();
    let target_name_clone = target_name.clone();

    let downstream_to_upstream = async move {
        let mut line = String::new();
        loop {
            line.clear();
            let n = d_reader.read_line(&mut line).await?;
            if n == 0 {
                return Ok::<(), anyhow::Error>(());
            }
            
            // Parse and log SV1 message
            if let Ok(msg) = Sv1Message::from_json(&line) {
                if let Some(method) = &msg.method {
                    info!("V1 Miner → Pool: {}", method);
                    
                    // Extract wallet/worker from login
                    if method == "login" {
                        if let Some(params) = &msg.params {
                            if let Some(serde_json::Value::String(login)) = params.first() {
                                let parts: Vec<&str> = login.split(':').collect();
                                if !parts.is_empty() {
                                    wallet_address = Some(parts[0].to_string());
                                    worker_name = parts.get(1).unwrap_or(&"worker1").to_string();
                                    info!("Extracted wallet: {}, worker: {}", parts[0], worker_name);
                                }
                            }
                        }
                    }
                    
                    // Record share submissions
                    if method == "submit" {
                        let wallet = wallet_address.as_deref().unwrap_or("unknown");
                        info!("Share submitted by {}/{}", wallet, worker_name);
                        
                        // Record share asynchronously (don't block on result)
                        let recorder = share_recorder_clone.clone();
                        let submission = ShareSubmission {
                            wallet_address: wallet.to_string(),
                            worker_name: worker_name.clone(),
                            target_name: target_name_clone.clone(),
                            difficulty: 1000.0, // TODO: Extract from job
                            valid: true, // Will be validated by pool response
                        };
                        
                        tokio::spawn(async move {
                            if let Err(e) = recorder.record_share(submission).await {
                                warn!("Failed to record share: {}", e);
                            }
                        });
                    }
                }
            }
            
            u_write.write_all(line.as_bytes()).await?;
        }
    };

    let upstream_to_downstream = async {
        let mut line = String::new();
        loop {
            line.clear();
            let n = u_reader.read_line(&mut line).await?;
            if n == 0 {
                return Ok::<(), anyhow::Error>(());
            }
            
            // Parse and log SV1 response
            if let Ok(msg) = Sv1Message::from_json(&line) {
                if let Some(method) = &msg.method {
                    info!("V1 Pool → Miner: {}", method);
                } else if msg.is_response() {
                    // Check for share acceptance/rejection
                    if let Some(error) = &msg.error {
                        warn!("Share rejected: {:?}", error);
                    }
                }
            }
            
            d_write.write_all(line.as_bytes()).await?;
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
// use tokio_util::codec::{Framed, LinesCodec}; // TODO: For SV1 support
// use futures::{SinkExt, StreamExt}; // TODO: For async utilities
// use serde_json::{json, Value}; // TODO: For SV1 message parsing

async fn handle_sv1_upstream(
    downstream_stream: NoiseTcpStream<Message>,
    target: Target,
    _config: &Config,
) -> Result<()> {
    info!("SV2 miner connecting to SV1 upstream pool: {}", target.address);
    info!("Protocol translation: SV2 (downstream) → SV1 (upstream)");

    // Connect to SV1 pool
    let upstream_socket = TcpStream::connect(&target.address).await
        .context("Failed to connect to upstream SV1 pool")?;

    info!("Connected to upstream SV1 pool: {}", target.address);

    // Split streams
    let (mut downstream_read, mut downstream_write) = downstream_stream.into_split();
    let (mut upstream_read, mut upstream_write) = upstream_socket.into_split();

    info!("SV2↔SV1 translation bridge established");

    // Bidirectional forwarding with protocol translation
    // Note: Full translation requires parsing SV2 messages and converting to SV1 JSON-RPC
    // This is a complex task that requires understanding both protocol specifications
    let mut buffer = vec![0u8; 8192];
    
    loop {
        tokio::select! {
            // Read from SV2 downstream miner
            downstream_msg = downstream_read.read_frame() => {
                match downstream_msg {
                    Ok(_frame) => {
                        info!("SV2 frame from miner (translation TODO)");
                        // TODO: Parse SV2 frame, translate to SV1 JSON, send to upstream
                        // This requires:
                        // 1. Extract message from frame
                        // 2. Match on message type (SetupConnection, OpenChannel, SubmitShares, etc.)
                        // 3. Translate to equivalent SV1 JSON-RPC message
                        // 4. Send to upstream pool
                    }
                    Err(e) => {
                        warn!("Error reading from SV2 downstream: {:?}", e);
                        break;
                    }
                }
            }

            // Read from SV1 upstream pool
            upstream_result = upstream_read.read(&mut buffer) => {
                match upstream_result {
                    Ok(n) if n > 0 => {
                        info!("SV1 data from pool: {} bytes (translation TODO)", n);
                        // TODO: Parse SV1 JSON, translate to SV2 frame, send to downstream
                    }
                    Ok(_) => {
                        info!("Upstream connection closed");
                        break;
                    }
                    Err(e) => {
                        warn!("Error reading from SV1 upstream: {:?}", e);
                        break;
                    }
                }
            }
        }
    }

    info!("SV2↔SV1 bridge closed");
    Ok(())
}
