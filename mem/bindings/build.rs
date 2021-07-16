fn main() {
    windows::build!(
        Windows::Win32::Foundation::{INVALID_HANDLE_VALUE, CloseHandle},

        Windows::Win32::System::Threading::{OpenProcess, GetCurrentProcess, PROCESS_ACCESS_RIGHTS,CreateThread, THREAD_CREATION_FLAGS, CreateRemoteThread, WaitForSingleObject, PROCESS_ACCESS_RIGHTS},

        Windows::Win32::System::LibraryLoader::{GetModuleHandleW, FreeLibraryAndExitThread, DisableThreadLibraryCalls, GetModuleHandleA, GetProcAddress},

        Windows::Win32::System::Console::{AllocConsole, FreeConsole},

        // Direct X
        // D3D9
        Windows::Win32::Graphics::Direct3D9::{D3D_SDK_VERSION, Direct3DCreate9, D3DPRESENT_PARAMETERS, IDirect3D9},
        // D3D10
        Windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIFactory, DXGI_RATIONAL, DXGI_MODE_DESC, DXGI_FORMAT, DXGI_SAMPLE_DESC, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_EFFECT},
        Windows::Win32::Graphics::Direct3D10::{D3D10CreateDeviceAndSwapChain, D3D10_SDK_VERSION},
        // D3D11
        Windows::Win32::Graphics::Direct3D11::{D3D11CreateDeviceAndSwapChain, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL, D3D11_SDK_VERSION, D3D11_CREATE_DEVICE_FLAG},
        // D3D12
        Windows::Win32::Graphics::Direct3D12::{D3D12CreateDevice, ID3D12Device, D3D12_COMMAND_QUEUE_DESC, ID3D12CommandQueue, ID3D12CommandAllocator, ID3D12GraphicsCommandList},

        Windows::Win32::UI::WindowsAndMessaging::{WNDCLASSEXW, CreateWindowExW, DestroyWindow, UnregisterClassW},

        Windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, LPTHREAD_START_ROUTINE, CHAR, DLL_THREAD_ATTACH, DLL_THREAD_DETACH, CHAR},

        Windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory, GetLastError},

        Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next, MODULEENTRY32, Module32First, Module32Next, CREATE_TOOLHELP_SNAPSHOT_FLAGS},

        Windows::Win32::System::WindowsProgramming::{INFINITE},

        Windows::Win32::UI::KeyboardAndMouseInput::GetAsyncKeyState,

        Windows::Win32::System::Memory::{VirtualProtect, VirtualProtectEx, VirtualQueryEx, VirtualAllocEx, VirtualFreeEx},
    );
}
