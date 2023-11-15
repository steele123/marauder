//! This is a DLL using our macros ONLY and not our full library when you don't care about anything
//! but the DLL generation.

#[marauder-macros::dll_main]
fn main() {
    println!("Hello, world!")
}