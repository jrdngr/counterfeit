use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hyper::{Body, Request, Response, StatusCode, Method};
use hyper::header::{self, HeaderValue};

pub fn get_body(base_path: &str, req: Request<Body>) -> io::Result<Response<Body>> {
    let mut response = Response::new(Body::empty());
    
    if let Some(path) = req.uri().path().split('?').nth(0) {
        let full_path = PathBuf::from(format!("{}{}", base_path, path));
        let file_path = choose_file(&full_path, req.method())?;

        let body_text = fs::read_to_string(&file_path)?;
        *response.body_mut() = Body::from(body_text);

        if let Some(ext) = file_path.extension() {
            if ext == "json" {
                response.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
            }
        }

        return Ok(response);

    }

   *response.status_mut() = StatusCode::NOT_FOUND;
    
    Ok(response)
}

pub fn choose_file(path: &Path, method: &Method) ->io::Result<PathBuf> {
    let available_files = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|file| file.path())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>();

    match available_files.into_iter().nth(0) {
        Some(file) => Ok(file),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "No files available")),
    }
}
