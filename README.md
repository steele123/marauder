# marauder
marauder is a windows (maybe eventually other OS) game hacking (kinda?) library, it's main goal is to make it easy to
create DLLs and inject them into processes. It also provides some utilities for reading/writing memory and currently
plans to support belong D3D hooks of all kinds.

# Checklists
- [✅] DLL creation
- [✅] Simple Injection
- [✅] Utility functions for reading/writing memory
- [⏳] D3D hooks

# Install
```toml
[dependencies]
marauder = "0.1.0"

# or if you just want the DLL creation macros
[dependencies]
marauder-macros = "0.1.0"
```

# Examples
Below will be a bunch of examples, if you want more indepth examples typically with comments you can check out the 
examples directory

## Easy DLL creation with minimal boilerplate
```rust
#[marauder::dll_main]
fn main() {
    // This is a fully functional DLL ready for injection!
    println!("I am a DLL inside the process, my module handle is: {}!", module_handle);
}
// We also support async! By default making your dll_main async you will be running on the tokio runtime
#[marauder::dll_main]
async fn main() {
    
}
```

## Injection
```rust
fn main() {
    let dll_path = std::env::var("dll_path").expect("You must provide a dll path");
    let path = std::path::Path::new(&dll_path);
    if !path.exists() {
        panic!("The DLL doesn't exist at {}", dll_path);
    }
    // By default the config will use a LoadLibrary injection with no stealth
    let config = Config::default();
    let injector = Injector::new(config);
    
    let pid = marauder::windows::utils::get_process_id("target_process.exe").unwrap();
    injector.inject(pid, &dll_path).unwrap();
    println!("Successfully Injected!")
}
```
