use async_trait::async_trait;
use crate::domain::app::App;
use crate::domain::register::Register;
use crate::ports::clients::discord::client::DiscordClient;

pub mod discord;



#[async_trait]
pub trait Client {
    async fn run(&mut self);
}


pub async fn init_client<R: Register + Send + Sync + 'static>(app: App<R>) -> impl Client {
    DiscordClient::new(app).await
}