use std::net::SocketAddr;

use crate::options::CounterfeitRunOptions;

#[derive(Debug, Clone)]
pub struct CounterfeitRunConfig {
    pub base_path: String,
    pub lenient: bool,
    pub write: bool,
    pub create_missing: bool,
    pub silent: bool,
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
            create_missing,
            silent,
            mut socket,
            port,
            param_prefix,
            param_postfix,
            param_surround,
        } = options;

        if let Some(port) = port {
            socket.set_port(port);
        }

        let (prefix, postfix) = if let Some(surround) = param_surround {
            (surround.clone(), surround.clone())
        } else {
            (param_prefix, param_postfix)
        };

        Self {
            base_path,
            lenient,
            write,
            create_missing,
            silent,
            socket,
            prefix,
            postfix,
        }
    }
}
