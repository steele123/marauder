use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Handle was invalid")]
    Handle,
    #[error(transparent)]
    Os(#[from] std::io::Error),
    #[error("Error converting C string to a rust &str")]
    StringConversion(#[from] std::str::Utf8Error),
    #[error("Process not found")]
    ProcessNotFound,
    #[error(transparent)]
    NulError(#[from] std::ffi::NulError),
    #[error("Couldn't write memory")]
    MemoryWrite,
    #[error("Couldn't find function in the process")]
    ProcessAddress,
    #[error("DLL path doesn't exist")]
    DllPath,
}
