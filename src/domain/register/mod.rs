mod logic;

use async_trait::async_trait;
use serenity::all::UserId;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct RegisterEntry {
    pub bot_id: u64,
    pub user_id: u64,
}

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("Could not create register entry")]
    EntryCreationError,
    #[error("Could not fetch register entry")]
    EntryFetchError,
}

#[async_trait]
pub trait Register {
    async fn fetch(&self, bot_id: u64) -> Option<Vec<RegisterEntry>>;
    async fn add(&self, entry: RegisterEntry) -> Result<(), RegisterError>;
    async fn remove(&self, bot_id: u64, user_id: u64) -> Result<(), RegisterError>;
    async fn list(&self, user_id: u64) -> Result<Vec<RegisterEntry>, RegisterError>;
}
