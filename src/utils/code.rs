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

use crate::models::{CodeTest, LanguageMetadata};

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

fn limit_process(lim: ProcLimit) -> (String, Vec<String>) {
    const MAX_PROC_COUNT: u32 = 1024;
    const MAX_OPEN_FILES: u32 = 1024;
    const MAX_FILE_SIZE: u32 = 33554432; // 2^25

    let timeout = (lim.timeout.unwrap_or(2_000) as f64) / 1000.0; // convert ms to s => 10 seconds
    let memory = lim.memory_limit.unwrap_or(1_000_00); // 100kb

    let call: Vec<String> = vec![
        // container runtime:
        String::from("crate"),
        // timeout:
        String::from("timeout"),
        String::from("-s"),
        String::from("9"),
        format!("{}", timeout),
        // prlimit:
        String::from("prlimit"),
        format!("--nproc={}", MAX_PROC_COUNT.to_string()),
        format!("--nofile={}", MAX_OPEN_FILES.to_string()),
        format!("--fsize={}", MAX_FILE_SIZE.to_string()),
        format!("--stack={}", memory),
        format!("--cpu={}", timeout),
    ];

    return (String::from("sudo"), call);
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

    let limits = limit_process(lim);

    let child_handle = Command::new(limits.0)
        .args(limits.1)
        .arg("sh")
        .arg(compiler_script)
        .arg(filepath.clone())
        .arg(filepath.replace(lang.extension.as_str(), "out"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    let (res, duration) = time_process(child_handle).await;

    let stdout = String::from_utf8(res.stdout).unwrap();
    let stderr = String::from_utf8(res.stderr).unwrap();

    Ok(ProcResult {
        stdout: stdout.clone(),
        stderr: stderr.clone(),
        output: if stderr.len() == 0 { stdout } else { stderr },
        code: res.status.code().unwrap_or(0),
        signal: 0,
        time: duration.as_millis(),
    })
}

pub async fn execute_code(
    executable: String,
    input: ProcInput,
    lim: ProcLimit,
) -> Result<ProcResult, &'static str> {
    let limits = limit_process(lim);

    let mut child_handle = Command::new(limits.0)
        .args(limits.1)
        .arg(executable)
        .args(&input.args.unwrap_or(vec![]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process"); // BUG when compile error

    if input.stdin.is_some() {
        let mut stdin = child_handle.stdin.take().unwrap();
        stdin.write(input.stdin.unwrap().as_bytes()).await.unwrap();
        drop(stdin);
    }

    let (res, duration) = time_process(child_handle).await;

    let stdout = String::from_utf8(res.stdout).unwrap();
    let stderr = String::from_utf8(res.stderr).unwrap();

    Ok(ProcResult {
        stdout: stdout.clone(),
        stderr: stderr.clone(),
        output: if stderr.len() == 0 { stdout } else { stderr },
        code: res.status.code().unwrap_or(0),
        signal: 0,
        time: duration.as_millis(),
    })
}

pub async fn test_code(exec: String, test: CodeTest) -> bool {
    let output = execute_code(
        exec,
        ProcInput {
            stdin: Some(test.input),
            args: None,
        },
        ProcLimit {
            timeout: Some(test.run_timeout),
            memory_limit: Some(test.run_memory_limit),
        },
    )
    .await
    .unwrap();

    if output.stdout == test.output {
        return true;
    }

    false
}
