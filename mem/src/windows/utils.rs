use std::{ffi::CStr, mem::size_of};

use bindings::Windows::Win32::{
    Foundation::INVALID_HANDLE_VALUE,
    System::{
        Diagnostics::ToolHelp::{MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS},
        SystemServices::CHAR,
    },
};

use crate::{
    error::Error,
    windows::wrappers::{
        close_handle, create_tool_help32_snapshot, module32_first, module32_next, process32_first, process32_next, DWORD,
        DWORD_PTR,
    },
};

pub fn convert_windows_string<'a, const N: usize>(string: [CHAR; N]) -> Result<&'a str, Error> {
    unsafe { Ok(CStr::from_ptr(string.as_ptr() as *const i8).to_str()?) }
}

pub fn get_process_id(process_name: &str) -> Result<DWORD, Error> {
    let mut process_id: DWORD = 0;

    let snapshot = create_tool_help32_snapshot(TH32CS_SNAPPROCESS, process_id);

    if snapshot == INVALID_HANDLE_VALUE {
        return Err(Error::Handle);
    }

    let mut entry = PROCESSENTRY32 {
        dwSize: size_of::<PROCESSENTRY32>() as u32,
        ..Default::default()
    };

    if process32_first(snapshot, &mut entry) {
        process_id = loop {
            let current_name = convert_windows_string(entry.szExeFile)?;

            if current_name == process_name {
                break entry.th32ProcessID;
            }

            if !process32_next(snapshot, &mut entry) {
                break 0;
            }
        };

        close_handle(snapshot);
    }

    if process_id == 0 {
        return Err(Error::ProcessNotFound);
    }

    Ok(process_id)
}

pub fn get_module_base(process_id: DWORD, module_name: &str) -> Result<DWORD_PTR, Error> {
    let mut module_base_address: DWORD_PTR = 0x0;

    let snapshot = create_tool_help32_snapshot(TH32CS_SNAPMODULE, process_id);

    let mut entry = MODULEENTRY32 {
        dwSize: size_of::<MODULEENTRY32>() as u32,
        ..Default::default()
    };

    if module32_first(snapshot, &mut entry) {
        module_base_address = loop {
            let current_name = convert_windows_string(entry.szModule)?;

            if current_name == module_name {
                break entry.modBaseAddr as DWORD_PTR;
            }

            if module32_next(snapshot, &mut entry) {
                break 0;
            }
        };

        close_handle(snapshot);
    }

    if module_base_address == 0 {
        return Err(std::io::Error::last_os_error().into());
    }

    Ok(module_base_address)
}
