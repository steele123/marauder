/*
use std::ffi::c_void;

use bindings::Windows::Win32::{
    Foundation::HANDLE,
    System::{
        Memory::{PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
        SystemServices::LPTHREAD_START_ROUTINE,
        Threading::PROCESS_ALL_ACCESS,
        WindowsProgramming::INFINITE,
    },
};

use crate::{
    error::Error,
    windows::{
        utils::{get_module_base, get_process_id},
        wrappers::{
            close_handle, create_remote_thread, open_process, ptr, read_process_memory, size_t, virtual_protect_ex,
            wait_for_single_object, write_process_memory, DWORD_PTR, LPCVOID, LPVOID,
        },
    },
};

pub struct Mem {
    pub process: HANDLE,
    pub module_base_address: DWORD_PTR,
}

#[cfg(feature = "external")]
impl Mem {
    /// # Errors
    pub fn new(process_name: &str) -> Result<Self, Error> {
        let process_id = get_process_id(process_name)?;
        let module_base_address = get_module_base(process_id, process_name)?;

        let process = open_process(PROCESS_ALL_ACCESS, false.into(), process_id);

        if process.is_null() {
            return Err(std::io::Error::last_os_error().into());
        }

        Ok(Self {
            process,
            module_base_address,
        })
    }

    /// # Errors
    pub fn write_value<T>(&self, pointer: ptr, output: &T, relative: bool) -> Result<bool, Error> {
        let relative_value_address = if relative {
            pointer + self.module_base_address
        } else {
            pointer
        };

        let mut bytes_written: usize = 0;

        write_process_memory(
            self.process,
            relative_value_address as LPVOID,
            (output as *const T) as LPVOID,
            std::mem::size_of::<T>() as usize,
            &mut bytes_written,
        )?;

        Ok(bytes_written != 0)
    }

    #[must_use]
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
            (&mut buffer as *mut T).cast::<std::ffi::c_void>(),
            std::mem::size_of::<T>(),
            std::ptr::null_mut::<size_t>(),
        );

        buffer
    }

    /// Puts a NOP code at a memory address. A NOP will literally do nothing, it
    /// is intended to replace an assembly instruction to make it no longer
    /// do anything yet still allow the process to compile.
    ///
    /// # Errors
    pub fn nop(&self, address: *mut c_void, size: usize) -> Result<(), Error> {
        let nop_array: Vec<usize> = vec![0; size];

        unsafe {
            std::ptr::write_bytes(nop_array.as_ptr() as *mut usize, 0x90, size);
        }

        self.patch(address, nop_array.as_ptr() as *mut c_void, size)
    }

    /// Idk if this will be used at all, maybe... Essentially you just create a
    /// thread for another process then your function will be called at that
    /// threads start routine.
    /// # Errors
    pub fn call_function(&self, function: LPTHREAD_START_ROUTINE) -> Result<(), Error> {
        let thread_handle = create_remote_thread(
            self.process,
            std::ptr::null_mut(),
            0,
            Option::from(function),
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
        )?;

        wait_for_single_object(thread_handle, INFINITE)?;
        close_handle(thread_handle)?;

        Ok(())
    }

    /// # Errors
    pub fn patch(&self, address: *mut c_void, base: LPVOID, size: usize) -> Result<(), Error> {
        let old_protect: *mut PAGE_PROTECTION_FLAGS = std::ptr::null_mut();

        // Changes a memory regions protection, so we can write to it.
        virtual_protect_ex(self.process, address, size, PAGE_EXECUTE_READWRITE, old_protect)?;

        write_process_memory(self.process, address, base, size, std::ptr::null_mut())?;

        // Cleans up other virtual protect
        virtual_protect_ex(self.process, address, size, unsafe { *old_protect }, old_protect)
    }
}
*/
