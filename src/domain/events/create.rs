use crate::domain::app::App;
use crate::domain::register::Register;
use async_trait::async_trait;

pub struct CreateEntry {
    pub user_id: String,
    pub bot_id: String,
    pub version: usize,
}

#[async_trait]
pub trait CreateEntryEvent {
    fn entry(&self) -> CreateEntry;
    fn is_bot(&self) -> bool;
    async fn not_a_bot_message(&self);
    async fn entry_added_message(&self);
    async fn failed_message(&self);
}

impl<R> App<R>
where
    R: Register,
{
    pub async fn add_to_register<E: CreateEntryEvent>(&self, event: E) {
        if !event.is_bot() {
            event.not_a_bot_message().await;
            return;
        }

        if let Err(why) = self.register.add(event.entry()).await {
            log::warn!("Failed to add new entry - {why:?}");
            event.failed_message().await;
        } else {
            log::info!("Added new entry");
            event.entry_added_message().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::register::{Register, RegisterEntry, RegisterError};
    use async_trait::async_trait;
    use tokio::sync::RwLock;
    use crate::domain::events::list::ListEntriesPayload;
    use crate::domain::events::remove::RemoveEntry;

    pub struct LocalRegister(RwLock<Vec<RegisterEntry>>);

    impl LocalRegister {
        pub fn new() -> Self {
            Self(RwLock::new(Vec::new()))
        }
    }

    #[async_trait]
    impl Register for LocalRegister {
        async fn fetch(&self, bot_id: String) -> Option<Vec<RegisterEntry>> {
            for entry in self.0.read().await.iter() {
                if entry.bot_id == bot_id {
                    return Some(vec![entry.clone()]);
                }
            }

            None
        }

        async fn add(&self, entry: CreateEntry) -> Result<(), RegisterError> {
            self.0.write().await.push(RegisterEntry {
                bot_id: entry.bot_id,
                user_id: entry.user_id,
            });
            log::info!("{:?}", self.0.read().await);
            Ok(())
        }

        async fn remove(&self, entry: RemoveEntry) -> Result<(), RegisterError> {
            let mut index: Option<usize> = None;
            for (i, entry) in self.0.read().await.iter().enumerate() {
                if entry.bot_id == entry.bot_id && entry.user_id == entry.user_id {
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                self.0.write().await.remove(i);
            }
            log::info!("{:?}", self.0.read().await);
            Ok(())
        }

        async fn list(&self, entry: ListEntriesPayload) -> Result<Vec<RegisterEntry>, RegisterError> {
            Ok(self
                .0
                .read()
                .await
                .iter()
                .filter_map(|entry| {
                    if entry.user_id == entry.user_id {
                        Some(entry.clone())
                    } else {
                        None
                    }
                })
                .collect())
        }
    }

    #[tokio::test]
    async fn test_add_to_register() {
        let local = LocalRegister::new();
        let app = App::new(local);
        // app.add_to_register(CreateEntry {
        //     bot_id: String::new(),
        //     user_id: String::new(),
        //     entry_version: 0,
        // })
        // .await
        // .unwrap();
    }
}