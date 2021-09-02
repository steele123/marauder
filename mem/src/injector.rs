use bindings::Windows::Win32::System::{
    Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE},
    Threading::PROCESS_ALL_ACCESS,
};

use crate::{
    error::Error,
    windows::wrappers::{
        close_handle, create_remote_thread, get_module_handle, get_proc_address, open_process, virtual_alloc_ex,
        write_process_memory, DWORD, LPVOID,
    },
};

/// Several methods of loading our library into the target process
pub enum InjectionMethod {
    /// This is the typical method when safety is not really a concern
    LoadLibrary,
    /// LoadLibraryEx is just a extended version of LoadLibrary which isn't
    /// always detected by anti-cheats
    LoadLibraryEx,
    /// By far the safest method of injection, this does mostly what the other
    /// methods do but is instead just manually done by us rather than
    /// windows functions thus you are hidden from ToolHelp32Snapshot and
    /// module crawling because windows didn't load them
    ManualMap,
}

/// These are methods in which the injector will execute the code from the DLL
/// that is injected
pub enum CodeExecutionMethod {
    /// Creates a new thread on the target process which will have DllMain
    /// called
    CreateRemoteThread,
    /// Hijacks an existing thread, this is used for stealth if the anti cheat
    /// detects creating a new thread
    ThreadHijack,
}

/// Choices of what to do with PE headers to protect ourselves and be more
/// stealthy
pub enum PECloaking {
    /// We will do nothing with PE headers
    Keep,
    /// PE headers will be erased
    Erase,
    /// PE headers will be scrambled with fake ones
    Fake,
}

pub struct Config {
    pub injection_method: InjectionMethod,
    pub execution_method: CodeExecutionMethod,
    pub cloak_thread: bool,
    pub randomize_file_name: bool,
    pub pe_cloaking: PECloaking,
}

impl Config {
    const fn max_stealth() -> Self {
        Self {
            injection_method: InjectionMethod::ManualMap,
            execution_method: CodeExecutionMethod::ThreadHijack,
            cloak_thread: true,
            randomize_file_name: true,
            pe_cloaking: PECloaking::Fake,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            injection_method: InjectionMethod::LoadLibrary,
            execution_method: CodeExecutionMethod::CreateRemoteThread,
            cloak_thread: false,
            randomize_file_name: false,
            pe_cloaking: PECloaking::Keep,
        }
    }
}

pub struct Injector {
    config: Config,
}

impl Injector {
    #[must_use]
    pub const fn new(config: Config) -> Self { Self { config } }

    /// # Errors
    pub fn inject(&self, process_id: u32, dll_path: &str) -> Result<(), Error> {
        let load_lib_address = get_proc_address(get_module_handle("Kernel32.dll")?, "LoadLibraryA")?;

        let dll_path_size = dll_path.as_bytes().len();

        let process_handle = open_process(PROCESS_ALL_ACCESS, false, process_id);

        let path = virtual_alloc_ex(process_handle, None, dll_path_size, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE)?;

        write_process_memory(process_handle, path, dll_path.as_ptr() as LPVOID, dll_path_size, None)?;

        let thread_handle = unsafe {
            type StartRoutine = extern "system" fn(LPVOID) -> DWORD;
            let start_routine: StartRoutine = std::mem::transmute(load_lib_address);
            create_remote_thread(process_handle, None, 0, Some(start_routine), Some(path), 0, None)?
        };

        close_handle(thread_handle)?;

        Ok(())
    }
}
