use axum::{extract::ConnectInfo, Json};
use std::{
    fs,
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::process::Command;

use crate::{
    models::{ExecuteRequest, ExecuteResponse, LanguageMetadata, StatusResponse},
    utils::{
        code::{compile_code, execute_code, ProcInput, ProcLimit},
        language,
        log::log_request,
    },
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

pub async fn execute(Json(body): Json<ExecuteRequest>) -> Json<ExecuteResponse> {
    // TODO...
    // https://github.com/engineer-man/piston/blob/919076e20924c1a267a4e1885d17c896c2e96c80/api/src/job.js#L113
    // https://man7.org/linux/man-pages/man2/nice.2.html
    // https://man7.org/linux/man-pages/man1/timeout.1.html
    // https://man7.org/linux/man-pages/man1/prlimit.1.html
    // https://github.com/sanpii/lxc-rs/blob/main/examples/run_command.rs https://github.com/servo/gaol/tree/master

    let metadata = language::find_metadata(body.language.clone()).unwrap();
    let mut src: Option<String> = None;

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string();
    let exec_dir = format!("task-{}", since_the_epoch).to_string();

    let ext = metadata.extension.clone();

    fs::create_dir(exec_dir.clone()).unwrap();
    for file in body.files {
        let path = format!("{}/{}", exec_dir, file.name);
        fs::write(path.clone(), file.content).expect("Unable to write file");

        if file.name.ends_with(ext.as_str()) {
            src = Some(path);
        }
    }

    let file = src.unwrap();

    let comp_result = compile_code(
        metadata,
        file.clone(),
        ProcLimit {
            timeout: body.compile_timeout,
            memory_limit: body.compile_memory_limit,
        },
    )
    .await
    .unwrap();

    let run_result = execute_code(
        file.replace(ext.as_str(), "out"),
        ProcInput {
            args: body.args,
            stdin: body.stdin,
        },
        ProcLimit {
            timeout: body.run_timeout,
            memory_limit: body.run_timeout,
        },
    )
    .await
    .unwrap();

    fs::remove_dir_all(exec_dir).unwrap();

    Json(ExecuteResponse {
        language: body.language,
        run: run_result,
        compile: Some(comp_result),
    })
}

pub async fn _placeholder() -> axum::response::Html<&'static str> {
    axum::response::Html("works")
}
