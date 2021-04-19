use mem::external::mem::Mem;

fn main() {
    if cfg!(feature = "external") {
        println!("I am an external cheat.")
    }

    let mem = Mem::new("BloonsTD6.exe").expect("couldn't find BTD6");
}
