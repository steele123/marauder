#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    non_snake_case,
    dead_code,
    clippy::cast_possible_wrap,
    clippy::upper_case_acronyms,
    clippy::not_unsafe_ptr_arg_deref
)]

#[cfg(not(target_os = "windows"))]
compile_error!("Currently only windows is supported");

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(feature = "external")]
pub mod external;
#[cfg(feature = "injector")]
pub mod injector;
#[cfg(feature = "internal")]
pub mod internal;

// This just checks if the user wants any of the graphic libraries
#[cfg(any(
    feature = "vulkan",
    feature = "opengl",
    feature = "d3d9",
    feature = "d3d10",
    feature = "d3d11",
    feature = "d3d12"
))]
pub mod hooks;

#[macro_use]
pub mod macros;

pub mod error;

// proc macros
cfg_macros! {
    pub use mem_macros::dll_main;
}
