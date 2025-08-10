use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, derive(Clone))]
pub struct ListEntriesPayload {
    pub user_id: String,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ListEvent {
    fn payload(&self) -> ListEntriesPayload;
    async fn failed_message(&self);
    async fn success_message(&self, entries: Vec<RegisterEntry>);
    async fn empty_message(&self);
}

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
    use super::*;
    use crate::domain::app::App;
    use crate::domain::register::{MockRegister, RegisterError};
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
