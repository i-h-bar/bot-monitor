use crate::domain::register::Register;
use crate::domain::status_event::{EventStatus, StatusEvent};

pub struct App<R>
where
    R: Register,
{
    pub register: R,
}

impl<R> App<R>
where
    R: Register,
{
    pub fn new(register: R) -> Self {
        Self { register }
    }

    pub async fn resolve_event<E: StatusEvent>(&self, event: E) {
        if !event.is_bot().await {
            return;
        }

        if let Some(entries) = self.fetch_from_register(event.bot_id().to_string()).await {
            match event.status() {
                EventStatus::Offline => {
                    log::info!("A bot went offline!");
                    event.send_offline_warning(entries).await
                }
                EventStatus::Online => {
                    log::info!("A bot came back online!");
                    event.send_online_message(entries).await
                }
                _ => {}
            }
        }
    }
}
