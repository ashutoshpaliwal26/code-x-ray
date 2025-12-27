// src/models.rs
use napi_derive::napi;
use serde::Serialize;

#[napi(object)]
#[derive(Debug, Serialize, Clone)]
pub struct SourceLocation {
    pub row: u32,
    pub column: u32,
}

#[napi(object)]
#[derive(Debug, Serialize, Clone)]
pub struct CodeSymbol {
    pub name: String,
    pub kind: String,
    pub start: SourceLocation,
    pub end: SourceLocation,
    pub signature: String,
}

#[napi(object)]
#[derive(Debug, Serialize, Clone)]
pub struct FileReport {
    pub path: String,
    pub language: String,
    pub size_bytes: u32,
    pub symbols: Vec<CodeSymbol>,
    pub error: Option<String>,
}

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct ScanResult {
    pub root_dir: String,
    pub duration_ms: f64,
    pub files_scanned: u32,
    pub files: Vec<FileReport>,
}