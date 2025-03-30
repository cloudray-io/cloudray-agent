use crate::generated::pb::a2o::a2o_message::A2oPayload;
use std::collections::VecDeque;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

pub struct MessageQueue {
    queue: Arc<RwLock<VecDeque<A2oPayload>>>,
}

impl MessageQueue {
    pub fn singleton() -> &'static Self {
        static INSTANCE: OnceLock<MessageQueue> = OnceLock::new();
        INSTANCE.get_or_init(|| Self {
            queue: Arc::new(RwLock::new(VecDeque::new())),
        })
    }

    pub async fn push(payload: A2oPayload) {
        MessageQueue::singleton()
            .queue
            .write()
            .await
            .push_back(payload);
    }

    pub async fn pop_front() -> Option<A2oPayload> {
        MessageQueue::singleton().queue.write().await.pop_front()
    }
}
