use crate::windows::wrappers::{ptr, DWORD_PTR, LPVOID};
use anyhow::Result;
use bindings::Windows::Win32::SystemServices::{HANDLE, LPTHREAD_START_ROUTINE};
use std::ffi::c_void;

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

pub trait MemFns {
    fn new<T>(process_name: &str) -> Result<T>;

    fn write_value<T>(&self, pointer: ptr, output: T, relative: bool) -> bool;

    fn read_value<T>(&self, pointer: ptr, relative: bool) -> T;

    fn nop(&self, address: *mut c_void, size: usize);

    fn call_function(&self, function: LPTHREAD_START_ROUTINE) -> Result<()>;

    fn patch(&self, address: *mut c_void, base: LPVOID, size: usize);
}
