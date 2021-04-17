#[cfg(not(target_os = "windows"))]
fn panic() {
    compile_error!("Currently only windows is supported");
}

#[macro_use]
#[cfg(target_os = "windows")]
pub mod internal;

#[cfg(target_os = "windows")]
pub mod external;