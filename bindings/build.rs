fn main() {
    windows::build!(
        Windows::Win32::SystemServices::{OpenProcess, IsWow64Process, HANDLE, FALSE, GetModuleHandleW, BOOL, FreeLibraryAndExitThread, DisableThreadLibraryCalls, GetCurrentProcess, NonClosableHandle, PROCESS_ACCESS_RIGHTS, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, CreateThread, SECURITY_ATTRIBUTES, THREAD_CREATION_FLAGS, LPTHREAD_START_ROUTINE, INVALID_HANDLE_VALUE, CreateRemoteThread, WaitForSingleObject, PAGE_TYPE, VirtualProtect, VirtualProtectEx},
        Windows::Win32::Debug::{ReadProcessMemory, WriteProcessMemory},
        Windows::Win32::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next, MODULEENTRY32, Module32First, Module32Next, },
        Windows::Win32::WindowsProgramming::{CloseHandle, INFINITE}
    );
}
