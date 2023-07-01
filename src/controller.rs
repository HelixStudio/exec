use axum::{extract::ConnectInfo, Json};
use std::{fs, net::SocketAddr};
use tokio::process::Command;

use crate::{
    models::{LanguageMetadata, StatusResponse},
    utils::{language, log::log_request},
};

const LOGGING: bool = false;

pub async fn status(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Json<StatusResponse> {
    if LOGGING {
        log_request(addr, "status");
    }

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

pub async fn runtimes(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Json<Vec<LanguageMetadata>> {
    if LOGGING {
        log_request(addr, "runtimes");
    }

    let mut langs: Vec<LanguageMetadata> = vec![];

    for path in fs::read_dir("./packages").unwrap() {
        let dir = path.unwrap().path();
        if dir.is_file() {
            continue;
        }

        let metadata = language::get_metadata(&dir).expect("metadata should be present");

        langs.push(metadata);
    }

    Json(langs)
}

pub async fn placeholder() -> axum::response::Html<&'static str> {
    axum::response::Html("works")
}
