#[cfg(not(target_os = "windows"))]
compile_error!("Currently only windows is supported");

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(any(target_os = "windows", feature = "internal"))]
pub mod internal;

#[cfg(any(target_os = "windows", feature = "external"))]
pub mod external;

pub mod mem;
