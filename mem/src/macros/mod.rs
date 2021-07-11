use bindings::Windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

pub(crate) use crate::windows::wrappers::{close_handle, create_thread, DWORD, HMODULE, LPVOID};

#[macro_use]
mod cfg;

cfg_macros! {
    #[macro_use]
    mod ptr;
}
