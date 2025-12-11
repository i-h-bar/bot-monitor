use crate::domain::app::App;
use crate::ports::inbound::client::events::help::HelpEvent;
use crate::ports::outbound::register::Register;

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
    use crate::domain::app::App;
    use crate::ports::inbound::client::events::help::MockHelpEvent;
    use crate::ports::outbound::register::MockRegister;

    #[tokio::test]
    async fn test_help_message() {
        let mut event = MockHelpEvent::new();
        event.expect_send_message().times(1).return_const(());

        let register = MockRegister::new();

        let app = App::new(register);

        app.send_help_message(event).await;
    }
}
