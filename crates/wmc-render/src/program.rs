use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

use crate::error::RenderError;

/// Compiled and linked GLSL shader program
pub struct ShaderProgram {
    program: WebGlProgram,
}

impl ShaderProgram {
    /// Creates a new shader program from vertex and fragment shader sources
    ///
    /// # Errors
    ///
    /// Returns [`RenderError`] if shader compilation or program linking fails
    pub fn new(
        gl: &WebGl2RenderingContext,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<Self, RenderError> {
        let vertex_shader =
            Self::compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_source)?;

        let fragment_shader =
            Self::compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, fragment_source)?;

        let program = Self::link_program(gl, &vertex_shader, &fragment_shader)?;

        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        Ok(Self { program })
    }

    /// Activates this shader program for rendering
    pub fn use_program(&self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));
    }

    /// Gets a uniform location by name
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::UniformLocationNotFound`] if the uniform doesn't exist
    pub fn get_uniform_location(
        &self,
        gl: &WebGl2RenderingContext,
        name: &str,
    ) -> Result<WebGlUniformLocation, RenderError> {
        gl.get_uniform_location(&self.program, name).ok_or_else(|| {
            RenderError::UniformLocationNotFound {
                name: name.to_string(),
            }
        })
    }

    /// Gets an attribute location by name
    #[must_use]
    pub fn get_attrib_location(&self, gl: &WebGl2RenderingContext, name: &str) -> i32 {
        gl.get_attrib_location(&self.program, name)
    }

    fn compile_shader(
        gl: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, RenderError> {
        let shader =
            gl.create_shader(shader_type)
                .ok_or_else(|| RenderError::ShaderCompilationFailed {
                    shader_type: shader_type_name(shader_type),
                    log: "Failed to create shader".to_string(),
                })?;

        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            let log = gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error".to_string());
            Err(RenderError::ShaderCompilationFailed {
                shader_type: shader_type_name(shader_type),
                log,
            })
        }
    }

    fn link_program(
        gl: &WebGl2RenderingContext,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, RenderError> {
        let program = gl
            .create_program()
            .ok_or_else(|| RenderError::ProgramLinkingFailed {
                log: "Failed to create program".to_string(),
            })?;

        gl.attach_shader(&program, vertex_shader);
        gl.attach_shader(&program, fragment_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            let log = gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error".to_string());
            Err(RenderError::ProgramLinkingFailed { log })
        }
    }
}

fn shader_type_name(shader_type: u32) -> String {
    match shader_type {
        WebGl2RenderingContext::VERTEX_SHADER => "vertex".to_string(),
        WebGl2RenderingContext::FRAGMENT_SHADER => "fragment".to_string(),
        _ => "unknown".to_string(),
    }
}
