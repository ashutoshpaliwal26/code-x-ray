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
use std::path::Path;

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

#[napi]
pub fn ast_of_file(path:String) -> FileReport {
    let path_buf = Path::new(&path);
    let path_str = path_buf.to_string_lossy().to_string();

    let content = match fs::read_to_string(&path_str){
        Ok(c) => c,
        Err(e) => return FileReport {
            path : path_str,
            language : "unknown".to_string(),
            size_bytes : 0,
            symbols: Vec::new(),
            error : Some(e.to_string())
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File};
    use std::io::Write;
    use tempfile::tempdir;

    // Helper function to create a temporary test file
    fn create_test_file(content: &str, extension: &str) -> (tempfile::TempDir, String) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(format!("test.{}", extension));
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", content).unwrap();
        (dir, file_path.to_string_lossy().into_owned())
    }

    #[test]
    fn test_ast_of_file_valid_rust() {
        let (_dir, path) = create_test_file(
            "fn main() {\n    println!(\"Hello, world!\");\n}",
            "rs"
        );
        
        let result = ast_of_file(path.clone());
        assert!(result.error.is_none());
        assert_eq!(result.path, path);
        assert_eq!(result.language, "Rust");
        assert!(!result.symbols.is_empty(), "Should find symbols in valid Rust code");
    }

    #[test]
    fn test_ast_of_file_nonexistent() {
        let result = ast_of_file("/nonexistent/file.rs".to_string());
        assert!(result.error.is_some());
        assert!(result.symbols.is_empty());
        assert_eq!(result.size_bytes, 0);
    }

    // Add more test cases here...
}