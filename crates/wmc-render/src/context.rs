use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::error::RenderError;

/// WebGL rendering context with canvas state
pub struct RenderContext {
    gl: WebGl2RenderingContext,
    width: u32,
    height: u32,
}

impl RenderContext {
    /// Creates a new rendering context from a canvas element
    ///
    /// # Errors
    ///
    /// Returns [`RenderError`] if WebGL2 context creation fails
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, RenderError> {
        let gl = canvas
            .get_context("webgl2")
            .map_err(|_| RenderError::WebGLContextCreationFailed)?
            .ok_or(RenderError::WebGLContextCreationFailed)?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| RenderError::UnsupportedWebGLVersion)?;

        let width = canvas.width();
        let height = canvas.height();

        gl.viewport(0, 0, width as i32, height as i32);

        Ok(Self { gl, width, height })
    }

    /// Returns a reference to the WebGL2 context
    #[must_use]
    pub const fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    /// Returns the canvas width in pixels
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the canvas height in pixels
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Resizes the viewport
    #[allow(clippy::cast_possible_wrap)]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.gl.viewport(0, 0, width as i32, height as i32);
    }

    /// Clears the canvas with the specified color
    #[allow(clippy::many_single_char_names)]
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}
