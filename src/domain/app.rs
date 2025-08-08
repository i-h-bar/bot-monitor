use crate::domain::events::status::{BotStates, StatusEvent};
use crate::domain::register::Register;

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
}
