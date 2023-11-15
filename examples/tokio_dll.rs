//! This is a sample of a DLL using the tokio runtime

// By default putting dll_main as async will use the tokio runtime, I have no
// plans atm of supporting any others as its very simple to build a runtime
#[marauder::dll_main]
async fn main() {
    println!("Hi from tokio, module_handle: {:?}", module_handle);
}
