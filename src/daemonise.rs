use crate::panic_error::PanicError;

#[cfg(unix)]
pub fn daemonise() -> Result<(), PanicError> {
    use daemonize::Daemonize;
    use std::fs::OpenOptions;

    let log_path = "/tmp/cloudray-agent.log";

    let stdout = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .map_err(|_| {
            PanicError::RuntimeError(format!("Could not create stdout log file: {}", log_path))
        })?;

    let stderr = stdout.try_clone().map_err(|_| {
        PanicError::RuntimeError(format!("Could not create stderr log file: {}", log_path))
    })?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/cloudray-agent.pid")
        .stdout(stdout)
        .stderr(stderr);

    println!("Running in background, logging to {}", log_path);

    match daemonize.start() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to run in background: {}", e);
            Err(PanicError::RuntimeError(e.to_string()))
        }
    }
}

#[cfg(not(unix))]
pub fn daemonise() -> Result<(), PanicError> {
    Err(PanicError::RuntimeError(
        "-d option is not supported on this platform".to_string(),
    ))
}
