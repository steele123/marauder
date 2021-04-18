use crate::windows::wrappers::HMODULE;

pub struct Process {
    pub handle: HMODULE,
}

#[cfg(feature = "external")]
impl Process {
    pub fn new(name: &str) {}
}