//! This is just a very dumb process that does nothing besides run and wait to be injected into

fn main() {
    println!(
        "I am a very dumb target process that will run forever!\nMy process id is {}",
        std::process::id()
    );

    loop {}
}
