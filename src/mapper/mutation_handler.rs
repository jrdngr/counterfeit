use std::io;

use hyper::{Body, Request};

pub trait MutationHandler {
    fn apply_mutation(&mut self, request: &Request<Body>) -> io::Result<()>;
}

pub struct ImmutableHandler;

impl MutationHandler for ImmutableHandler {
    fn apply_mutation(&mut self, _request: &Request<Body>) -> io::Result<()> {
        Ok(())
    }
}
