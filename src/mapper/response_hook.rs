use std::io;

use hyper::{Body, Response};

pub trait ResponseHook {
    fn process_response(&mut self, response: Response<Body>) -> io::Result<Response<Body>>;
}

pub struct IdentityResponseHook;

impl ResponseHook for IdentityResponseHook {
    fn process_response(&mut self, response: Response<Body>) -> io::Result<Response<Body>> {
        Ok(response)
    }
}
