use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry, RegisterError};

impl<R> App<R>
where
    R: Register,
{
    pub async fn fetch_from_register(&self, bot_id: String) -> Option<Vec<RegisterEntry>> {
        self.register.fetch(bot_id).await
    }

    pub async fn remove_from_register(
        &self,
        bot_id: String,
        user_id: String,
    ) -> Result<(), RegisterError> {
        self.register.remove(bot_id, user_id).await
    }

    pub async fn list_user_entries(
        &self,
        user_id: String,
    ) -> Result<Vec<RegisterEntry>, RegisterError> {
        self.register.list(user_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::register::{Register, RegisterEntry, RegisterError};
    use async_trait::async_trait;
    use tokio::sync::RwLock;
    use crate::domain::register::create_entry::CreateEntry;

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
            self.0.write().await.push(RegisterEntry{ bot_id: entry.bot_id, user_id: entry.user_id });
            log::info!("{:?}", self.0.read().await);
            Ok(())
        }

        async fn remove(&self, bot_id: String, user_id: String) -> Result<(), RegisterError> {
            let mut index: Option<usize> = None;
            for (i, entry) in self.0.read().await.iter().enumerate() {
                if entry.bot_id == bot_id && entry.user_id == user_id {
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                self.0.write().await.remove(i);
            }
            log::info!("{:?}", self.0.read().await);
            Ok(())
        }

        async fn list(&self, user_id: String) -> Result<Vec<RegisterEntry>, RegisterError> {
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
