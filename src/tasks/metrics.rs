use crate::config::{COLLECT_METRICS_EVERY, CPU_MAX_SAMPLES, CPU_SAMPLE_INTERVAL};
use crate::generated::pb::a2o::a2o_message::A2oPayload;
use crate::generated::pb::a2o::{MetricEvent, MetricType};
use crate::message_queue::MessageQueue;
use crate::utils::current_timestamp_secs;
use std::sync::Arc;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::debug;

type CpuSamples = Arc<Mutex<Vec<f32>>>;

pub async fn run_metrics_task() -> JoinHandle<anyhow::Result<()>> {
    let samples: CpuSamples = Arc::new(Mutex::new(Vec::new()));
    tokio::spawn(sample_cpu_task(samples.clone()));
    tokio::spawn(metrics_task(samples))
}

async fn metrics_task(samples: CpuSamples) -> anyhow::Result<()> {
    let mut sys = System::new();
    sys = collect_boot_metrics(sys).await?;

    loop {
        sys = collect_metrics(sys, samples.clone()).await?;
        sleep(COLLECT_METRICS_EVERY).await;
    }
}

async fn sample_cpu_task(samples: CpuSamples) {
    let mut sys = System::new();
    loop {
        sys.refresh_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
        );
        let usage = sys.global_cpu_usage();
        {
            let mut sample_lock = samples.lock().await;
            sample_lock.push(usage);
            if sample_lock.len() > CPU_MAX_SAMPLES {
                sample_lock.remove(0);
            }
        }
        sleep(CPU_SAMPLE_INTERVAL).await;
    }
}

async fn collect_metrics(mut sys: System, samples: CpuSamples) -> anyhow::Result<System> {
    debug!("Collecting metrics");
    sys.refresh_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::with_ram(
            MemoryRefreshKind::nothing().with_ram(),
        )),
    );

    let used_memory = sys.used_memory();
    let event = MetricEvent {
        metric_type: MetricType::MetricMemoryUsedBytes as i32,
        metric_at_ts: current_timestamp_secs()?,
        label: String::default(),
        value: used_memory as f64,
    };

    MessageQueue::push(A2oPayload::MetricEvent(event)).await;

    let avg_cpu = {
        let sample_lock = samples.lock().await;
        if sample_lock.is_empty() {
            0.0
        } else {
            let sum: f32 = sample_lock.iter().sum();
            sum / sample_lock.len() as f32
        }
    };

    let event = MetricEvent {
        metric_type: MetricType::MetricCpuGlobalPercent as i32,
        metric_at_ts: current_timestamp_secs()?,
        label: String::default(),
        value: avg_cpu as f64,
    };

    MessageQueue::push(A2oPayload::MetricEvent(event)).await;

    Ok(sys)
}

async fn collect_boot_metrics(mut sys: System) -> anyhow::Result<System> {
    debug!("Collecting boot metrics");
    sys.refresh_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
    );

    let total_memory = sys.total_memory();
    let event = MetricEvent {
        metric_type: MetricType::MetricMemoryTotalBytes as i32,
        metric_at_ts: current_timestamp_secs()?,
        label: String::default(),
        value: total_memory as f64,
    };

    MessageQueue::push(A2oPayload::MetricEvent(event)).await;

    Ok(sys)
}
