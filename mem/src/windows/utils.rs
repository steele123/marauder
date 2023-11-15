use std::{ffi::CStr, mem::size_of};

use windows::Win32::{
    Foundation::CHAR,
    System::Diagnostics::ToolHelp::{TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS},
};

use crate::{
    error::Error,
    windows::wrappers::{
        close_handle, create_tool_help32_snapshot, module32_first, module32_next, process32_first, process32_next,
        ModuleEntry32, ProcessEntry32, DWORD, DWORD_PTR,
    },
};

/// This will convert a pointer to a string into a string.
/// # Errors
/// `std::ffi::Error` if an error occurs.
pub fn convert_windows_string<'a, const N: usize>(string: [CHAR; N]) -> Result<&'a str, Error> {
    unsafe { Ok(CStr::from_ptr(string.as_ptr().cast::<i8>()).to_str()?) }
}

/// `get_process_id` returns the ID of the process name.
/// # Errors
/// `error::Error` if an error occurs.
pub fn get_process_id(process_name: &str) -> Result<DWORD, Error> {
    let mut process_id: DWORD = 0;

    let snapshot = create_tool_help32_snapshot(TH32CS_SNAPPROCESS, process_id)?;

    let mut entry = ProcessEntry32 {
        dwSize: size_of::<ProcessEntry32>() as u32,
        ..ProcessEntry32::default()
    };

    if process32_first(snapshot, &mut entry).is_ok() {
        process_id = loop {
            let current_name = convert_windows_string(entry.szExeFile)?;

            if current_name == process_name {
                break entry.th32ProcessID;
            }

            if process32_next(snapshot, &mut entry).is_err() {
                break 0;
            }
        };

        close_handle(snapshot)?;
    }

    if process_id == 0 {
        return Err(Error::ProcessNotFound);
    }

    Ok(process_id)
}

/// `get_module_base` returns the base address of the module name.
/// # Errors
/// `error::Error` if an error occurs.
pub fn get_module_base(process_id: DWORD, module_name: &str) -> Result<DWORD_PTR, Error> {
    let mut module_base_address: DWORD_PTR = 0x0;

    let snapshot = create_tool_help32_snapshot(TH32CS_SNAPMODULE, process_id)?;

    let mut entry = ModuleEntry32 {
        dwSize: size_of::<ModuleEntry32>() as u32,
        ..ModuleEntry32::default()
    };

    if module32_first(snapshot, &mut entry).is_ok() {
        module_base_address = loop {
            let current_name = convert_windows_string(entry.szModule)?;

            if current_name == module_name {
                break entry.modBaseAddr as DWORD_PTR;
            }

            if module32_next(snapshot, &mut entry).is_ok() {
                break 0;
            }
        };

        close_handle(snapshot)?;
    }

    if module_base_address == 0 {
        return Err(std::io::Error::last_os_error().into());
    }

    Ok(module_base_address)
}
