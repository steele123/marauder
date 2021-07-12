use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Handle was invalid: {0}")]
    Handle(u32),
    #[error(transparent)]
    Os(#[from] std::io::Error),
    #[error("Error converting C string to a rust &str")]
    StringConversion(#[from] std::str::Utf8Error),
    #[error("Process not found")]
    ProcessNotFound,
    #[error(transparent)]
    NulError(#[from] std::ffi::NulError),
    #[error("Couldn't find function in the process: {0}")]
    ProcessAddress(u32),
    #[error("Error allocating or deallocting: {0}")]
    Allocation(u32),
    #[error("Error pertaining to memory access: {0}")]
    MemoryError(u32),
    #[error("Error pertaining to processes: {0}")]
    ProcessError(u32),
    #[error("Timeout error")]
    Timeout,
    #[error("DLL path doesn't exist")]
    DllPath,
    #[error("You must enable the feature for that render type")]
    RenderType,
    #[error("Couldn't find the graphics rendering module!")]
    RenderModule,
}
