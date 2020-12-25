use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct CounterfeitRunConfig {
    pub base_path: String,
    pub write: bool,
    pub create_missing: bool,
    pub silent: bool,
    pub socket: SocketAddr,
    pub prefix: String,
    pub postfix: String,
}

