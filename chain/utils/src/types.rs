use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PeerId([u8; 32]);

#[derive(Debug, Clone)]
pub struct PeerInfo
{
    pub addr: SocketAddr,
    pub id: PeerId,
}
