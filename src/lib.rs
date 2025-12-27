#![deny(clippy::all)]

// Declare modules
mod models;
mod languages;
mod parser;

use napi_derive::napi;
use rayon::prelude::*;
use ignore::WalkBuilder;
use std::fs;
use std::time::Instant;

use models::{ScanResult, FileReport};
use languages::SupportedLanguage;
use parser::CodeParser;

#[napi]
pub fn scan_project(root_path: String) -> ScanResult {
    let start = Instant::now();

    let walker = WalkBuilder::new(&root_path)
        .hidden(false)
        .git_ignore(true)
        .build();

    let paths: Vec<_> = walker
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    let files: Vec<FileReport> = paths.par_iter().map(|path| {
        let path_str = path.to_string_lossy().to_string();
        
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return FileReport {
                path: path_str,
                language: "unknown".to_string(),
                size_bytes: 0,
                symbols: vec![],
                error: Some(e.to_string()),
            },
        };

        let lang = SupportedLanguage::from_path(&path_str);
        let symbols = CodeParser::parse(&content, lang);

        FileReport {
            path: path_str,
            language: format!("{:?}", lang),
            size_bytes: content.len() as u32,
            symbols,
            error: None,
        }
    }).collect();

    ScanResult {
        root_dir: root_path,
        duration_ms: start.elapsed().as_secs_f64() * 1000.0,
        files_scanned: files.len() as u32,
        files,
    }
}