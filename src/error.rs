fn error(msg: &str) {
    eprintln!("[error] - {}", msg);
    process::exit(1);
}