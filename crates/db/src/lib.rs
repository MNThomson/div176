pub trait Database {
    fn healthcheck(self) -> Result<(), ()>;
}

#[derive(Clone)]
pub struct DB {}

impl Database for DB {
    fn healthcheck(self) -> Result<(), ()> {
        Ok(())
    }
}
