#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Day {0} is not implemented")]
    DayNotImplemented(u8),

    #[error("Example {0} is not found")]
    ExampleNotFound(usize),
}
