use crate::adapters::register::local::LocalRegister;
use crate::domain::register::Register;

mod local;
mod dynamodb;

pub async fn init_register() -> impl Register {
    LocalRegister::new()
}
