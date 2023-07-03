use serde::{Deserialize, Serialize};

use crate::utils::code::ProcResult;

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub passed: Vec<String>,
    pub failed: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LanguageMetadata {
    pub language: String,
    pub speed_index: f32,
    pub is_compiled: bool,
    pub extension: String,
    pub aliases: Vec<String>,
}

#[derive(Deserialize)]
pub struct ExecuteRequestFiles {
    pub name: String,
    pub content: String,
    pub encoding: Option<String>,
}

#[derive(Deserialize)]
pub struct ExecuteRequest {
    pub language: String,
    pub files: Vec<ExecuteRequestFiles>,
    pub stdin: Option<String>,
    pub args: Option<Vec<String>>,
    pub run_timeout: Option<i64>,
    pub run_memory_limit: Option<i64>,
    pub compile_timeout: Option<i64>,
    pub compile_memory_limit: Option<i64>,
}

#[derive(Serialize)]
pub struct ExecuteResponse {
    pub language: String,
    pub run: ProcResult,
    pub compile: Option<ProcResult>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CodeTest {
    pub input: String,
    pub output: String,
    pub run_timeout: i64,
    pub run_memory_limit: i64,
    pub points: Option<u32>,
    pub passed: Option<bool>,
}

#[derive(Deserialize)]
pub struct TestRequest {
    pub language: String,
    pub files: Vec<ExecuteRequestFiles>,
    pub tests: Vec<CodeTest>,
    pub compile_timeout: Option<i64>,
    pub compile_memory_limit: Option<i64>,
}
