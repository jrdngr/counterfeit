use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use hyper::header::{self, HeaderValue};
use hyper::rt::Future;
use hyper::service::{make_service_fn, service_fn_ok};
use hyper::{Body, Method, Response, Server};
use structopt::StructOpt;

use crate::options::CounterfeitOptions;

pub mod options;
pub mod mapper;

pub use crate::mapper::FileMapper;

pub const BASE_PATH: &str = "./responses";

pub type MultiFileIndexMap = Arc<Mutex<HashMap<PathBuf, usize>>>;

fn main() -> io::Result<()> {
    let _options = CounterfeitOptions::from_args();

    let addr = ([127, 0, 0, 1], 3000).into();

    let index_map: MultiFileIndexMap = Arc::new(Mutex::new(HashMap::new()));

    let make_service = make_service_fn(move |_| {
        let mapper = FileMapper::new(BASE_PATH, Arc::clone(&index_map));
        service_fn_ok(move |request| {
            let mut response = if request.method() == Method::OPTIONS {
                Response::new(Body::empty())
            } else {
                mapper.map_request(request)
            };
            response.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_static("*"),
            );
            response.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_static("*"),
            );
            response.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderValue::from_static("*"),
            );
            response
        })
    });

    let server = Server::bind(&addr)
        .serve(make_service)
        .map_err(|e| eprintln!("Server error: {}", e));

    println!("Serving files at {:?}", &addr);

    hyper::rt::run(server);

    Ok(())
}
