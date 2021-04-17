fn main() {
    windows::build!(
        Windows::Win32::SystemServices::{OpenProcess, HANDLE, TRUE, GetModuleHandleW, BOOL},
        Windows::Win32::Debug::{ReadProcessMemory, WriteProcessMemory},
        Windows::Win32::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next},
    );
}