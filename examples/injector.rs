use marauder::{
    injector::{Config, Injector},
    windows::utils::get_process_id,
};

fn main() {
    let dll_path = std::env::var("dll_path").expect("You must provide a dll path");
    let process_name = std::env::var("process_name").expect("You must give a process name");

    let path = std::path::Path::new(&dll_path);
    if !path.exists() {
        panic!("The DLL doesn't exist at {}", dll_path);
    }
    let config = Config::default();
    let injector = Injector::new(config);

    let pid = get_process_id(&process_name).unwrap();
    injector.inject(pid, &dll_path).unwrap();
    println!("Successfully Injected!")
}
