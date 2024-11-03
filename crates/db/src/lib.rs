use unreachable_macro::with_unreachable_defaults;

#[with_unreachable_defaults]
pub trait Database {
    fn healthcheck(&self) -> Result<(), ()>;
}

#[derive(Clone)]
pub struct DB {}

impl Database for DB {
    fn healthcheck(&self) -> Result<(), ()> {
        Ok(())
    }
}
