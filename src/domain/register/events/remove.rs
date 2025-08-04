use crate::domain::app::App;
use crate::domain::register::Register;
use async_trait::async_trait;

pub struct RemoveEntry {
    pub user_id: String,
    pub bot_id: String,
}

#[async_trait]
pub trait RemoveEvent {
    fn entry(&self) -> RemoveEntry;
    async fn failed_message(&self);
    async fn success_message(&self);
}

impl<R> App<R>
where
    R: Register,
{
    pub async fn remove_from_register<E: RemoveEvent>(&self, event: E) {
        if let Err(why) = self.register.remove(event.entry()).await {
            log::error!("Error while removing event: {why:?}");
            event.failed_message().await;
        } else {
            log::info!("Successfully removed entry");
            event.success_message().await;
        }
    }
}
