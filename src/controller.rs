use axum::Json;
use std::fs;
use tokio::process::Command;

use crate::{models::StatusResponse, utils::language};

pub async fn status() -> Json<StatusResponse> {
    let paths = fs::read_dir("./packages").unwrap();

    let mut did_pass: Vec<String> = vec![];
    let mut didnt_pass: Vec<String> = vec![];

    for path in paths {
        let dir = path.unwrap().path();
        if !dir.is_dir() {
            continue;
        }

        let metadata = language::get_metadata(&dir).expect("metadata should be present");

        let output = Command::new("sh")
            .arg(format!("{}/run.sh", dir.display()))
            .arg(format!("{}/test.{}", dir.display(), metadata.extension))
            .output()
            .await
            .unwrap();

        if output.status.success() && output.stdout.eq("OK\n".as_bytes()) {
            did_pass.push(metadata.language);
        } else {
            didnt_pass.push(metadata.language);
        }
    }

    Json(StatusResponse {
        status: format!("{}/{}", did_pass.len(), didnt_pass.len() + did_pass.len()),
        passed: did_pass,
        failed: didnt_pass,
    })
}

pub async fn placeholder() -> axum::response::Html<&'static str> {
    axum::response::Html("works")
}
