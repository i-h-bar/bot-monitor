use async_trait::async_trait;

use crate::ports::outbound::register::RegisterEntry;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, derive(Clone))]
pub enum BotStates {
    Offline,
    Online,
    NA,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait StatusEvent {
    fn bot_id(&self) -> String;
    fn state(&self) -> BotStates;
    async fn is_bot(&self) -> bool;
    async fn send_offline_warning(&self, entries: Vec<RegisterEntry>);
    async fn send_online_message(&self, entries: Vec<RegisterEntry>);
}
