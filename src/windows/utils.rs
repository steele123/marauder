use crate::windows::wrappers::{create_tool_help32_snapshot, DWORD, DWORD_PTR};
use bindings::Windows::Win32::ToolHelp::{CREATE_TOOLHELP_SNAPSHOT_FLAGS, PROCESSENTRY32};
use std::fmt::Error;

pub fn get_process_id(process_name: &str) -> Result<DWORD, Error> {
    let mut process_id: DWORD = 0;

    let snapshot = create_tool_help32_snapshot(
        CREATE_TOOLHELP_SNAPSHOT_FLAGS::TH32CS_SNAPMODULE,
        process_id,
    );

    let mut entry = PROCESSENTRY32::default();

    Ok(process_id)
}

pub fn get_module_base(process_id: DWORD, module_name: &str) -> Result<DWORD_PTR, Error> {
    let mut module_base_address: DWORD_PTR = 0x0;

    let snapshot = create_tool_help32_snapshot(
        CREATE_TOOLHELP_SNAPSHOT_FLAGS::TH32CS_SNAPMODULE,
        process_id,
    );

    Ok(module_base_address)
}
