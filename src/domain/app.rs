use crate::ports::outbound::register::Register;

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
