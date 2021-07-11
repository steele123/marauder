#[cfg(any(feature = "d3d9", feature = "d3d10", feature = "d3d11", feature = "d3d12"))]
pub mod d3d;

#[cfg(feature = "opengl")]
pub mod opengl;

#[cfg(feature = "vulkan")]
pub mod vulkan;

pub enum RenderType {
    OPENGL,
    VULKAN,
    D3D9,
    D3D10,
    D3D11,
    D3D12,
}
