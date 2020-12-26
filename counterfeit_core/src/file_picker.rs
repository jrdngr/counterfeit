use std::path::{Path, PathBuf};

use crate::Error;

pub trait FilePicker {
    type Request;

    fn pick_file(&self, directory: &Path, request: &Self::Request) -> Result<PathBuf, Error>;
}
