use crate::adapters::register::init_register;
use crate::domain::app::App;
use crate::ports::clients::Client;
use crate::ports::clients::init_client;
use dotenv::dotenv;

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
