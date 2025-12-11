use crate::domain::app::App;
use adapters::inbound::clients::init_client;
use adapters::outbound::init_register;
use dotenv::dotenv;
use ports::inbound::client::Client;

mod adapters;
mod domain;
pub mod ports;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let register = init_register().await;
    let app = App::new(register);
    let mut client = init_client(app).await;

    client.run().await;
}
