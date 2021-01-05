pub mod config;
pub mod dispatcher;
pub mod error;
pub mod rest;

pub use config::CounterfeitConfig;
pub use dispatcher::Dispatcher;
pub use error::Error;
pub use rest::{RestDirDispatcher, RestFileDispatcher};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub type MultiFileIndexMap = Arc<Mutex<HashMap<PathBuf, usize>>>;
