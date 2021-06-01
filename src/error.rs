use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Handle was invalid")]
    Handle,
    #[error(transparent)]
    Os(#[from] std::io::Error),
    #[error("Error converting C string to a rust &str")]
    StringConversion(#[from] std::str::Utf8Error),
}
