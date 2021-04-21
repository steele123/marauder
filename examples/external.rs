use mem::MemFns;

fn main() {
    if cfg!(feature = "external") {
        println!("I am an external cheat.")
    }

    let mem = mem::external::Mem::new("BloonsTD6.exe").expect("couldn't find BTD6");
}
