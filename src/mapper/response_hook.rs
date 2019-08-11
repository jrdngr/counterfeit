use std::io;
use std::path::{Path, PathBuf};

use hyper::{Body, Request, Response, StatusCode};

pub trait ResponseHook {
    fn process_response(&mut self, response: Response<Body>) -> io::Result<Response<Body>>;
}