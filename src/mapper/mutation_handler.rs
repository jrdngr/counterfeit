use std::io;
use std::path::{Path, PathBuf};

use hyper::{Body, Request, Response, StatusCode};

pub trait MutationHandler {
    fn apply_mutation(&mut self, request: &Request<Body>) -> io::Result<()>;
}