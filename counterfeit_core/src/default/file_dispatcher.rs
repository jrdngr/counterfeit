use hyper::{Body, Method, Request};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::{Dispatcher, Error, MultiFileIndexMap};
pub struct DefaultFileDispatcher {
    create_missing: bool,
    multifile_indices: MultiFileIndexMap,
}

impl DefaultFileDispatcher {
    pub fn new(create_missing: bool, index_map: MultiFileIndexMap) -> Self {
        Self {
            create_missing,
            multifile_indices: index_map,
        }
    }
}

impl Dispatcher for DefaultFileDispatcher {
    fn dispatch(
        &self,
        directory: impl AsRef<Path>,
        request: &Request<Body>,
    ) -> Result<PathBuf, Error> {
        let available_files = fs::read_dir(&directory)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .filter(|p| file_matches(p, &request.method()))
            .collect::<Vec<PathBuf>>();

        if available_files.is_empty() {
            if self.create_missing {
                let file_name = format!("{}.json", request.method().to_string().to_lowercase());

                let mut path = PathBuf::new();
                path.push(directory);
                path.push(file_name);

                fs::File::create(&path)?;

                Ok(path)
            } else {
                Err(io::Error::new(io::ErrorKind::NotFound, "No files available").into())
            }
        } else {
            let mut indices = self.multifile_indices.lock().unwrap();
            let index = indices
                .entry(PathBuf::from(directory.as_ref()))
                .or_insert_with(|| 0);
            if *index >= available_files.len() {
                *index = 0;
            }

            match available_files.into_iter().nth(*index) {
                Some(file) => {
                    *index += 1;
                    Ok(file)
                }
                None => Err(io::Error::new(io::ErrorKind::Other, "Could not read file").into()),
            }
        }
    }
}

fn file_matches(file_path: &PathBuf, method: &Method) -> bool {
    let method_str = method.to_string().to_lowercase();

    match file_path.file_stem().and_then(|stem| stem.to_str()) {
        Some(stem) => {
            stem == method_str || stem.to_lowercase().starts_with(&format!("{}_", method_str))
        }
        None => false,
    }
}
