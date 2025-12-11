use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct CreateEntry {
    pub user_id: String,
    pub bot_id: String,
    pub version: usize,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CreateEntryEvent {
    fn entry(&self) -> CreateEntry;
    fn is_bot(&self) -> bool;
    async fn not_a_bot_message(&self);
    async fn entry_added_message(&self);
    async fn failed_message(&self);
}
