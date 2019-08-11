use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

use hyper::{Body, Request, Response, StatusCode, Method};
use walkdir::WalkDir;

use crate::config::CounterfeitRunConfig;
use crate::MultiFileIndexMap;


pub trait DirPicker {
    fn pick_directory(&mut self, request: &Request<Body>) -> io::Result<PathBuf>;
}

pub struct StandardDirPicker {
    base_path: String,
}

impl StandardDirPicker {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }
}

impl DirPicker for StandardDirPicker {
    fn pick_directory(&mut self, request: &Request<Body>) -> io::Result<PathBuf> {
        let path = request.uri().path();
        let full_path = PathBuf::from(format!("{}{}", &self.base_path, path));

        if full_path.exists() {
            return Ok(full_path);
        }

        let all_paths = list_dirs_recursive(&self.base_path);

        let matching_path = all_paths
            .iter()
            .filter_map(|potential_path| PathMatch::get_match(&path, &potential_path, &self.config))
            .max_by(|p1, p2| p1.num_exact_matches.cmp(&p2.num_exact_matches));

        match matching_path {
            Some(p) => PathBuf::from(p),
            None => PathBuf::from(path),
        }
    }
}

struct PathMatch {
    path: PathBuf,
    num_exact_matches: usize,
}

impl From<PathMatch> for PathBuf {
    fn from(path_match: PathMatch) -> PathBuf {
        path_match.path
    }
}

impl PathMatch {
    pub fn get_match(
        target: &Path,
        potential: &Path,
        config: &CounterfeitRunConfig,
    ) -> Option<Self> {
        if target.components().count() != potential.components().count() {
            return None;
        }

        let (exact_count, param_count) = target.components().zip(potential.components()).fold(
            (0, 0),
            |(exact_acc, param_acc), (tc, pc)| {
                if tc == pc {
                    (exact_acc + 1, param_acc)
                } else if is_param(&pc, config) {
                    (exact_acc, param_acc + 1)
                } else {
                    (exact_acc, param_acc)
                }
            },
        );

        if exact_count + param_count == target.components().count() {
            let result = PathMatch {
                path: PathBuf::from(potential),
                num_exact_matches: exact_count,
            };
            Some(result)
        } else {
            None
        }
    }
}

fn is_param(component: &Component, config: &CounterfeitRunConfig) -> bool {
    match component.as_os_str().to_str() {
        Some(s) => s.starts_with(&config.prefix) && s.ends_with(&config.postfix),
        None => false,
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
