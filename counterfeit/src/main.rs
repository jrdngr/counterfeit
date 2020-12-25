use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use hyper::Server;
use structopt::StructOpt;

pub mod config;
pub mod mapper;
pub mod options;

pub use crate::config::CounterfeitRunConfig;
pub use crate::mapper::MakeFileMapperService;

use crate::options::CounterfeitOptions;

pub type MultiFileIndexMap = Arc<Mutex<HashMap<PathBuf, usize>>>;

#[tokio::main]
async fn main() -> Result<()> {
    match CounterfeitOptions::from_args() {
        CounterfeitOptions::Run(run_options) => run(run_options.into()).await,
        CounterfeitOptions::Save(_save_options) => todo!(),
    }
}

async fn run(config: CounterfeitRunConfig) -> Result<()> {
    let index_map: MultiFileIndexMap = Arc::new(Mutex::new(HashMap::new()));

    let socket = config.socket;
   
    let make_service = MakeFileMapperService::new(config, index_map);

    let server = Server::bind(&socket).serve(make_service);
    println!("Serving files at: {}", &socket);

    server.await?;

    Ok(())
}
