fn main() {
    println!("External");
    if cfg!(feature = "external") {
        println!("aww")
    }

    if cfg!(feature = "internal") {
        println!("aww2")
    }
}
