use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry};
use async_trait::async_trait;

pub struct ListEntriesPayload {
    pub user_id: String,
}

#[async_trait]
pub trait ListEvent {
    fn payload(&self) -> ListEntriesPayload;
    async fn failed_message(&self);
    async fn success_message(&self, entries: Vec<RegisterEntry>);
    async fn empty_message(&self);
}

impl<R> App<R>
where
    R: Register,
{
    pub async fn list_entries<L: ListEvent>(&self, event: L) {
        let entries = match self.register.list(event.payload().user_id).await {
            Ok(entries) => entries,
            Err(why) => {
                log::warn!("Failed to list entries in register: {why:?}");
                event.failed_message().await;
                return;
            }
        };

        if entries.is_empty() {
            event.empty_message().await;
        } else {
            event.success_message(entries).await;
        }
    }
}
