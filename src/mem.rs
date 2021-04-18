use crate::windows::wrappers::{DWORD_PTR, HMODULE};

pub struct Mem {
    pub handle: HMODULE,
    pub module_base_address: DWORD_PTR,
}

#[cfg(feature = "external")]
impl Mem {
    pub fn new(name: &str) {}
}

#[cfg(feature = "internal")]
impl Mem {
    pub fn new() {}
}
