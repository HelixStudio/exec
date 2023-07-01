use std::{
    fs::OpenOptions,
    io::Write,
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn gen_log_entry(addr: SocketAddr, method: String) -> String {
    let path = format!("/api/v1/{}", method);
    let ip_addr = addr.ip().to_string();
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    format!("{} {} {}\n", ip_addr, path, since_the_epoch.as_millis())
}

pub fn log_request(addr: SocketAddr, method: &str) {
    let entry = gen_log_entry(addr, String::from(method));

    let mut log_file = OpenOptions::new()
        .append(true)
        .open("log")
        .expect("cannot open file");

    log_file.write_all(entry.as_bytes()).unwrap();
}
