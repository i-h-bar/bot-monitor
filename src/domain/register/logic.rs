use crate::domain::app::App;
use crate::domain::register::{Register, RegisterEntry};

impl<R> App<R> where R: Register {
    async fn add_to_register(&self, entry: RegisterEntry) {
        self.register.add(entry).await.unwrap()
    }
}