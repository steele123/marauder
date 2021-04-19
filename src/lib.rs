#[cfg(not(target_os = "windows"))]
compile_error!("Currently only windows is supported");

#[cfg(target_os = "windows")]
pub mod windows;

#[macro_use]
#[cfg(any(feature = "internal"))]
pub mod internal;

pub mod external;
