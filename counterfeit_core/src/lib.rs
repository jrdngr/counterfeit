pub mod config;
pub mod mapper;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub use crate::config::CounterfeitRunConfig;
pub use crate::mapper::MakeFileMapperService;

pub type MultiFileIndexMap = Arc<Mutex<HashMap<PathBuf, usize>>>;
