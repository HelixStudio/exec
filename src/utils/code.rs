use serde::Serialize;
use std::{
    process::{Output, Stdio},
    time::Duration,
};
use tokio::{
    io::AsyncWriteExt,
    process::{Child, Command},
    time::Instant,
};

use crate::models::LanguageMetadata;

pub struct ProcLimit {
    pub timeout: Option<i64>,
    pub memory_limit: Option<i64>,
}

pub struct ProcInput {
    pub stdin: Option<String>,
    pub args: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct ProcResult {
    pub stdout: String,
    pub stderr: String,
    pub output: String,
    pub code: i32,
    pub signal: i32,
    pub time: u128,
}

fn limit_process(child_handle: &Child, lim: ProcLimit) {
    // TODO
}

async fn time_process(child_handle: Child) -> (Output, Duration) {
    let start_time = Instant::now();

    let res = child_handle
        .wait_with_output()
        .await
        .expect("Failed to wait for child process");

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);

    (res, elapsed_time)
}

pub async fn compile_code(
    lang: LanguageMetadata,
    filepath: String,
    lim: ProcLimit,
) -> Result<ProcResult, &'static str> {
    if !lang.is_compiled {
        return Err("Language is not compiled");
    }

    if !std::path::Path::new(filepath.as_str()).exists() {
        return Err("Source file does not exist");
    }

    let compiler_script = format!("./packages/{}/compile.sh", lang.language);

    let child_handle = Command::new("sh")
        .arg(compiler_script)
        .arg(filepath.clone())
        .arg(filepath.replace(lang.extension.as_str(), "out"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    limit_process(&child_handle, lim);

    let (res, duration) = time_process(child_handle).await;

    let stdout = String::from_utf8(res.stdout).unwrap();
    let stderr = String::from_utf8(res.stderr).unwrap();

    Ok(ProcResult {
        stdout: stdout.clone(),
        stderr: stderr.clone(),
        output: if stderr.len() == 0 { stdout } else { stderr },
        code: res.status.code().unwrap(),
        signal: 0,
        time: duration.as_millis(),
    })
}

pub async fn execute_code(
    executable: String,
    input: ProcInput,
    lim: ProcLimit,
) -> Result<ProcResult, &'static str> {
    let mut child_handle = Command::new(executable)
        .args(&input.args.unwrap_or(vec![]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    if input.stdin.is_some() {
        let mut stdin = child_handle.stdin.take().unwrap();
        stdin.write(input.stdin.unwrap().as_bytes()).await.unwrap();
        drop(stdin);
    }

    limit_process(&child_handle, lim);

    let (res, duration) = time_process(child_handle).await;

    let stdout = String::from_utf8(res.stdout).unwrap();
    let stderr = String::from_utf8(res.stderr).unwrap();

    Ok(ProcResult {
        stdout: stdout.clone(),
        stderr: stderr.clone(),
        output: if stderr.len() == 0 { stdout } else { stderr },
        code: res.status.code().unwrap(),
        signal: 0,
        time: duration.as_millis(),
    })
}
