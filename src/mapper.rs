use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hyper::header::{self, HeaderValue};
use hyper::{Body, Method, Request, Response, StatusCode};

use crate::MultiFileIndexMap;

pub struct FileMapper {
    base_path: String,
    multifile_indices: MultiFileIndexMap,
}

impl FileMapper {
    pub fn new(base_path: &str, index_map: MultiFileIndexMap) -> Self {
        Self {
            base_path: base_path.to_string(),
            multifile_indices: index_map,
        }
    }

    pub fn get_body(&self, req: Request<Body>) -> io::Result<Response<Body>> {
        let mut response = Response::new(Body::empty());

        if let Some(path) = req.uri().path().split('?').nth(0) {
            let full_path = PathBuf::from(format!("{}{}", &self.base_path, path));
            let file_path = self.choose_file(&full_path, req.method())?;

            let body_text = fs::read_to_string(&file_path)?;
            *response.body_mut() = Body::from(body_text);

            if let Some(ext) = file_path.extension() {
                if ext == "json" {
                    response.headers_mut().insert(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json"),
                    );
                }
            }

            return Ok(response);
        }

        *response.status_mut() = StatusCode::NOT_FOUND;

        Ok(response)
    }

    pub fn map_request(&self, request: Request<Body>) -> Response<Body> {
        match self.get_body(request) {
            Ok(response) => response,
            Err(e) => {
                use std::io::ErrorKind;

                let mut response = Response::new(Body::from(format!("{}", &e)));

                *response.status_mut() = match e.kind() {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                response
            }
        }
    }

    pub fn choose_file(&self, path: &Path, method: &Method) -> io::Result<PathBuf> {
        let available_files = fs::read_dir(path)?
            .filter_map(Result::ok)
            .map(|file| file.path())
            .filter(|path| path.is_file())
            .filter(|path| file_matches(path, method))
            .collect::<Vec<PathBuf>>();

        match available_files.into_iter().nth(0) {
            Some(file) => Ok(file),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No files available",
            )),
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
