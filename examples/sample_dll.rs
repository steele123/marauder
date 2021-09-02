//! This example is a very basic DLL that will be injected into the process and
//! print to stdout

use mem::windows::wrappers::{HModule, DWORD, LPVOID};

fn hack_thread() {
    println!("Hi from the sample dll");
}

#[no_mangle]
pub extern "system" fn DllMain(_module_handle: HModule, dw_reason: DWORD, _lp_reserved: LPVOID) -> bool {
    match dw_reason {
        1u32 => {
            std::thread::spawn(|| hack_thread());
        },
        _ => return false,
    };

    true
}
