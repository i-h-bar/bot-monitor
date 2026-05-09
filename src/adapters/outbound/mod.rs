use crate::ports::outbound::register::Register;

pub mod register;

#[cfg(all(feature = "dynamodb", feature = "postgres"))]
compile_error!("features 'dynamodb' and 'postgres' are mutually exclusive; use --no-default-features to switch backends");

#[cfg(not(any(feature = "dynamodb", feature = "postgres")))]
compile_error!("one of the features 'dynamodb' or 'postgres' must be enabled");

#[cfg(feature = "dynamodb")]
pub async fn init_register() -> impl Register {
    register::dynamodb::DynamoDB::new().await
}

#[cfg(feature = "postgres")]
pub async fn init_register() -> impl Register {
    register::postgres::Postgres::new().await
}
