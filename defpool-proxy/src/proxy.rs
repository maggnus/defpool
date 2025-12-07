use crate::config::Config;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{error, info};
use serde::Deserialize;

use stratum_apps::{
    key_utils::Secp256k1PublicKey,
    network_helpers::noise_stream::NoiseTcpStream,
    stratum_core::{
        codec_sv2::HandshakeRole,
        framing_sv2::framing::Frame,
        noise_sv2::{Initiator, Responder},
    },
    utils::types::Message,
};

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
    mut downstream_socket: TcpStream,
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
        handle_sv1_upstream(downstream_stream, target).await
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
        
    let initiator = Initiator::from_raw_k(upstream_pubkey.into_bytes())
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
use serde_json::json;

async fn handle_sv1_upstream(
    mut downstream_stream: NoiseTcpStream<Message>,
    target: Target,
) -> Result<()> {
    info!("Connecting to upstream (SV1): {}", target.address);
    let upstream_socket = TcpStream::connect(&target.address).await
        .context("Failed to connect to upstream")?;
    
    let mut upstream_framed = Framed::new(upstream_socket, LinesCodec::new());

    // Basic SV1 Login (Hardcoded for now, should come from miner)
    // In a real proxy, we need to wait for SV2 SetupConnection/OpenStandardMiningChannel
    // and translate it. For this MVP, we'll just send a login to SupportXMR.
    let login_req = json!({
        "id": 1,
        "method": "login",
        "params": {
            "login": "44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBDDws8keQf66JxvVXuquhE3mAyUAL4f8cpAGzBVCTLG0P5sqDK17I3wcBiRT",
            "pass": "x",
            "agent": "defpool-proxy/0.1"
        }
    });
    upstream_framed.send(login_req.to_string()).await?;

    let (mut d_read, mut d_write) = downstream_stream.into_split();

    // Loop to bridge messages (Simplified: Just logging for now to prove connection)
    loop {
        tokio::select! {
            // Read from Upstream (SV1)
            Some(line_res) = upstream_framed.next() => {
                match line_res {
                    Ok(line) => {
                        info!("Received from upstream (SV1): {}", line);
                        // TODO: Translate SV1 response -> SV2 message -> d_write
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
