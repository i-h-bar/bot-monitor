use async_trait::async_trait;
use serenity::all::UserId;
use tokio::sync::RwLock;
use crate::domain::register::{Register, RegisterEntry, RegisterError};

pub struct LocalRegister(RwLock<Vec<RegisterEntry>>);


impl LocalRegister {
    pub fn new() -> Self {
        Self(RwLock::new(Vec::new()))
    }
}

#[async_trait]
impl Register for LocalRegister {
    async fn fetch(&self, bot_id: u64) -> Option<RegisterEntry> {
        for entry in self.0.read().await.iter() {
            if entry.bot_id == bot_id {
                return Some(entry.clone());
            }
        }

        None
    }

    async fn add(&self, entry: RegisterEntry) -> Result<(), RegisterError> {
        self.0.write().await.push(entry);
        log::info!("{:?}", self.0.read().await);
        Ok(())
    }
}