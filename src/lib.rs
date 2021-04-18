#[cfg(not(target_os = "windows"))]
compile_error!("Currently only windows is supported");

#[cfg(all(feature = "external", feature = "internal"))]
compile_error!("you may not compile with both features 'external' and 'internal'");

#[macro_use]
#[cfg(any(target_os = "windows", feature = "internal"))]
pub mod internal;

#[cfg(any(target_os = "windows", feature = "external"))]
pub mod external;

#[cfg(target_os = "windows")]
pub mod windows;

pub mod process;
