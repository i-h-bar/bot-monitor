use crate::ports::inbound::client::events::create::CreateEntry;
use crate::ports::inbound::client::events::list::ListEntriesPayload;
use crate::ports::inbound::client::events::remove::RemoveEntry;
use async_trait::async_trait;
use thiserror::Error;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct RegisterEntry {
    pub bot_id: String,
    pub user_id: String,
    pub guild_id: Option<String>,
}

#[cfg_attr(test, derive(Clone, PartialEq))]
#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum RegisterError {
    #[error("Could not create register entry")]
    EntryCreationError,
    #[error("Could not fetch register entry")]
    EntryFetchError,
    #[error("Could not remove register entry")]
    EntryRemoveError,
    #[error("Could not update register entry")]
    EntryUpdateError,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Register {
    async fn fetch(&self, bot_id: String) -> Option<Vec<RegisterEntry>>;
    async fn add(&self, entry: CreateEntry) -> Result<(), RegisterError>;
    async fn remove(&self, entry: RemoveEntry) -> Result<(), RegisterError>;
    async fn list(&self, entry: ListEntriesPayload) -> Result<Vec<RegisterEntry>, RegisterError>;
    async fn fetch_by_guild(&self, guild_id: String) -> Result<Vec<RegisterEntry>, RegisterError>;
    async fn fetch_missing_guild_id(&self) -> Result<Vec<RegisterEntry>, RegisterError>;
    async fn update_guild_id(
        &self,
        bot_id: String,
        user_id: String,
        guild_id: String,
    ) -> Result<(), RegisterError>;
}
