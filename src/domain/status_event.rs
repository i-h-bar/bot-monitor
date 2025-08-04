use crate::domain::register::RegisterEntry;
use async_trait::async_trait;

pub enum EventStatus {
    Offline,
    Online,
    NA,
}

#[async_trait]
pub trait StatusEvent {
    fn bot_id(&self) -> String;
    fn status(&self) -> EventStatus;
    async fn is_bot(&self) -> bool;

    async fn send_offline_warning(&self, entries: Vec<RegisterEntry>);
    async fn send_online_message(&self, entries: Vec<RegisterEntry>);
}
