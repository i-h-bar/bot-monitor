use crate::domain::app::App;
use crate::ports::inbound::client::events::list::ListEvent;
use crate::ports::outbound::register::Register;

impl<R> App<R>
where
    R: Register,
{
    pub async fn list_entries<L: ListEvent>(&self, event: L) {
        let entries = match self.register.list(event.payload()).await {
            Ok(entries) => entries,
            Err(why) => {
                log::warn!("Failed to list entries in register: {why:?}");
                event.failed_message().await;
                return;
            }
        };

        if entries.is_empty() {
            event.empty_message().await;
        } else {
            event.success_message(entries).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::app::App;
    use crate::ports::inbound::client::events::list::{ListEntriesPayload, MockListEvent};
    use crate::ports::outbound::register::{MockRegister, RegisterEntry, RegisterError};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_list_entries_error() {
        let user_id = String::from("user_id_12345");
        let payload = ListEntriesPayload { user_id };

        let mut register = MockRegister::new();
        register
            .expect_list()
            .times(1)
            .return_const(Err(RegisterError::EntryFetchError));

        let mut event = MockListEvent::new();
        event.expect_failed_message().times(1).return_const(());
        event.expect_success_message().times(0).return_const(());
        event.expect_empty_message().times(0).return_const(());
        event.expect_payload().times(1).return_const(payload);

        let app = App::new(register);

        app.list_entries(event).await;
    }

    #[tokio::test]
    async fn test_list_entries_empty() {
        let user_id = String::from("user_id_12345");
        let payload = ListEntriesPayload { user_id };

        let mut register = MockRegister::new();
        register.expect_list().times(1).return_const(Ok(Vec::new()));

        let mut event = MockListEvent::new();
        event.expect_failed_message().times(0).return_const(());
        event.expect_success_message().times(0).return_const(());
        event.expect_empty_message().times(1).return_const(());
        event.expect_payload().times(1).return_const(payload);

        let app = App::new(register);

        app.list_entries(event).await;
    }

    #[tokio::test]
    async fn test_list_entries_one_result() {
        let user_id = String::from("user_id_12345");
        let bot_id = String::from("bot_id_12345");
        let payload = ListEntriesPayload {
            user_id: user_id.clone(),
        };
        let entry = RegisterEntry {
            user_id: user_id.clone(),
            bot_id: bot_id.clone(),
        };
        let entries = vec![entry.clone()];

        let mut register = MockRegister::new();
        register
            .expect_list()
            .times(1)
            .return_const(Ok(entries.clone()));

        let mut event = MockListEvent::new();
        event.expect_failed_message().times(0).return_const(());
        event
            .expect_success_message()
            .times(1)
            .with(eq(entries.clone()))
            .return_const(());
        event.expect_empty_message().times(0).return_const(());
        event.expect_payload().times(1).return_const(payload);

        let app = App::new(register);

        app.list_entries(event).await;
    }

    #[tokio::test]
    async fn test_list_entries_multiple_result() {
        let user_id = String::from("user_id_12345");
        let bot_id = String::from("bot_id_12345");
        let payload = ListEntriesPayload {
            user_id: user_id.clone(),
        };
        let entry = RegisterEntry {
            user_id: user_id.clone(),
            bot_id: bot_id.clone(),
        };
        let entries = vec![entry.clone(), entry.clone(), entry.clone()];

        let mut register = MockRegister::new();
        register
            .expect_list()
            .times(1)
            .return_const(Ok(entries.clone()));

        let mut event = MockListEvent::new();
        event.expect_failed_message().times(0).return_const(());
        event
            .expect_success_message()
            .times(1)
            .with(eq(entries.clone()))
            .return_const(());
        event.expect_empty_message().times(0).return_const(());
        event.expect_payload().times(1).return_const(payload);

        let app = App::new(register);

        app.list_entries(event).await;
    }
}
