fn main() {
    if cfg!(feature = "external") {
        println!("This is a external memory reader for the game assault cube, I cannot redistribute the files but its a easy game to find")
    }

    let mem = mem::internal::mem::Mem::new("AssaultCube.exe");
}
