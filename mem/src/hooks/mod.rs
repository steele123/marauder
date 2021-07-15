use crate::error::{Error, Result};

#[cfg(any(feature = "d3d9", feature = "d3d10", feature = "d3d11", feature = "d3d12"))]
pub mod d3d;

#[cfg(feature = "opengl")]
pub mod opengl;

#[cfg(feature = "vulkan")]
pub mod vulkan;

pub enum RenderType {
    OPENGL = 0,
    VULKAN,
    D3D9,
    D3D10,
    D3D11,
    D3D12,
}

pub struct GraphicsHook {
    method_table: Vec<*const usize>,
}

impl GraphicsHook {
    /// Acquires the method table by creating a dummy device
    pub fn new(render_type: RenderType) -> Result<Self> {
        match render_type {
            RenderType::OPENGL => {},
            RenderType::VULKAN => {},
            RenderType::D3D9 | RenderType::D3D10 | RenderType::D3D11 | RenderType::D3D12 => {},
        }

        Ok(GraphicsHook)
    }

    // TODO: DOC
    pub fn hook(index: u16) -> Result<()> {}

    // TODO: DOC
    pub fn unhook() -> Result<()> {}
}
