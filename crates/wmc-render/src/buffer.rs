use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::error::RenderError;

/// WebGL buffer wrapper
pub struct GpuBuffer {
    buffer: WebGlBuffer,
    target: u32,
    usage: u32,
}

impl GpuBuffer {
    /// Creates a new GPU buffer
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::BufferAllocationFailed`] if buffer creation fails
    pub fn new(gl: &WebGl2RenderingContext, target: u32, usage: u32) -> Result<Self, RenderError> {
        let buffer = gl
            .create_buffer()
            .ok_or(RenderError::BufferAllocationFailed { size: 0 })?;

        Ok(Self {
            buffer,
            target,
            usage,
        })
    }

    /// Binds the buffer to its target
    pub fn bind(&self, gl: &WebGl2RenderingContext) {
        gl.bind_buffer(self.target, Some(&self.buffer));
    }

    /// Unbinds the buffer from its target
    pub fn unbind(&self, gl: &WebGl2RenderingContext) {
        gl.bind_buffer(self.target, None);
    }

    /// Uploads data to the buffer
    pub fn upload_data(&self, gl: &WebGl2RenderingContext, data: &[u8]) {
        self.bind(gl);
        gl.buffer_data_with_u8_array(self.target, data, self.usage);
    }

    /// Updates a portion of the buffer data
    pub fn update_data(&self, gl: &WebGl2RenderingContext, offset: i32, data: &[u8]) {
        self.bind(gl);
        gl.buffer_sub_data_with_i32_and_u8_array(self.target, offset, data);
    }
}
