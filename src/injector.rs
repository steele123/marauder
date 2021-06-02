use crate::error::Error;
use crate::windows::utils::get_process_id;
use crate::windows::wrappers::open_process;
use bindings::Windows::Win32::System::SystemServices::INVALID_HANDLE_VALUE;
use bindings::Windows::Win32::SystemServices::{
    HANDLE, INVALID_HANDLE_VALUE, PROCESS_ACCESS_RIGHTS,
};

/// Several different methods of loading our library into the target process
pub enum InjectionMethod {
    /// This is the typical method when safety is not really a concern
    LoadLibrary,
    /// LoadLibraryEx is just a extended version of LoadLibrary which isn't always detected by anti-cheats
    LoadLibraryEx,
    /// By far the safest method of injection, this does mostly what the other methods do but is
    /// instead just manually done by us rather than windows functions thus you are hidden from
    /// ToolHelp32Snapshot and module crawling because windows didn't load them
    ManualMap,
}

/// These are methods in which the injector will execute the code from the DLL that is injected
pub enum CodeExecutionMethod {
    /// Creates a new thread on the target process which will have DllMain called
    CreateRemoteThread,
    /// Hijacks an existing thread, this is used for stealth if the anti cheat detects creating a new thread
    ThreadHijack,
}

/// Choices of what to do with PE headers to protect ourselves and be more stealthy
pub enum PECloaking {
    /// We will do nothing with PE headers
    Keep,
    /// PE headers will be erased
    Erase,
    /// PE headers will be scrambled with fake ones
    Fake,
}

pub struct Config {
    injection_method: InjectionMethod,
    execution_method: CodeExecutionMethod,
    cloak_thread: Option<bool>,
    randomize_file_name: Option<bool>,
    pe_cloaking: Option<PECloaking>,
}

pub struct Injector {
    config: Config,
}

impl Injector {
    pub fn new(config: Config) -> Injector {
        Injector { config }
    }

    pub fn inject(&self, process_id: u32, dll_path: &str) -> Result<(), Error> {
        let process_handle = open_process(
            PROCESS_ACCESS_RIGHTS::PROCESS_ALL_ACCESS,
            false.into(),
            process_id,
        );

        if process_handle == INVALID_HANDLE_VALUE {
            return Err(Error::Handle);
        }

        Ok(())
    }
}
