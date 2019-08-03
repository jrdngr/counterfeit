use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

pub mod mapper;

const BASE_PATH: &str = "./responses";

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = || {
        service_fn_ok(map_request)
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("Server error: {}", e));

    hyper::rt::run(server);
}

fn map_request(request: Request<Body>) -> Response<Body> {
    match mapper::get_body(BASE_PATH, request) {
        Ok(response) => response,
        Err(e) => {
            let mut response = Response::new(Body::from(format!("{}", e)));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
    }
}
