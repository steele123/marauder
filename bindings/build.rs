fn main() {
    windows::build!(
        Windows::Win32::SystemServices::{OpenProcess, HANDLE, GetModuleHandleW, BOOL, FreeLibraryAndExitThread, DisableThreadLibraryCalls, GetCurrentProcess, NonClosableHandle},
        Windows::Win32::Debug::{ReadProcessMemory, WriteProcessMemory},
        Windows::Win32::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next, MODULEENTRY32, Module32First, Module32Next, },
        Windows::Win32::WindowsProgramming::CloseHandle
    );
}
