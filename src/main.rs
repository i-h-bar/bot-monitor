use crate::ports::clients::Client;
use dotenv::dotenv;
use crate::domain::app::App;
use crate::ports::clients::init_client;

mod adapters;
mod domain;
pub mod ports;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = App;
    let mut client = init_client(app).await;

    client.run().await;
}
