use hyper::{Body, Request};
use std::path::{Path, PathBuf};

use crate::Error;

pub trait Dispatcher {
    fn dispatch(
        &self,
        base_directory: impl AsRef<Path>,
        request: &Request<Body>,
    ) -> Result<PathBuf, Error>;
}
