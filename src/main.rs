use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

use hyper::Server;
use hyper::rt::Future;
use hyper::service::{make_service_fn, service_fn_ok};

pub mod mapper;

pub use crate::mapper::FileMapper;

pub const BASE_PATH: &str = "./responses";

pub type MultiFileIndexMap = Arc<Mutex<HashMap<String, usize>>>;

fn main() -> io::Result<()> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let index_map: MultiFileIndexMap = Arc::new(Mutex::new(HashMap::new()));

    let make_service = make_service_fn(move |_| { 
        let mapper = FileMapper::new(BASE_PATH, Arc::clone(&index_map));
        service_fn_ok(move |request| mapper.map_request(request))
    });

    let server = Server::bind(&addr)
        .serve(make_service)
        .map_err(|e| eprintln!("Server error: {}", e));

    hyper::rt::run(server);

    Ok(())
}
