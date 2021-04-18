use std::os::raw::*;

use bindings::Windows::Win32::Debug::{ReadProcessMemory, WriteProcessMemory};
use bindings::Windows::Win32::SystemServices::{
    DisableThreadLibraryCalls, FreeLibraryAndExitThread, GetCurrentProcess, GetModuleHandleW,
    NonClosableHandle, OpenProcess, BOOL, HANDLE, PROCESS_ACCESS_RIGHTS, PWSTR,
};
use bindings::Windows::Win32::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, CREATE_TOOLHELP_SNAPSHOT_FLAGS,
    PROCESSENTRY32,
};

// size_t will be either 4 or 8 bytes depending on whether it is x64 or x86 because it is usize
// it is also a type from windows.
#[allow(non_camel_case_types)]
pub type size_t = usize;

// Windows Data Types
pub type DWORD = c_ulong;
#[allow(non_camel_case_types)]
pub type DWORD_PTR = usize;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type WCHAR = u16;
pub type LPCWSTR = WCHAR;
pub type HMODULE = isize;

pub fn get_current_process() -> NonClosableHandle {
    unsafe { GetCurrentProcess() }
}

pub fn free_library_and_exit_thread(module_handle: HMODULE, exit_code: DWORD) {
    unsafe {
        FreeLibraryAndExitThread(module_handle, exit_code);
    }
}

pub fn disable_thread_library_calls(library_module: HMODULE) -> bool {
    unsafe { DisableThreadLibraryCalls(library_module).into() }
}

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
    unsafe { Process32First(snapshot, process_entry).into() }
}

pub fn process32_next(snapshot: HANDLE, process_entry: &mut PROCESSENTRY32) -> bool {
    unsafe { Process32Next(snapshot, process_entry).into() }
}

pub fn write_process_memory(
    process_handle: HANDLE,
    base_address: LPVOID,
    buffer: LPCVOID,
    size: size_t,
    number_of_bytes_written: *mut size_t,
) -> bool {
    unsafe {
        WriteProcessMemory(
            process_handle,
            base_address,
            buffer,
            size,
            number_of_bytes_written,
        )
        .into()
    }
}

pub fn read_process_memory(
    process_handle: HANDLE,
    base_address: LPCVOID,
    buffer: LPVOID,
    size: size_t,
    number_of_bytes_written: *mut size_t,
) -> bool {
    unsafe {
        ReadProcessMemory(
            process_handle,
            base_address,
            buffer,
            size,
            number_of_bytes_written,
        )
        .into()
    }
}
