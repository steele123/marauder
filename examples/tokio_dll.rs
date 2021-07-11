//! This is a sample of a DLL using the tokio runtime

// By default putting dll_main as async will use the tokio runtime, I have no
// plans atm of supporting any others as its very simple to build a runtime
#[mem::dll_main]
async fn main() {
    println!("Hi from the macro dll, module_handle: {}", module_handle);
}
