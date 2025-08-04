pub mod events;
mod logic;

use crate::domain::register::events::remove::RemoveEntry;
use async_trait::async_trait;
use events::create::CreateEntry;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct RegisterEntry {
    pub bot_id: String,
    pub user_id: String,
}

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum RegisterError {
    #[error("Could not create register entry")]
    EntryCreationError,
    #[error("Could not fetch register entry")]
    EntryFetchError,
    #[error("Could not remove register entry")]
    EntryRemoveError,
}

#[async_trait]
pub trait Register {
    async fn fetch(&self, bot_id: String) -> Option<Vec<RegisterEntry>>;
    async fn add(&self, entry: CreateEntry) -> Result<(), RegisterError>;
    async fn remove(&self, entry: RemoveEntry) -> Result<(), RegisterError>;
    async fn list(&self, user_id: String) -> Result<Vec<RegisterEntry>, RegisterError>;
}
