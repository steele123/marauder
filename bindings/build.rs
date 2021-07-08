fn main() {
    windows::build!(
        Windows::Win32::System::Threading::{OpenProcess, GetCurrentProcess, PROCESS_ACCESS_RIGHTS,
            CreateThread, THREAD_CREATION_FLAGS, CreateRemoteThread, WaitForSingleObject, PROCESS_ACCESS_RIGHTS,
            PROCESS_ALL_ACCESS},

        Windows::Win32::System::SystemServices::{HANDLE, FALSE, GetModuleHandleW, BOOL, FreeLibraryAndExitThread,
            DisableThreadLibraryCalls, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, SECURITY_ATTRIBUTES,
            LPTHREAD_START_ROUTINE, INVALID_HANDLE_VALUE, PAGE_TYPE, MEMORY_BASIC_INFORMATION, GetModuleHandleA,
            CHAR, PAGE_EXECUTE_READWRITE, PAGE_READWRITE, GetProcAddress, DLL_THREAD_ATTACH, DLL_THREAD_DETACH},

        Windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},

        Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS,
            Process32First, Process32Next, MODULEENTRY32, Module32First, Module32Next, CREATE_TOOLHELP_SNAPSHOT_FLAGS,
            TH32CS_SNAPPROCESS, TH32CS_SNAPMODULE},

        Windows::Win32::System::WindowsProgramming::{CloseHandle, INFINITE},

        Windows::Win32::UI::KeyboardAndMouseInput::GetAsyncKeyState,

        Windows::Win32::System::Memory::{VirtualProtect, VirtualProtectEx, VirtualQueryEx, VirtualAllocEx,
            MEM_RESERVE, MEM_COMMIT, VirtualFreeEx, MEM_RELEASE}
    );
}
