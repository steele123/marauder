# mem-rs
mem-rs is a windows memory hacking library

## Note
This project is in active development the main focus is on DLL injection internal related stuff, external cheats are not the top 
priority at this moment.

# Examples
Below will be a bunch of examples, if you want more indepth examples typically with comments you can check out the 
examples directory

## Easy DLL creation with minimal boilerplate
```rust
#[mem::dll_main]
fn main() {
    // This is a fully functional DLL ready for injection!
    println!("I am a DLL inside the process, my module handle is: {}!", module_handle);
}
// We also support async! By default making your dll_main async you will be running on the tokio runtime
#[mem::dll_main]
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
    
    let pid = mem::windows::utils::get_process_id("target_process.exe").unwrap();
    injector.inject(pid, &dll_path).unwrap();
    println!("Successfully Injected!")
}
```