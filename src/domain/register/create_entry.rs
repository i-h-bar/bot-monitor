use async_trait::async_trait;
use crate::domain::app::App;
use crate::domain::register::Register;


pub struct CreateEntry {
    pub user_id: String,
    pub bot_id: String,
    pub entry_version: usize,
}

#[async_trait]
pub trait CreateEntryEvent {
    async fn entry(&self) -> CreateEntry;
    async fn is_bot(&self) -> bool;
    async fn not_a_bot_message(&self);
    async fn entry_added_message(&self);
    async fn failed_message(&self);
}


impl<R> App<R> where R: Register {
    pub async fn add_to_register<E: CreateEntryEvent>(&self, event: E) {
        if !event.is_bot().await {
            event.not_a_bot_message().await;
            return
        }

        if let Err(why) = self.register.add(event.entry().await).await {
            log::warn!("Failed to add new entry - {why:?}");
            event.failed_message().await;
        } else {
            log::info!("Added new entry");
            event.entry_added_message().await;
        };
    }
}