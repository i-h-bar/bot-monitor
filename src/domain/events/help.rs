use crate::domain::app::App;
use crate::domain::register::Register;
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait HelpEvent {
    async fn send_message(&self);
}

impl<R> App<R>
where
    R: Register,
{
    pub async fn send_help_message<H: HelpEvent>(&self, event: H) {
        event.send_message().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::app::App;
    use crate::domain::register::MockRegister;

    #[tokio::test]
    async fn test_help_message() {
        let mut event = MockHelpEvent::new();
        event.expect_send_message().times(1).return_const(());

        let register = MockRegister::new();

        let app = App::new(register);

        app.send_help_message(event).await;
    }
}
