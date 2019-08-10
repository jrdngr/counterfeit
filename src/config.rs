use std::net::SocketAddr;

use crate::options::CounterfeitRunOptions;

#[derive(Debug, Clone)]
pub struct CounterfeitRunConfig {
    pub base_path: String,
    pub lenient: bool,
    pub write: bool,
    pub socket: SocketAddr,
    pub prefix: String,
    pub postfix: String,
}

impl From<CounterfeitRunOptions> for CounterfeitRunConfig {
    fn from(options: CounterfeitRunOptions) -> Self {
        let CounterfeitRunOptions {
            base_path,
            lenient,
            write,
            mut socket,
            port,
            param_prefix,
            param_postfix,
            param_surround,
        } = options;

        if let Some(port) = port {
            socket.set_port(port);
        }

        let mut prefix = String::new();
        if let Some(surround) = &param_surround {
            prefix.push_str(surround);
        }
        if let Some(pre) = &param_prefix {
            prefix.push_str(pre);
        }

        let mut postfix = String::new();
        if let Some(post) = &param_postfix {
            postfix.push_str(post);
        }
        if let Some(surround) = &param_surround {
            postfix.push_str(surround);
        }

        Self {
            base_path,
            lenient,
            write,
            socket,
            prefix,
            postfix,
        }
    }
}
