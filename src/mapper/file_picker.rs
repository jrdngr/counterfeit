use std::io;
use std::path::{Path, PathBuf};

use hyper::{Body, Request, Response, StatusCode};

pub trait FilePicker {
    fn pick_file(&mut self, directory: &Path, request: &Request<Body>) -> io::Result<PathBuf>;
}