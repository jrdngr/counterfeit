use std::io;

use hyper::{Body, Request};

pub trait ResponseMutation: Send + Sync {
    fn apply_mutation(&mut self, request: &Request<Body>) -> io::Result<()>;
}
