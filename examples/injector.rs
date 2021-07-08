use mem::injector::{Config, InjectionMethod, Injector};
use mem::windows::utils::get_process_id;

fn main() {
    let dll_path = r#"C:\Users\14403\CLionProjects\mem-rs\target\debug\examples\sample_dll.dll"#;
    let path = std::path::Path::new(dll_path);
    if !path.exists() {
        panic!("The DLL doesn't exist at {}", dll_path);
    }
    let config = Config::default();
    let injector = Injector::new(config);

    //let pid = get_process_id("target_process.exe").unwrap();
    injector.inject(101784, &dll_path).unwrap();
    println!("Successfully Injected!")
}
