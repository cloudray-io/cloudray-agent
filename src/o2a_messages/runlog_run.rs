use crate::config::{set_send_report_at_now, RUNLOG_RUN_TIMEOUT};
use crate::generated::pb::a2o::a2o_message::A2oPayload;
use crate::generated::pb::a2o::{
    OutputStreamType, RunlogFailReason, RunlogFailed, RunlogFinished, RunlogOutputFragment,
    RunlogStarted,
};
use crate::generated::pb::o2a::RunlogRun;
use crate::message_queue::MessageQueue;
use crate::utils::current_timestamp_secs;
use std::fs;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::BufReader;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tracing::{debug, error};
use webterm_core::random::random_alphanumeric;

pub async fn process_run_runlog(message: RunlogRun) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(run_task(message))
}

async fn run_task(message: RunlogRun) -> anyhow::Result<()> {
    let parsed_script = message.parsed_script;
    let runlog_id = message.runlog_id;

    let script_path = PathBuf::from(format!(
        "/tmp/script_{}_{}.sh",
        runlog_id,
        random_alphanumeric(10)
    ));

    let _script_guard = ScriptGuard::new(script_path.clone());

    fs::write(&script_path, &parsed_script)?;

    let mut child = Command::new("bash")
        .arg(script_path.to_str().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    MessageQueue::push(A2oPayload::RunlogStarted(RunlogStarted {
        runlog_id,
        started_at_ts: current_timestamp_secs()?,
    }))
    .await;

    set_send_report_at_now().await;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let stdout_handle = tokio::spawn(handle_output(
        BufReader::new(stdout),
        runlog_id,
        OutputStreamType::OstStdout,
    ));
    let stderr_handle = tokio::spawn(handle_output(
        BufReader::new(stderr),
        runlog_id,
        OutputStreamType::OstStderr,
    ));

    match timeout(RUNLOG_RUN_TIMEOUT, async {
        let status = child.wait().await?;
        let _ = tokio::join!(stdout_handle, stderr_handle);
        Ok::<_, anyhow::Error>(status)
    })
    .await
    {
        Ok(result) => {
            let status = result?;

            MessageQueue::push(A2oPayload::RunlogFinished(RunlogFinished {
                runlog_id,
                finished_at_ts: current_timestamp_secs()?,
                exit_code: status.code().unwrap_or(-1),
            }))
            .await;
        }
        Err(_) => {
            child.kill().await?;

            MessageQueue::push(A2oPayload::RunlogFailed(RunlogFailed {
                runlog_id,
                failed_at_ts: current_timestamp_secs()?,
                fail_reason: RunlogFailReason::RfrTimeout as i32,
            }))
            .await;
        }
    }

    let _ = fs::remove_file(script_path);
    set_send_report_at_now().await;
    Ok(())
}

async fn handle_output<R>(
    mut reader: BufReader<R>,
    runlog_id: u64,
    stream_type: OutputStreamType,
) -> anyhow::Result<()>
where
    R: AsyncRead + Unpin,
{
    let mut buffer = [0; 1024];

    while let Ok(n) = reader.read(&mut buffer).await {
        if n == 0 {
            break;
        }
        MessageQueue::push(A2oPayload::RunlogOutputFragment(RunlogOutputFragment {
            runlog_id,
            output_at_ts: current_timestamp_secs()?,
            stream_type: stream_type as i32,
            output_fragment: buffer[..n].to_vec(),
        }))
        .await;
        set_send_report_at_now().await;
    }

    Ok(())
}

// RAII Guard for script file cleanup
struct ScriptGuard {
    path: PathBuf,
}

impl ScriptGuard {
    fn new(path: PathBuf) -> Self {
        ScriptGuard { path }
    }
}

impl Drop for ScriptGuard {
    fn drop(&mut self) {
        match fs::remove_file(&self.path) {
            Ok(_) => {
                debug!(path = %self.path.display(), "Removed temporary script file");
            }
            Err(e) => {
                if self.path.exists() {
                    error!(path = %self.path.display(), "Failed to remove script file: {}", e);
                }
            }
        }
    }
}
