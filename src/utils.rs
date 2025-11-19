use std::time::{Duration, SystemTime};

pub fn current_timestamp() -> anyhow::Result<Duration> {
    Ok(SystemTime::now().duration_since(std::time::UNIX_EPOCH)?)
}

pub fn current_timestamp_secs() -> anyhow::Result<u64> {
    current_timestamp().map(|d| d.as_secs())
}

pub fn current_timestamp_nanos() -> anyhow::Result<u64> {
    current_timestamp().map(|d| d.as_nanos() as u64)
}
