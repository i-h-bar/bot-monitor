use async_trait::async_trait;
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
    async fn check(&self, bot_id: &str) {
        for entry in self.0.read().await.iter() {
            if entry.bot_id == bot_id {
                return;
            }
        }
    }

    async fn add(&self, entry: RegisterEntry) -> Result<(), RegisterError> {
        self.0.write().await.push(entry);

        Ok(())
    }
}