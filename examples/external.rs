use mem::error::Error;
use mem::external::Mem;

fn main() -> Result<(), Error> {
    if cfg!(feature = "external") {
        println!("This is a external memory reader for the game assault cube, I cannot redistribute the files but its a easy game to find")
    }

    let mem = Mem::new("AssaultCube.exe").expect("You don't have AssaultCube.exe open");

    Ok(())
}
