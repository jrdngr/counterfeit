use std::path::PathBuf;

use crate::Error;

pub trait DirPicker {
    type Request;

    fn pick_directory(&self, request: &Self::Request) -> Result<PathBuf, Error>;
}
