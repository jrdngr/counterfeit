use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use counterfeit_core::{CounterfeitRunConfig, MakeFileMapperService, MultiFileIndexMap};
use hyper::Server;
use structopt::StructOpt;

pub mod options;

use crate::options::CounterfeitOptions;

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
