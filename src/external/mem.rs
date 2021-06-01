use crate::error::Error;
use crate::windows::utils::{get_module_base, get_process_id};
use crate::windows::wrappers::{
    close_handle, create_remote_thread, open_process, ptr, read_process_memory, size_t,
    virtual_protect, virtual_protect_ex, virtual_query_ex, wait_for_single_object,
    write_process_memory, DWORD, DWORD_PTR, LPCVOID, LPVOID,
};
use bindings::Windows::Win32::SystemServices::{
    FALSE, HANDLE, INVALID_HANDLE_VALUE, LPTHREAD_START_ROUTINE, MEMORY_BASIC_INFORMATION,
    PAGE_TYPE, PROCESS_ACCESS_RIGHTS, SECURITY_ATTRIBUTES,
};
use bindings::Windows::Win32::WindowsProgramming::INFINITE;
use std::ffi::c_void;

pub struct Mem {
    pub process: HANDLE,
    pub module_base_address: DWORD_PTR,
}

#[cfg(feature = "external")]
impl Mem {
    pub fn new(process_name: &str) -> Result<Self, Error> {
        let process_id = get_process_id(process_name)?;
        let module_base_address = get_module_base(process_id, process_name)?;

        let process = open_process(PROCESS_ACCESS_RIGHTS::PROCESS_ALL_ACCESS, FALSE, process_id);

        if process.is_null() {
            return Err(std::io::Error::last_os_error().into());
        }

        Ok(Self {
            process,
            module_base_address,
        })
    }

    pub fn write_value<T>(&self, pointer: ptr, output: T, relative: bool) -> bool {
        let relative_value_address = if relative {
            pointer + self.module_base_address
        } else {
            pointer
        };

        let mut bytes_written: usize = 0;

        write_process_memory(
            self.process,
            relative_value_address as LPVOID,
            (&output as *const T) as LPVOID,
            std::mem::size_of::<T>() as usize,
            &mut bytes_written,
        );

        bytes_written != 0
    }

    pub fn read_value<T>(&self, pointer: ptr, relative: bool) -> T {
        let relative_value_address = if relative {
            pointer + self.module_base_address
        } else {
            pointer
        };

        let mut buffer: T = unsafe { std::mem::zeroed() };

        read_process_memory(
            self.process,
            relative_value_address as LPCVOID,
            &mut buffer as *mut T as LPVOID,
            std::mem::size_of::<T>(),
            0 as *mut size_t,
        );

        buffer
    }

    /// Puts a NOP code at a memory address. A NOP will literally do nothing, it is intended to replace
    /// a assembly instruction to make it no longer do anything yet still allow the process to compile.
    pub fn nop(&self, address: *mut c_void, size: usize) {
        let nop_array = vec![0; size];

        unsafe {
            std::ptr::write_bytes(nop_array.as_ptr() as *mut usize, 0x90, size);
        }

        self.patch(address, nop_array.as_ptr() as *mut c_void, size);
    }

    /// Idk if this will be used at all, maybe... Essentially you just create a thread for another process
    /// then your function will be called at that threads start routine.
    pub fn call_function(&self, function: LPTHREAD_START_ROUTINE) -> Result<(), Error> {
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
            return Err(Error::Handle);
        }

        wait_for_single_object(thread_handle, INFINITE);
        close_handle(thread_handle);

        Ok(())
    }

    pub fn patch(&self, address: *mut c_void, base: LPVOID, size: usize) {
        let old_protect: *mut PAGE_TYPE = std::ptr::null_mut();

        // Changes a memory regions protection so we can write to it.
        virtual_protect_ex(
            self.process,
            address,
            size,
            PAGE_TYPE::PAGE_EXECUTE_READWRITE,
            old_protect,
        );

        write_process_memory(self.process, address, base, size, std::ptr::null_mut());

        // Cleans up other virtual protect
        virtual_protect_ex(
            self.process,
            address,
            size,
            unsafe { *old_protect },
            old_protect,
        );
    }
}
