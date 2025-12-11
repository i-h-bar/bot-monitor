use crate::adapters::inbound::clients::discord::client::DiscordClient;
use crate::domain::app::App;
use crate::ports::inbound::client::Client;
use crate::ports::outbound::register::Register;

pub mod discord;

pub async fn init_client<R: Register + Send + Sync + 'static>(app: App<R>) -> impl Client {
    DiscordClient::new(app).await
}
