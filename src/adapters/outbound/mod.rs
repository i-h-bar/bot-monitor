use crate::ports::outbound::register::Register;
use register::dynamodb::DynamoDB;

pub mod register;

pub async fn init_register() -> impl Register {
    DynamoDB::new().await
}
