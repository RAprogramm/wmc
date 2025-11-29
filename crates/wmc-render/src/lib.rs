//! World Map Component - WebGL rendering engine
//!
//! This crate provides WebGL-based rendering for world maps and markers.

/// GPU buffer management
pub mod buffer;
/// WebGL rendering context
pub mod context;
/// Render error types
pub mod error;
/// Shader program utilities
pub mod program;
/// GLSL shader sources
pub mod shaders;
/// World map renderer
pub mod world_renderer;

pub use buffer::GpuBuffer;
pub use context::RenderContext;
pub use error::RenderError;
pub use program::ShaderProgram;
pub use world_renderer::WorldRenderer;
