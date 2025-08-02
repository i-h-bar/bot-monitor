use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry};

impl<R> App<R>
where
    R: Register,
{
    pub async fn add_to_register(&self, entry: RegisterEntry) {
        self.register.add(entry).await.unwrap()
    }

    pub async fn fetch_from_register(&self, bot_id: u64) -> Option<RegisterEntry> {
        self.register.fetch(bot_id).await
    }
}
