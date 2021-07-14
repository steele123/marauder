use std::{ffi::CString, ptr::null_mut};

use bindings::Windows::Win32::{
    Foundation::HWND,
    Graphics::Direct3D9::{
        Direct3DCreate9, IDirect3D9, D3DFMT_UNKNOWN, D3DMULTISAMPLE_NONE, D3DPRESENT_PARAMETERS, D3DSWAPEFFECT_DISCARD,
        D3D_SDK_VERSION,
    },
    UI::WindowsAndMessaging::{
        CreateWindowExW, DestroyWindow, UnregisterClassW, CS_HREDRAW, CS_VREDRAW, WNDCLASSEXW, WS_EX_APPWINDOW,
        WS_OVERLAPPEDWINDOW,
    },
};

use crate::{
    hooks::RenderType,
    windows::{
        utils::convert_windows_string,
        wrappers::{get_module_handle, get_proc_address, HandleInstance},
    },
};

fn setup_d3d_hook(render_type: RenderType) {
    let window_class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: None,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: Default::default(),
        hIcon: Default::default(),
        hCursor: Default::default(),
        hbrBackground: Default::default(),
        lpszMenuName: Default::default(),
        lpszClassName: "mem".into(),
        hIconSm: Default::default(),
    };

    let window = unsafe {
        CreateWindowExW(
            WS_EX_APPWINDOW,
            window_class.lpszClassName,
            "",
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            100,
            100,
            null_mut(),
            null_mut(),
            window_class.hInstance,
            null_mut(),
        )
    };

    match render_type {
        RenderType::D3D9 => {
            let direct3d9 = unsafe { Direct3DCreate9(D3D_SDK_VERSION).unwrap() };
            let params = D3DPRESENT_PARAMETERS {
                BackBufferWidth: 0,
                BackBufferHeight: 0,
                BackBufferFormat: D3DFMT_UNKNOWN,
                BackBufferCount: 0,
                MultiSampleType: D3DMULTISAMPLE_NONE,
                MultiSampleQuality: 0,
                SwapEffect: D3DSWAPEFFECT_DISCARD,
                hDeviceWindow: window,
                Windowed: true.into(),
                EnableAutoDepthStencil: false.into(),
                AutoDepthStencilFormat: D3DFMT_UNKNOWN,
                Flags: 0,
                FullScreen_RefreshRateInHz: 0,
                PresentationInterval: 0,
            };
            //let dummy_device = unsafe { direct3d9.CreateDevice(D3DADAPTER_DE)
            // };
        },
        RenderType::D3D10 => {},
        RenderType::D3D11 => {},
        RenderType::D3D12 => {},
        _ => unreachable!(),
    }
}

fn get_graphics_library(library_name: &str) -> HandleInstance {
    let d3d9_library = if let Ok(library) = get_module_handle(library_name) {
        library
    } else {
    };
}
