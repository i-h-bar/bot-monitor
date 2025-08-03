use crate::domain::register::{Register, RegisterEntry, RegisterError};
use async_trait::async_trait;
use serenity::all::UserId;
use tokio::sync::RwLock;

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

    async fn remove(&self, bot_id: u64, user_id: u64) -> Result<(), RegisterError> {
        let mut index: Option<usize> = None;
        log::info!("removing bot {}", bot_id);
        for (i, entry) in self.0.read().await.iter().enumerate() {
            if entry.bot_id == bot_id && entry.user_id == user_id {
                log::info!("found in index {}", i);
                index = Some(i);
            }
        }

        if let Some(i) = index {
            self.0.write().await.remove(i);
        }
        log::info!("{:?}", self.0.read().await);
        Ok(())
    }

    async fn list(&self, user_id: u64) -> Result<Vec<RegisterEntry>, RegisterError> {
        Ok(self
            .0
            .read()
            .await
            .iter()
            .filter_map(|entry| {
                if entry.user_id == user_id {
                    Some(entry.clone())
                } else {
                    None
                }
            })
            .collect())
    }
}
