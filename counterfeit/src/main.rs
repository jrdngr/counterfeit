pub mod options;
pub mod services;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use counterfeit_core::MultiFileIndexMap;
use hyper::Server;
use services::MakeFileMapperService;
use structopt::StructOpt;

use crate::options::{CounterfeitOptions, CounterfeitServeOptions};

#[tokio::main]
async fn main() -> Result<()> {
    match CounterfeitOptions::from_args() {
        CounterfeitOptions::Serve(serve_options) => run(serve_options).await,
        CounterfeitOptions::Save(_save_options) => todo!(),
    }
}

async fn run(options: CounterfeitServeOptions) -> Result<()> {
    let index_map: MultiFileIndexMap = Arc::new(Mutex::new(HashMap::new()));

    let socket = format!("{}:{}", options.host, options.port)
        .parse()
        .expect("Invalid socket address");

    let make_service = MakeFileMapperService::new(options.into(), index_map);

    let server = Server::bind(&socket).serve(make_service);
    println!("Serving files at: {}", &socket);

    server.await?;

    Ok(())
}
