#[cfg(not(target_os = "windows"))]
compile_error!("Currently only windows is supported");

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(any(feature = "external"))]
pub mod external;
#[cfg(any(feature = "internal"))]
pub mod internal;

#[macro_use]
pub mod macros;
