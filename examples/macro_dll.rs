//! This is a sample dll using our macro for DllMain generation

// Basically your fn main is injected inside of a extern "system" DllMain
// function inside of a new thread this is nearly identical to our sample_dll
// example except just requires less boilerplate
#[mem::dll_main]
fn main() {
    println!("Hi from the macro dll {}", module_handle);
}
