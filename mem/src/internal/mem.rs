use crate::windows::wrappers::DWORD_PTR;

pub struct Mem {
    // module_base_address is typically not a usize but I want it to be added to pointers without casts
    pub module_base_address: DWORD_PTR,
}

impl Mem {
    /*
    fn new(optional_module_name: Option<&str>) -> Result<Self> {
        let module_base_address = match optional_module_name {
            Some(module_name) => get_module_handle(module_name),
            None => get_module_handle(std::ptr::null as &str),
        } as usize;

        let process = get_current_process();

        Ok(Self {
            process,
            module_base_address,
        })
    }
    */
}
