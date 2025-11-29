use std::fmt;

use masterror::AppError;

/// Rendering errors
#[derive(Debug, Clone)]
pub enum RenderError {
    /// Failed to create WebGL2 context
    WebGLContextCreationFailed,
    /// Shader compilation failed
    ShaderCompilationFailed {
        /// Shader type (vertex/fragment)
        shader_type: String,
        /// Compilation error log
        log: String,
    },
    /// Program linking failed
    ProgramLinkingFailed {
        /// Linking error log
        log: String,
    },
    /// Buffer allocation failed
    BufferAllocationFailed {
        /// Requested size in bytes
        size: usize,
    },
    /// WebGL2 not supported
    UnsupportedWebGLVersion,
    /// Uniform location not found
    UniformLocationNotFound {
        /// Uniform name
        name: String,
    },
    /// VAO creation failed
    VaoCreationFailed,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebGLContextCreationFailed => write!(f, "Failed to create WebGL2 context"),
            Self::ShaderCompilationFailed { shader_type, log } => {
                write!(f, "Shader compilation failed ({shader_type}): {log}")
            },
            Self::ProgramLinkingFailed { log } => write!(f, "Program linking failed: {log}"),
            Self::BufferAllocationFailed { size } => {
                write!(f, "Buffer allocation failed: {size} bytes")
            },
            Self::UnsupportedWebGLVersion => write!(f, "WebGL2 not supported"),
            Self::UniformLocationNotFound { name } => {
                write!(f, "Uniform location not found: {name}")
            },
            Self::VaoCreationFailed => write!(f, "Failed to create Vertex Array Object"),
        }
    }
}

impl std::error::Error for RenderError {}

impl From<RenderError> for AppError {
    fn from(err: RenderError) -> Self {
        Self::internal(err.to_string())
    }
}
