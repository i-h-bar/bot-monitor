use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, derive(Clone))]
pub enum BotStates {
    Offline,
    Online,
    NA,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait StatusEvent {
    fn bot_id(&self) -> String;
    fn state(&self) -> BotStates;
    async fn is_bot(&self) -> bool;
    async fn send_offline_warning(&self, entries: Vec<RegisterEntry>);
    async fn send_online_message(&self, entries: Vec<RegisterEntry>);
}

impl<R> App<R>
where
    R: Register,
{
    pub async fn resolve_event<E: StatusEvent>(&self, event: E) {
        if !event.is_bot().await {
            return;
        }

        if let Some(entries) = self.register.fetch(event.bot_id()).await {
            match event.state() {
                BotStates::Offline => {
                    log::info!("A bot went offline!");
                    event.send_offline_warning(entries).await;
                }
                BotStates::Online => {
                    log::info!("A bot came back online!");
                    event.send_online_message(entries).await;
                }
                BotStates::NA => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::register::MockRegister;

    #[tokio::test]
    async fn test_resolve_not_bot() {
        let bot_id = String::from("bot_id_12345");

        let mut register = MockRegister::new();
        register.expect_fetch().times(0).return_const(None);

        let mut status_event = MockStatusEvent::new();
        status_event
            .expect_bot_id()
            .times(0)
            .return_const(bot_id.clone());
        status_event.expect_is_bot().times(1).return_const(false);
        status_event
            .expect_state()
            .times(0)
            .return_const(BotStates::Offline);
        status_event
            .expect_send_offline_warning()
            .times(0)
            .return_const(());
        status_event
            .expect_send_online_message()
            .times(0)
            .return_const(());

        let app = App::new(register);

        app.resolve_event(status_event).await;
    }

    #[tokio::test]
    async fn test_resolve_event_offline() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let entry = RegisterEntry {
            bot_id: bot_id.clone(),
            user_id: user_id.clone(),
        };
        let entries = vec![entry];

        let mut register = MockRegister::new();
        register
            .expect_fetch()
            .times(1)
            .with(eq(bot_id.clone()))
            .return_const(Some(entries.clone()));

        let mut status_event = MockStatusEvent::new();
        status_event
            .expect_bot_id()
            .times(1)
            .return_const(bot_id.clone());
        status_event.expect_is_bot().times(1).return_const(true);
        status_event
            .expect_state()
            .times(1)
            .return_const(BotStates::Offline);
        status_event
            .expect_send_offline_warning()
            .times(1)
            .with(eq(entries))
            .return_const(());
        status_event
            .expect_send_online_message()
            .times(0)
            .return_const(());

        let app = App::new(register);

        app.resolve_event(status_event).await;
    }

    #[tokio::test]
    async fn test_resolve_event_online() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let entry = RegisterEntry {
            bot_id: bot_id.clone(),
            user_id: user_id.clone(),
        };
        let entries = vec![entry];

        let mut register = MockRegister::new();
        register
            .expect_fetch()
            .times(1)
            .with(eq(bot_id.clone()))
            .return_const(Some(entries.clone()));

        let mut status_event = MockStatusEvent::new();
        status_event
            .expect_bot_id()
            .times(1)
            .return_const(bot_id.clone());
        status_event.expect_is_bot().times(1).return_const(true);
        status_event
            .expect_state()
            .times(1)
            .return_const(BotStates::Online);
        status_event
            .expect_send_offline_warning()
            .times(0)
            .return_const(());
        status_event
            .expect_send_online_message()
            .times(1)
            .with(eq(entries))
            .return_const(());

        let app = App::new(register);

        app.resolve_event(status_event).await;
    }

    #[tokio::test]
    async fn test_resolve_event_na() {
        let bot_id = String::from("bot_id_12345");
        let user_id = String::from("user_id_12345");
        let entry = RegisterEntry {
            bot_id: bot_id.clone(),
            user_id: user_id.clone(),
        };
        let entries = vec![entry];

        let mut register = MockRegister::new();
        register
            .expect_fetch()
            .times(1)
            .with(eq(bot_id.clone()))
            .return_const(Some(entries.clone()));

        let mut status_event = MockStatusEvent::new();
        status_event
            .expect_bot_id()
            .times(1)
            .return_const(bot_id.clone());
        status_event.expect_is_bot().times(1).return_const(true);
        status_event
            .expect_state()
            .times(1)
            .return_const(BotStates::NA);
        status_event
            .expect_send_offline_warning()
            .times(0)
            .return_const(());
        status_event
            .expect_send_online_message()
            .times(0)
            .return_const(());

        let app = App::new(register);

        app.resolve_event(status_event).await;
    }
}
