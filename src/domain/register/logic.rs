use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry, RegisterError};

impl<R> App<R>
where
    R: Register,
{
    pub async fn add_to_register(&self, entry: RegisterEntry) -> Result<(), RegisterError> {
        self.register.add(entry).await
    }

    pub async fn fetch_from_register(&self, bot_id: u64) -> Option<Vec<RegisterEntry>> {
        self.register.fetch(bot_id).await
    }

    pub async fn remove_from_register(
        &self,
        bot_id: u64,
        user_id: u64,
    ) -> Result<(), RegisterError> {
        self.register.remove(bot_id, user_id).await
    }

    pub async fn list_user_entries(
        &self,
        user_id: u64,
    ) -> Result<Vec<RegisterEntry>, RegisterError> {
        self.register.list(user_id).await
    }
}
