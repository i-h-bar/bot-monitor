use crate::domain::app::App;
use crate::domain::register::Register;
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;


#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct RemoveEntry {
    pub user_id: String,
    pub bot_id: String,
}


#[cfg_attr(test, automock)]
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::app::App;
    use crate::domain::register::MockRegister;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_remove_from_register() {
        let user_id = String::from("user_id_12345");
        let bot_id = String::from("bot_id12345");

        let entry = RemoveEntry { user_id, bot_id };

        let mut register = MockRegister::new();
        register.expect_remove().times(1).with(eq(entry.clone())).return_const(Ok(()));


        let mut event = MockRemoveEvent::new();
        event.expect_entry().times(1).return_const(entry.clone());
    }
}