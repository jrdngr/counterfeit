use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hyper::{Body, Method, Request};

use crate::MultiFileIndexMap;
use crate::mapper::MapperResult;

pub trait FilePicker {
    fn pick_file(&self, directory: &Path, request: &Request<Body>) -> MapperResult;
}

pub struct StandardFilePicker {
    multifile_indices: MultiFileIndexMap,
}

impl StandardFilePicker {
    pub fn new(index_map: MultiFileIndexMap) -> Self {
        Self {
            multifile_indices: index_map,
        }
    }
}

impl FilePicker for StandardFilePicker {
    fn pick_file(&self, directory: &Path, request: &Request<Body>) -> MapperResult {
        let available_files = fs::read_dir(&directory)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .filter(|p| file_matches(p, request.method()))
            .collect::<Vec<PathBuf>>();

        if available_files.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No files available",
            ))
        } else {
            let mut indices = self.multifile_indices.lock().unwrap();
            let index = indices.entry(PathBuf::from(directory)).or_insert_with(|| 0);
            if *index >= available_files.len() {
                *index = 0;
            }

            match available_files.into_iter().nth(*index) {
                Some(file) => {
                    *index += 1;
                    Ok(file)
                }
                None => Err(io::Error::new(io::ErrorKind::Other, "Could not read file")),
            }
        }
    }
}

fn file_matches(file_path: &PathBuf, method: &Method) -> bool {
    let method_str = method.as_str().to_lowercase();

    match file_path.file_stem().and_then(|stem| stem.to_str()) {
        Some(stem) => {
            stem == method_str || stem.to_lowercase().starts_with(&format!("{}_", method_str))
        }
        None => false,
    }
}
