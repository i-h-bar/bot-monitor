use async_trait::async_trait;

use crate::ports::outbound::register::RegisterEntry;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, derive(Clone))]
pub struct ListEntriesPayload {
    pub user_id: String,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ListEvent {
    fn payload(&self) -> ListEntriesPayload;
    async fn failed_message(&self);
    async fn success_message(&self, entries: Vec<RegisterEntry>);
    async fn empty_message(&self);
}
