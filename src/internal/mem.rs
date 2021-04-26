use crate::windows::utils::{get_module_base, get_process_id};
use crate::windows::wrappers::{
    close_handle, create_remote_thread, get_current_process, get_module_handle, open_process, ptr,
    read_process_memory, size_t, virtual_protect, virtual_protect_ex, virtual_query_ex,
    wait_for_single_object, write_process_memory, DWORD, DWORD_PTR, LPCVOID, LPVOID,
};
use crate::MemFns;
use anyhow::anyhow;
use anyhow::Result;
use bindings::Windows::Win32::SystemServices::{
    FALSE, HANDLE, INVALID_HANDLE_VALUE, LPTHREAD_START_ROUTINE, MEMORY_BASIC_INFORMATION,
    PAGE_TYPE, PROCESS_ACCESS_RIGHTS, SECURITY_ATTRIBUTES,
};
use bindings::Windows::Win32::WindowsProgramming::INFINITE;
use std::ffi::c_void;
use std::io::Error;

pub struct Mem {
    pub process: HANDLE,
    // module_base_address is typically not a usize but I want it to be added to pointers without casts
    pub module_base_address: DWORD_PTR,
}

#[cfg(feature = "internal")]
impl Mem {
    /*
    fn new(optional_module_name: Option<&str>) -> Result<Self> {
        let module_base_address = match optional_module_name {
            Some(module_name) => get_module_handle(module_name),
            None => get_module_handle(std::ptr::null as &str),
        } as usize;

        let process = get_current_process();

        Ok(Self {
            process,
            module_base_address,
        })
    }
    */
}
