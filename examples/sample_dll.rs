//! This example is a very basic DLL that will be injected into the process and print to stdout
use bindings::Windows::Win32::System::SystemServices::{
    BOOL, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};
use mem::windows::wrappers::{DWORD, HMODULE, LPVOID};
use std::thread::spawn;

fn hack_thread() {
    println!("Hi from the sample dll");
}

#[no_mangle]
pub extern "system" fn DllMain(module_handle: HMODULE, dw_reason: DWORD, reserved: LPVOID) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            spawn(hack_thread);
        }
        DLL_PROCESS_DETACH => {}
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => {
            unreachable!()
        }
    }

    true.into()
}
