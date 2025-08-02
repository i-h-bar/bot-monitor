mod logic;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

pub struct RegisterEntry {
    id: Uuid,
    pub bot_id: String,
    pub user_id: String,
    pub last_successful_ping: DateTime<Utc>
}

#[derive(Error, Debug)]
#[error("Could not create register entry")]
pub struct RegisterError;


#[async_trait]
pub trait Register {
    async fn check(&self, bot_id: &str);
    async fn add(&self, entry: RegisterEntry) -> Result<(), RegisterError>;
}