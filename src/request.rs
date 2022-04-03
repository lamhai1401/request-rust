use super::errors::Error;

pub trait Request {
    const URL: &'static str = "http://localhost:4000";
    fn new() -> Self
    where
        Self: Sized;
    fn get(&mut self) -> Result<(), Error>;
    fn post(&mut self, body: String) -> Result<(), Error>;
}
