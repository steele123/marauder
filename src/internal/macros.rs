use crate::windows::wrappers::{
    close_handle, create_thread, disable_thread_library_calls, DWORD, HMODULE, LPVOID,
};
use bindings::Windows::Win32::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

/// This will help you create a DllMain function so you can inject it properly.
#[macro_export]
macro_rules! dll_main {
    ($func:expr) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "system" fn DllMain(module: HMODULE, reason: DWORD, reserved: LPVOID) -> u32 {
            match reason {
                DLL_PROCESS_ATTACH => {
                    disable_thread_library_calls(module);
                    close_handle(create_thread(
                        std::ptr::null_mut(),
                        0,
                        Some($func),
                        module,
                        0,
                        std::ptr::null_mut(),
                    ))
                }
                _ => (),
            };
        }

        return true as u32;
    };
}
