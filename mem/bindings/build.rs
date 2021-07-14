fn main() {
    windows::build!(
        Windows::Win32::Foundation::{INVALID_HANDLE_VALUE, CloseHandle},

        Windows::Win32::System::Threading::{OpenProcess, GetCurrentProcess, PROCESS_ACCESS_RIGHTS,CreateThread, THREAD_CREATION_FLAGS, CreateRemoteThread, WaitForSingleObject, PROCESS_ACCESS_RIGHTS},

        Windows::Win32::System::LibraryLoader::{GetModuleHandleW, FreeLibraryAndExitThread, DisableThreadLibraryCalls, GetModuleHandleA, GetProcAddress},

        Windows::Win32::System::Console::{AllocConsole, FreeConsole},

        Windows::Win32::Graphics::Direct3D9::{D3D_SDK_VERSION, Direct3DCreate9, D3DPRESENT_PARAMETERS, IDirect3D9},

        Windows::Win32::UI::WindowsAndMessaging::{WNDCLASSEXW, CreateWindowExW, DestroyWindow, UnregisterClassW},

        Windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, LPTHREAD_START_ROUTINE, CHAR, DLL_THREAD_ATTACH, DLL_THREAD_DETACH, CHAR},

        Windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory, GetLastError},

        Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next, MODULEENTRY32, Module32First, Module32Next, CREATE_TOOLHELP_SNAPSHOT_FLAGS},

        Windows::Win32::System::WindowsProgramming::{INFINITE},

        Windows::Win32::UI::KeyboardAndMouseInput::GetAsyncKeyState,

        Windows::Win32::System::Memory::{VirtualProtect, VirtualProtectEx, VirtualQueryEx, VirtualAllocEx, VirtualFreeEx},
    );
}
