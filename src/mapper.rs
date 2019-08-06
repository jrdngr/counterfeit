use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hyper::header::{self, HeaderValue};
use hyper::{Body, Method, Request, Response, StatusCode};
use walkdir::WalkDir;

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

    pub fn choose_file(&self, path: &Path, method: &Method) -> io::Result<PathBuf> {
        let path = self.process_path(path);

        let available_files = fs::read_dir(&path)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .filter(|p| file_matches(p, method))
            .collect::<Vec<PathBuf>>();

        if available_files.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No files available",
            ))
        } else {
            let mut indices = self.multifile_indices.lock().unwrap();
            let index = indices.entry(path).or_insert_with(|| 0);
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

    fn process_path(&self, path: &Path) -> PathBuf {
        // if path.exists() {
        //     return PathBuf::from(path);
        // }

        let path_len = path.components().count();
        dbg!(&path);
        dbg!(path_len);

        let all_paths: Vec<PathBuf> = list_dirs_recursive(&self.base_path)
            .into_iter()
            .filter(|path| path.components().count() == path_len)
            .map(|path| {
                dbg!(&path);
                dbg!(path.components().count());
                path
            })
            .collect();

        PathBuf::from(path)    
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

fn list_dirs_recursive<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| PathBuf::from(entry.path()))
        .filter(|path| path.is_dir())
        .collect()
}
