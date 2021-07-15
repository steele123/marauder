use std::{ffi::CString, ptr::null_mut};

use bindings::Windows::Win32::{
    Foundation::HWND,
    Graphics::Direct3D9::{
        Direct3DCreate9, IDirect3D9, IDirect3DDevice9, D3DCREATE_SOFTWARE_VERTEXPROCESSING, D3DDEVTYPE_HAL, D3DFMT_UNKNOWN,
        D3DMULTISAMPLE_NONE, D3DPRESENT_PARAMETERS, D3DSWAPEFFECT_DISCARD, D3D_SDK_VERSION,
    },
    UI::WindowsAndMessaging::{
        CreateWindowExW, DestroyWindow, UnregisterClassW, CS_HREDRAW, CS_VREDRAW, WNDCLASSEXW, WS_EX_APPWINDOW,
        WS_OVERLAPPEDWINDOW,
    },
};

use crate::{
    error::{Error, Result},
    hooks::{MethodTable, RenderType},
    windows::{
        utils::convert_windows_string,
        wrappers::{get_module_handle, get_proc_address, HandleInstance},
    },
};

#[cfg(feature = "d3d9")]
const D3D9_VTABLE_ELEMENTS: usize = 119;
#[cfg(feature = "d3d10")]
const D3D10_VTABLE_ELEMENTS: usize = 116;
#[cfg(feature = "d3d11")]
const D3D11_VTABLE_ELEMENTS: usize = 205;
#[cfg(feature = "d3d12")]
const D3D12_VTABLE_ELEMENTS: usize = 150;

pub fn get_method_table(render_type: RenderType) -> Result<Vec<MethodTable>> {
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

    return match render_type {
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
            let device_interface: *mut IDirect3DDevice9 = std::ptr::null_mut();
            let dummy_device = unsafe {
                direct3d9.CreateDevice(
                    0u32,
                    D3DDEVTYPE_HAL,
                    params.hDeviceWindow,
                    32u32 | 256u32,
                    std::mem::transmute(params),
                    std::mem::transmute(device_interface),
                )
            };

            if dummy_device.is_err() {
                Err(Error::DummyDevice)
            }

            // size is the size of the elements, not the bytes this is similar to calloc in
            // c++
            let method_table = unsafe {
                std::slice::from_raw_parts((device_interface as *const *const MethodTable).read(), D3D9_VTABLE_ELEMENTS)
            }
            .to_vec();
            if method_table.is_empty() {
                Err(Error::DummyDevice)
            }

            Ok(method_table)
        },
        RenderType::D3D10 => {},
        RenderType::D3D11 => {},
        RenderType::D3D12 => {},
        _ => unreachable!(),
    };
}
