use crate::adapters::register::dynamodb::DynamoDB;
use crate::domain::register::Register;

mod dynamodb;
mod local;

pub async fn init_register() -> impl Register {
    DynamoDB::new().await
}
