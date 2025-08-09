use crate::domain::app::App;
use crate::domain::register::Register;
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;


#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct CreateEntry {
    pub user_id: String,
    pub bot_id: String,
    pub version: usize,
}

#[cfg_attr(test, automock)]
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
    use crate::domain::register::{MockRegister, RegisterError};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_add_to_register() {
        let user_id = String::from("user_id_12345");
        let bot_id = String::from("bot_id_12345");

        let entry = CreateEntry { user_id, bot_id, version: 1 };

        let mut register = MockRegister::new();
        register.expect_add().times(1).with(eq(entry.clone())).return_const(Ok(()));

        let mut event = MockCreateEntryEvent::new();
        event.expect_is_bot().times(1).return_const(true);
        event.expect_entry().times(1).return_const(entry.clone());
        event.expect_entry_added_message().times(1).return_const(());

        let app = App::new(register);

        app.add_to_register(event).await;
    }

    #[tokio::test]
    async fn test_add_to_register_error() {
        let user_id = String::from("user_id_12345");
        let bot_id = String::from("bot_id_12345");

        let entry = CreateEntry { user_id, bot_id, version: 1 };

        let mut register = MockRegister::new();
        register.expect_add().times(1).with(eq(entry.clone())).return_const(Err(RegisterError::EntryCreationError));

        let mut event = MockCreateEntryEvent::new();
        event.expect_is_bot().times(1).return_const(true);
        event.expect_entry().times(1).return_const(entry.clone());
        event.expect_failed_message().times(1).return_const(());

        let app = App::new(register);

        app.add_to_register(event).await;
    }

    #[tokio::test]
    async fn test_add_to_register_not_a_bot() {
        let mut register = MockRegister::new();
        register.expect_add().times(0).return_const(Ok(()));

        let mut event = MockCreateEntryEvent::new();
        event.expect_is_bot().times(1).return_const(false);
        event.expect_not_a_bot_message().times(1).return_const(());

        let app = App::new(register);

        app.add_to_register(event).await;
    }
}
