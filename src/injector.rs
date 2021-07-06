use crate::error::Error;
use crate::windows::wrappers::{open_process, DWORD, LPVOID};
use bindings::Windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use bindings::Windows::Win32::System::Memory::{
    VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, VIRTUAL_ALLOCATION_TYPE,
};
use bindings::Windows::Win32::System::SystemServices::{
    GetModuleHandleA, GetProcAddress, HINSTANCE, INVALID_HANDLE_VALUE, PAGE_READWRITE, PSTR, PWSTR,
};
use bindings::Windows::Win32::System::Threading::PROCESS_ALL_ACCESS;
use bindings::Windows::Win32::System::WindowsProgramming::CloseHandle;
use std::ffi::CString;
use std::ptr::null_mut;

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
    pub injection_method: InjectionMethod,
    pub execution_method: CodeExecutionMethod,
    pub cloak_thread: Option<bool>,
    pub randomize_file_name: Option<bool>,
    pub pe_cloaking: Option<PECloaking>,
}

pub struct Injector {
    config: Config,
}

impl Injector {
    pub fn new(config: Config) -> Injector {
        Injector { config }
    }

    pub fn inject(&self, process_id: u32, dll_path: &str) -> Result<(), Error> {
        let load_lib_address = get_fn_address("Kernel32.dll", "LoadLibraryA")?;

        let dll_path_cstr = CString::new(dll_path)?;
        let dll_path_size = dll_path_cstr.as_bytes_with_nul().len();

        let process_handle = open_process(PROCESS_ALL_ACCESS, false.into(), process_id);

        if process_handle == INVALID_HANDLE_VALUE {
            return Err(Error::Handle);
        }

        let path = unsafe {
            VirtualAllocEx(
                process_handle,
                null_mut(),
                dll_path_size,
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            )
        };

        if path.is_null() {
            return Err(std::io::Error::last_os_error().into());
        }

        let success: bool = unsafe {
            WriteProcessMemory(
                process_handle,
                path,
                dll_path_cstr.as_ptr() as LPVOID,
                dll_path_size,
                null_mut(),
            )
            .into()
        };

        if !success {
            return Err(Error::MemoryWrite);
        }

        let thread_handle = unsafe {
            type StartRoutine = unsafe extern "system" fn(LPVOID) -> DWORD;
            let start_routine: StartRoutine = std::mem::transmute(load_lib_address);
            bindings::Windows::Win32::System::Threading::CreateRemoteThread(
                process_handle,
                null_mut(),
                0,
                Some(start_routine),
                path,
                0,
                null_mut(),
            )
        };

        if thread_handle == INVALID_HANDLE_VALUE {
            unsafe { VirtualFreeEx(process_handle, path, dll_path_size, MEM_RELEASE) };

            return Err(Error::Handle);
        }

        unsafe { VirtualFreeEx(process_handle, path, dll_path_size, MEM_RELEASE) };
        unsafe { CloseHandle(thread_handle) };

        Ok(())
    }
}

fn get_fn_address<'a>(module_name: &str, fn_name: &str) -> Result<u64, Error> {
    let mod_str = CString::new(module_name)?;
    let fn_str = CString::new(fn_name)?;

    let module_handle = unsafe { GetModuleHandleA(PSTR(mod_str.into_raw() as _)) };

    if module_handle == HINSTANCE::NULL {
        return Err(Error::Handle);
    }

    let function = unsafe { GetProcAddress(module_handle, PSTR(fn_str.into_raw() as _)) };

    match function {
        None => Err(Error::ProcessAddress),
        Some(function) => Ok(function as u64),
    }
}
