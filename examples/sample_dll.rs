//! This example is a very basic DLL that will be injected into the process and print to stdout
use mem::windows::wrappers::{DWORD, HMODULE, LPVOID};

fn hack_thread() {
    println!("Hi from the sample dll :)");
}

#[no_mangle]
pub extern "system" fn DllMain(module_handle: HMODULE, dw_reason: DWORD, reserved: LPVOID) {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(hack_thread);
        }
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        DLL_PROCESS_DETACH => {}
    }
}
