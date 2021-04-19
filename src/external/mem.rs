use crate::windows::utils::{get_module_base, get_process_id};
use crate::windows::wrappers::{
    close_handle, create_remote_thread, open_process, wait_for_single_object, DWORD, DWORD_PTR,
    LPVOID,
};
use bindings::Windows::Win32::SystemServices::{
    FALSE, HANDLE, INVALID_HANDLE_VALUE, LPTHREAD_START_ROUTINE, PROCESS_ACCESS_RIGHTS,
    SECURITY_ATTRIBUTES,
};
use bindings::Windows::Win32::WindowsProgramming::INFINITE;
use std::io::Error;

pub struct Mem {
    pub process: HANDLE,
    pub module_base_address: DWORD_PTR,
}

impl Mem {
    pub fn new(process_name: &str) -> Result<Mem, Error> {
        let process_id = get_process_id(process_name)?;
        let module_base_address = get_module_base(process_id, process_name)?;

        let process = open_process(PROCESS_ACCESS_RIGHTS::PROCESS_ALL_ACCESS, FALSE, process_id);

        if process.is_null() {
            return Err(Error::last_os_error());
        }

        Ok(Mem {
            process,
            module_base_address,
        })
    }

    pub fn close() {}

    pub fn call_function(&self, function: LPTHREAD_START_ROUTINE) -> Result<()> {
        let thread_handle = create_remote_thread(
            self.process,
            std::ptr::null_mut(),
            0,
            Option::from(function),
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
        );

        if thread_handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }

        wait_for_single_object(thread_handle, INFINITE);
        close_handle(thread_handle);

        Ok(())
    }
}
