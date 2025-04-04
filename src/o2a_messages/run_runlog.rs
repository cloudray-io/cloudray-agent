use crate::generated::pb::a2o::a2o_message::A2oPayload;
use crate::generated::pb::a2o::{
    OutputStreamType, RunlogFailReason, RunlogFailed, RunlogFinished, RunlogOutputFragment,
    RunlogStarted,
};
use crate::generated::pb::o2a::RunRunlog;
use crate::message_queue::MessageQueue;
use crate::utils::current_timestamp_secs;
use std::fs;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::BufReader;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;
use tokio::time::timeout;
use webterm_core::random::random_alphanumeric;

pub async fn process_run_runlog(message: RunRunlog) -> anyhow::Result<()> {
    let parsed_script = message.parsed_script;
    let runlog_id = message.runlog_id;

    let script_path = PathBuf::from(format!(
        "/tmp/script_{}_{}.sh",
        runlog_id,
        random_alphanumeric(10)
    ));

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

    match timeout(Duration::from_secs(60 * 60), async {
        let status = child.wait().await?;
        let _ = tokio::join!(stdout_handle, stderr_handle);
        Ok::<_, anyhow::Error>(status)
    })
    .await
    {
        Ok(result) => {
            let status = result?;
            let _ = fs::remove_file(script_path);

            MessageQueue::push(A2oPayload::RunlogFinished(RunlogFinished {
                runlog_id,
                finished_at_ts: current_timestamp_secs()?,
                exit_code: status.code().unwrap_or(-1) as i32,
            }))
            .await;
        }
        Err(_) => {
            child.kill().await?;
            let _ = fs::remove_file(script_path);

            MessageQueue::push(A2oPayload::RunlogFailed(RunlogFailed {
                runlog_id,
                failed_at_ts: current_timestamp_secs()?,
                fail_reason: RunlogFailReason::RfrTimeout as i32,
            }))
            .await;
        }
    }

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
    }

    Ok(())
}
