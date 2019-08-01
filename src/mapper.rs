use std::fs;
use std::io;
use std::path::PathBuf;

use hyper::{Body, Request, Response, StatusCode};
use hyper::header::{CONTENT_TYPE, HeaderValue};

pub fn get_body(base_path: &str, req: Request<Body>) -> io::Result<Response<Body>> {
   let mut response = Response::new(Body::empty());
    let path = PathBuf::from(format!("{}{}", base_path, req.uri().path()));
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry_path = entry?.path();
            if entry_path.is_file() {
                let body_text = fs::read_to_string(&entry_path)?;
                *response.body_mut() = Body::from(body_text);

                if let Some(ext) = entry_path.extension() {
                    if ext == "json" {
                        response.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                    }
                }

                return Ok(response);
            }
        }
    }

   *response.status_mut() = StatusCode::NOT_FOUND;
    
    Ok(response)
}
