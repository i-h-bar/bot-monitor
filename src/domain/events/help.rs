use crate::domain::app::App;
use crate::domain::register::Register;
use async_trait::async_trait;

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
