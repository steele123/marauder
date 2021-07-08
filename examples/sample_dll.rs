//! This example is a very basic DLL that will be injected into the process and
//! print to stdout
use std::thread::spawn;

use bindings::Windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};
use mem::windows::wrappers::{DWORD, HMODULE, LPVOID};

fn hack_thread() {
    println!("Hi from the sample dll");
}

#[no_mangle]
pub extern "system" fn DllMain(module_handle: HMODULE, dw_reason: DWORD, reserved: LPVOID) -> bool {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            spawn(hack_thread);
        },
        DLL_PROCESS_DETACH => {},
        DLL_THREAD_ATTACH => {},
        DLL_THREAD_DETACH => {},
        _ => {
            unreachable!()
        },
    }

    true
}
