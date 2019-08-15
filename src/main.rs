use std::collections::HashMap;
use std::io;
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use hyper::header::{self, HeaderValue};
use hyper::rt::Future;
use hyper::service::{make_service_fn, service_fn_ok};
use hyper::{Body, Method, Response, Server};
use structopt::StructOpt;

pub mod config;
pub mod mapper;
pub mod old_mapper;
pub mod options;

pub use crate::config::CounterfeitRunConfig;
pub use crate::mapper::{FileMapper, RequestMapper};

use crate::options::CounterfeitOptions;

pub type MultiFileIndexMap = Arc<Mutex<HashMap<PathBuf, usize>>>;

fn main() -> io::Result<()> {
    match CounterfeitOptions::from_args() {
        CounterfeitOptions::Run(run_options) => run(run_options.into()),
        CounterfeitOptions::Save(_save_options) => unimplemented!(),
    }
}

fn run(config: CounterfeitRunConfig) -> io::Result<()> {
    let index_map: MultiFileIndexMap = Arc::new(Mutex::new(HashMap::new()));

    let socket = config.socket;

    let make_service = make_service_fn(move |_| {
        let mut mapper = FileMapper::standard(config.clone(), Arc::clone(&index_map));
        service_fn_ok(move |request| {
            let mut response = if request.method() == Method::OPTIONS {
                Response::new(Body::empty())
            } else {
                match mapper.map_request(request) {
                    Ok(response) => response,
                    Err(e) => {
                        let mut response = Response::new(Body::from(e.description().to_string()));
                        *response.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
                        response
                    },
                }
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

    let server = Server::bind(&socket)
        .serve(make_service)
        .map_err(|e| eprintln!("Server error: {}", e));

    println!("Serving files at {:?}", &socket);

    hyper::rt::run(server);

    Ok(())
}
