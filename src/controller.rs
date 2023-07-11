use axum::{extract::ConnectInfo, Json};
use std::{
    fs,
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::process::Command;

use crate::{
    models::{
        CodeTest, ExecuteRequest, ExecuteResponse, LanguageMetadata, StatusResponse, TestRequest,
    },
    utils::{
        code::{compile_code, execute_code, test_code, ProcInput, ProcLimit},
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

    let comp_result = compile_code(metadata, file.clone()).await.unwrap();

    let run_result = execute_code(
        file.replace(&format!(".{}", ext), ".out"),
        ProcInput {
            args: body.args,
            stdin: body.stdin,
        },
        ProcLimit {
            timeout: body.run_timeout,
            memory_limit: body.run_memory_limit,
            in_container: Some(true),
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

pub async fn test(Json(body): Json<TestRequest>) -> Json<Vec<CodeTest>> {
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

    let _compile_result = compile_code(metadata, file.clone()).await.unwrap();

    let executable = file.replace(&format!(".{}", ext), ".out");
    let mut result_tests: Vec<CodeTest> = vec![];

    for test in body.tests {
        let passed = test_code(executable.clone(), test.clone()).await;
        result_tests.push(CodeTest {
            input: test.input,
            output: test.output,
            run_timeout: test.run_timeout,
            run_memory_limit: test.run_memory_limit,
            points: if passed { test.points } else { Some(0) },
            passed: Some(passed),
        })
    }

    fs::remove_dir_all(exec_dir).unwrap();

    Json(result_tests)
}

pub async fn _placeholder() -> axum::response::Html<&'static str> {
    axum::response::Html("works")
}
