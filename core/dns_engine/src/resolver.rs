use std::net::SocketAddr;
use std::sync::Arc;

use socket2::{Domain, Protocol, Socket, Type};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

use crate::{EngineConfig, Result, VoidBlockEngine, VoidBlockError};

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub domain: String,
    pub ttl_seconds: u32,
}

pub fn extract_question(packet: &[u8]) -> Result<DnsQuestion> {
    if packet.len() < 12 {
        return Err(VoidBlockError::InvalidQuery("DNS packet too small".to_string()));
    }

    let mut index = 12usize;
    let mut labels = Vec::new();
    loop {
        let length = *packet.get(index).ok_or_else(|| VoidBlockError::InvalidQuery("unterminated question name".to_string()))? as usize;
        index += 1;
        if length == 0 {
            break;
        }
        
        // RFC 1035: DNS labels must be <= 63 octets
        if length > 63 {
            return Err(VoidBlockError::InvalidQuery("DNS label exceeds 63 bytes".to_string()));
        }
        
        let label = packet
            .get(index..index + length)
            .ok_or_else(|| VoidBlockError::InvalidQuery("label exceeds packet length".to_string()))?;
        let text = std::str::from_utf8(label)
            .map_err(|_| VoidBlockError::InvalidQuery("domain label is not UTF-8".to_string()))?;
        labels.push(text.to_string());
        index += length;
    }

    // RFC 1035: FQDN must be <= 253 characters
    let domain = labels.join(".");
    if domain.len() > 253 {
        return Err(VoidBlockError::InvalidQuery("domain name exceeds 253 characters".to_string()));
    }

    Ok(DnsQuestion { domain, ttl_seconds: 60 })
}

pub fn build_nxdomain_response(packet: &[u8]) -> Result<Vec<u8>> {
    if packet.len() < 12 {
        return Err(VoidBlockError::InvalidQuery("DNS packet too small".to_string()));
    }
    let mut response = packet.to_vec();
    response[2] |= 0b1000_0000;
    response[2] |= 0b0000_0010;
    response[3] |= 0b0000_0011;
    Ok(response)
}

pub async fn run(config: EngineConfig) -> Result<()> {
    let engine = Arc::new(VoidBlockEngine::open(&config)?);
    let udp = Arc::new(bind_udp(config.bind_addr)?);
    let tcp = bind_tcp(config.bind_addr)?;

    info!(address = %config.bind_addr, "VoidBlock DNS resolver started");

    let udp_task = spawn_udp_loop(udp, engine.clone());
    let tcp_task = spawn_tcp_loop(tcp, engine.clone());

    tokio::select! {
        result = udp_task => {
            if let Err(error) = result {
                error!(%error, "UDP loop failed");
            }
        }
        result = tcp_task => {
            if let Err(error) = result {
                error!(%error, "TCP loop failed");
            }
        }
    }

    Ok(())
}

fn bind_udp(addr: SocketAddr) -> Result<UdpSocket> {
    let socket = Socket::new(Domain::for_address(addr), Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_reuse_address(true)?;
    socket.bind(&addr.into())?;
    socket.set_nonblocking(true)?;
    Ok(UdpSocket::from_std(socket.into())?)
}

fn bind_tcp(addr: SocketAddr) -> Result<TcpListener> {
    let socket = Socket::new(Domain::for_address(addr), Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    socket.bind(&addr.into())?;
    socket.listen(128)?;
    socket.set_nonblocking(true)?;
    Ok(TcpListener::from_std(socket.into())?)
}

fn spawn_udp_loop(socket: Arc<UdpSocket>, engine: Arc<VoidBlockEngine>) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        let mut buffer = vec![0u8; 2048];
        loop {
            let (size, peer) = socket.recv_from(&mut buffer).await?;
            let request = buffer[..size].to_vec();
            let socket = Arc::clone(&socket);
            let engine = Arc::clone(&engine);
            let _ = tokio::spawn(async move {
                match engine.handle_query(&request).await {
                    Ok(response) => {
                        if let Err(error) = socket.send_to(&response, peer).await {
                            error!(%error, %peer, "failed to send DNS response");
                        }
                    }
                    Err(error) => {
                        warn!(%error, %peer, "failed to handle DNS query");
                    }
                }
            });
        }
    })
}

fn spawn_tcp_loop(listener: TcpListener, engine: Arc<VoidBlockEngine>) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        loop {
            let (stream, peer) = listener.accept().await?;
            debug!(%peer, "accepted DNS-over-TCP connection");
            let engine = engine.clone();
            let _ = tokio::spawn(async move {
                if let Err(error) = handle_tcp_connection(stream, engine).await {
                    warn!(%error, %peer, "TCP connection failed");
                }
            });
        }
    })
}

async fn handle_tcp_connection(mut stream: TcpStream, engine: Arc<VoidBlockEngine>) -> Result<()> {
    let mut length = [0u8; 2];
    stream.read_exact(&mut length).await?;
    let size = u16::from_be_bytes(length) as usize;
    let mut packet = vec![0u8; size];
    stream.read_exact(&mut packet).await?;
    let response = engine.handle_query(&packet).await?;
    let response_length = u16::try_from(response.len()).map_err(|_| VoidBlockError::Resolver("DNS response too large".to_string()))?;
    stream.write_all(&response_length.to_be_bytes()).await?;
    stream.write_all(&response).await?;
    stream.shutdown().await?;
    Ok(())
}
