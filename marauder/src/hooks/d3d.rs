use std::{
    ffi::CString,
    ptr::{null, null_mut},
};

use bindings::Windows::Win32::{
    Foundation::HWND,
    Graphics::{
        Direct3D10::{D3D10CreateDeviceAndSwapChain, D3D10_DRIVER_TYPE_HARDWARE, D3D10_SDK_VERSION},
        Direct3D11::{
            D3D11CreateDeviceAndSwapChain, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION, D3D_DRIVER_TYPE_HARDWARE,
            D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0,
        },
        Direct3D12::{
            D3D12CreateDevice, ID3D12CommandAllocator, ID3D12CommandQueue, ID3D12Device, ID3D12GraphicsCommandList,
            D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE,
        },
        Direct3D9::{
            Direct3DCreate9, IDirect3D9, IDirect3DDevice9, D3DCREATE_SOFTWARE_VERTEXPROCESSING, D3DDEVTYPE_HAL,
            D3DFMT_UNKNOWN, D3DMULTISAMPLE_NONE, D3DPRESENT_PARAMETERS, D3DSWAPEFFECT_DISCARD, D3D_SDK_VERSION,
        },
        Dxgi::{
            CreateDXGIFactory, IDXGIAdapter, IDXGIFactory, DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC,
            DXGI_MODE_SCALING, DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_RATIONAL,
            DXGI_SAMPLE_DESC, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_EFFECT_DISCARD,
            DXGI_SWAP_EFFECT_FLIP_DISCARD,
        },
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
        lpszClassName: "marauder".into(),
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

    let method_table = match render_type {
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
        RenderType::D3D10 => unsafe {
            let factory = CreateDXGIFactory::<IDXGIFactory>();
            if factory.is_err() {
                return Err(Error::DummyDevice);
            }
            let adapter: *const IDXGIAdapter = null();
            factory.unwrap().EnumAdapters(&adapter as u32);
            let refresh_rate = DXGI_RATIONAL {
                Numerator: 60,
                Denominator: 1,
            };

            let buffer_desc = DXGI_MODE_DESC {
                Width: 100,
                Height: 100,
                RefreshRate: refresh_rate,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
            };
            let sample_desc = DXGI_SAMPLE_DESC { Count: 1, Quality: 0 };
            let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: buffer,
                SampleDesc: sample,
                BufferUsage: 32,
                BufferCount: 1,
                OutputWindow: window,
                Windowed: true.into(),
                SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32,
            };

            let swap_chain = null_mut();
            let device = null_mut();

            D3D10CreateDeviceAndSwapChain(
                adapter,
                D3D10_DRIVER_TYPE_HARDWARE,
                null(),
                0,
                D3D10_SDK_VERSION,
                &swap_chain_desc as *mut DXGI_SWAP_CHAIN_DESC,
                swap_chain,
                device,
            )
            .unwrap();
        },
        RenderType::D3D11 => unsafe {
            let feature_levels = vec![D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0];
            let refresh_rate = DXGI_RATIONAL {
                Numerator: 60,
                Denominator: 1,
            };
            let buffer_desc = DXGI_MODE_DESC {
                Width: 100,
                Height: 100,
                RefreshRate: refresh_rate,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
            };
            let sample_desc = DXGI_SAMPLE_DESC { Count: 1, Quality: 0 };
            let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: buffer_desc,
                SampleDesc: sample_desc,
                BufferUsage: 32,
                BufferCount: 1,
                OutputWindow: window,
                Windowed: true.into(),
                SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32,
            };
            let swap_chain = null_mut();
            let device = null_mut();
            let context = null_mut();

            D3D11CreateDeviceAndSwapChain(
                null_mut(),
                D3D_DRIVER_TYPE_HARDWARE,
                null_mut(),
                D3D11_CREATE_DEVICE_FLAG(0),
                feature_levels.as_ptr(),
                2,
                D3D11_SDK_VERSION,
                swap_chain_desc as *const DXGI_SWAP_CHAIN_DESC,
                swap_chain,
                device,
                feature_level,
                context,
            )
            .unwrap();
        },
        RenderType::D3D12 => unsafe {
            let factory = CreateDXGIFactory::<IDXGIFactory>();
            let adapter = factory.unwrap().EnumAdapters();
            let device = D3D12CreateDevice::<ID3D12Device>(adapter, D3D_FEATURE_LEVEL_11_0);
            let queue_desc = D3D12_COMMAND_QUEUE_DESC {
                Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                Priority: 0,
                Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                NodeMask: 0,
            };
            let command_queue = device.unwrap().CreateCommandQueue::<ID3D12CommandQueue>(&queue_desc).unwrap();
            let command_allocator = device
                .unwrap()
                .CreateCommandAllocator::<ID3D12CommandAllocator>(D3D12_COMMAND_LIST_TYPE_DIRECT)
                .unwrap();
            let command_list = device
                .unwrap()
                .CreateCommandList::<ID3D12GraphicsCommandList>(
                    0,
                    D3D12_COMMAND_LIST_TYPE_DIRECT,
                    command_allocator,
                    null_mut(),
                )
                .unwrap();
            let refresh_rate = DXGI_RATIONAL {
                Numerator: 60,
                Denominator: 1,
            };
            let buffer_desc = DXGI_MODE_DESC {
                Width: 100,
                Height: 100,
                RefreshRate: refresh_rate,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
            };
            let sample_desc = DXGI_SAMPLE_DESC { Count: 1, Quality: 0 };
            let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: buffer_desc,
                SampleDesc: sample_desc,
                BufferUsage: 32,
                BufferCount: 2,
                OutputWindow: window,
                Windowed: true.into(),
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32,
            };
            let swap_chain = factory
                .unwrap()
                .CreateSwapChain(command_queue, &swap_chain_desc as *mut DXGI_SWAP_CHAIN_DESC)
                .unwrap();
        },
        _ => unreachable!(),
    };

    destroy_class(&window_class, &window);

    method_table
}

fn destroy_class(class: &WNDCLASSEXW, window: &HWND) {
    unsafe {
        DestroyWindow(window);
        UnregisterClassW(class.lpszClassName, class.hInstance);
    };
}
