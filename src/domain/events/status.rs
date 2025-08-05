use crate::domain::register::RegisterEntry;
use async_trait::async_trait;

pub enum BotStates {
    Offline,
    Online,
    NA,
}

#[async_trait]
pub trait StatusEvent {
    fn bot_id(&self) -> String;
    fn state(&self) -> BotStates;
    async fn is_bot(&self) -> bool;

    async fn send_offline_warning(&self, entries: Vec<RegisterEntry>);
    async fn send_online_message(&self, entries: Vec<RegisterEntry>);
}
