use crate::domain::app::App;
use crate::ports::inbound::client::events::remove::RemoveEvent;
use crate::ports::outbound::register::Register;

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
    use crate::domain::app::App;
    use crate::ports::inbound::client::events::remove::{MockRemoveEvent, RemoveEntry};
    use crate::ports::outbound::register::{MockRegister, RegisterError};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_remove_from_register() {
        let user_id = String::from("user_id_12345");
        let bot_id = String::from("bot_id12345");

        let entry = RemoveEntry { user_id, bot_id };

        let mut register = MockRegister::new();
        register
            .expect_remove()
            .times(1)
            .with(eq(entry.clone()))
            .return_const(Ok(()));

        let mut event = MockRemoveEvent::new();
        event.expect_entry().times(1).return_const(entry.clone());
        event.expect_success_message().times(1).return_const(());
        event.expect_failed_message().times(0).return_const(());

        let app = App::new(register);
        app.remove_from_register(event).await;
    }

    #[tokio::test]
    async fn test_remove_from_register_error() {
        let user_id = String::from("user_id_12345");
        let bot_id = String::from("bot_id12345");

        let entry = RemoveEntry { user_id, bot_id };

        let mut register = MockRegister::new();
        register
            .expect_remove()
            .times(1)
            .with(eq(entry.clone()))
            .return_const(Err(RegisterError::EntryRemoveError));

        let mut event = MockRemoveEvent::new();
        event.expect_entry().times(1).return_const(entry.clone());
        event.expect_success_message().times(0).return_const(());
        event.expect_failed_message().times(1).return_const(());

        let app = App::new(register);
        app.remove_from_register(event).await;
    }
}
