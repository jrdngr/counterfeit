use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn echo(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            // we'll be back
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    response
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = || {
        service_fn_ok(echo)
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}