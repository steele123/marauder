use std::os::raw::*;

use bindings::Windows::Win32::Debug::{ReadProcessMemory, WriteProcessMemory};
use bindings::Windows::Win32::SystemServices::{
    GetModuleHandleW, OpenProcess, BOOL, HANDLE, PROCESS_ACCESS_RIGHTS, PWSTR, TRUE,
};
use bindings::Windows::Win32::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, CREATE_TOOLHELP_SNAPSHOT_FLAGS,
    PROCESSENTRY32,
};

pub type DWORD = c_ulong;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type SIZE_T = usize;
pub type WCHAR = u16;
pub type LPCWSTR = WCHAR;
pub type HMODULE = isize;

pub fn get_module_handle(module_name: PWSTR) -> HMODULE {
    unsafe { GetModuleHandleW(module_name) }
}

pub fn open_process(
    desired_access: PROCESS_ACCESS_RIGHTS,
    inherit_handle: BOOL,
    process_id: DWORD,
) -> HANDLE {
    unsafe { OpenProcess(desired_access, inherit_handle, process_id) }
}

pub fn create_tool_help32_snapshot(
    flags: CREATE_TOOLHELP_SNAPSHOT_FLAGS,
    process_id: DWORD,
) -> HANDLE {
    unsafe { CreateToolhelp32Snapshot(flags, process_id) }
}

pub fn process32_first(snapshot: HANDLE, process_entry: &mut PROCESSENTRY32) -> bool {
    unsafe {
        let ret = Process32First(snapshot, process_entry);
        ret == TRUE
    }
}

pub fn process32_next(snapshot: HANDLE, process_entry: &mut PROCESSENTRY32) -> bool {
    unsafe {
        let ret = Process32Next(snapshot, process_entry);
        ret == TRUE
    }
}

pub fn write_process_memory(
    process_handle: HANDLE,
    base_address: LPVOID,
    buffer: LPCVOID,
    size: SIZE_T,
    number_of_bytes_written: *mut SIZE_T,
) -> bool {
    unsafe {
        let ret = WriteProcessMemory(
            process_handle,
            base_address,
            buffer,
            size,
            number_of_bytes_written,
        );
        ret == TRUE
    }
}

pub fn read_process_memory(
    process_handle: HANDLE,
    base_address: LPCVOID,
    buffer: LPVOID,
    size: SIZE_T,
    number_of_bytes_written: *mut SIZE_T,
) -> bool {
    unsafe {
        let ret = ReadProcessMemory(
            process_handle,
            base_address,
            buffer,
            size,
            number_of_bytes_written,
        );
        ret == TRUE
    }
}
