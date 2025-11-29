use web_sys::WebGl2RenderingContext;
use wmc_core::{
    projection::{GeoCoord, Projection},
    topology::{Geometry, WorldTopology},
};

use crate::{
    buffer::GpuBuffer, context::RenderContext, error::RenderError, program::ShaderProgram,
};

/// WebGL renderer for world map topology
pub struct WorldRenderer {
    program: ShaderProgram,
    vao: web_sys::WebGlVertexArrayObject,
    _vertex_buffer: GpuBuffer,
    vertex_count: i32,
    u_color: web_sys::WebGlUniformLocation,
    u_resolution: web_sys::WebGlUniformLocation,
}

impl WorldRenderer {
    /// Creates a new world renderer
    ///
    /// # Errors
    ///
    /// Returns [`RenderError`] if shader compilation or buffer creation fails
    pub fn new(
        ctx: &RenderContext,
        topology: &WorldTopology,
        projection: &dyn Projection,
    ) -> Result<Self, RenderError> {
        let gl = ctx.gl();

        let program = ShaderProgram::new(
            gl,
            crate::shaders::WORLD_VERTEX,
            crate::shaders::WORLD_FRAGMENT,
        )?;

        let u_color = program.get_uniform_location(gl, "u_color")?;
        let u_resolution = program.get_uniform_location(gl, "u_resolution")?;

        let vertices = Self::build_vertices(topology, projection);
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let vertex_count = (vertices.len() / 2) as i32;

        let vertex_buffer = GpuBuffer::new(
            gl,
            WebGl2RenderingContext::ARRAY_BUFFER,
            WebGl2RenderingContext::STATIC_DRAW,
        )?;

        #[allow(unsafe_code)]
        let vertices_bytes = unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr().cast::<u8>(),
                vertices.len() * std::mem::size_of::<f32>(),
            )
        };

        vertex_buffer.upload_data(gl, vertices_bytes);

        let vao = gl
            .create_vertex_array()
            .ok_or(RenderError::VaoCreationFailed)?;

        gl.bind_vertex_array(Some(&vao));

        vertex_buffer.bind(gl);
        gl.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.bind_vertex_array(None);

        Ok(Self {
            program,
            vao,
            _vertex_buffer: vertex_buffer,
            vertex_count,
            u_color,
            u_resolution,
        })
    }

    /// Draws the world map
    pub fn draw(&self, ctx: &RenderContext, color: [f32; 4], line_width: f32) {
        let gl = ctx.gl();

        self.program.use_program(gl);

        gl.uniform4f(Some(&self.u_color), color[0], color[1], color[2], color[3]);
        #[allow(clippy::cast_precision_loss)]
        {
            gl.uniform2f(
                Some(&self.u_resolution),
                ctx.width() as f32,
                ctx.height() as f32,
            );
        }

        gl.line_width(line_width);

        gl.bind_vertex_array(Some(&self.vao));
        gl.draw_arrays(WebGl2RenderingContext::LINES, 0, self.vertex_count);
        gl.bind_vertex_array(None);
    }

    fn build_vertices(topology: &WorldTopology, projection: &dyn Projection) -> Vec<f32> {
        let estimated_size = topology.point_count() * 2 * 2;
        let mut vertices = Vec::with_capacity(estimated_size);

        for feature in &topology.features {
            match &feature.geometry {
                Geometry::LineString(points) => {
                    Self::add_line(&mut vertices, points, projection);
                },
                Geometry::MultiLineString(lines) => {
                    for line in lines {
                        Self::add_line(&mut vertices, line, projection);
                    }
                },
            }
        }

        vertices
    }

    #[allow(clippy::cast_possible_truncation)]
    fn add_line(vertices: &mut Vec<f32>, points: &[GeoCoord], projection: &dyn Projection) {
        for i in 0..points.len().saturating_sub(1) {
            let p1 = projection.project(points[i]);
            let p2 = projection.project(points[i + 1]);

            vertices.push(p1.x as f32);
            vertices.push(p1.y as f32);
            vertices.push(p2.x as f32);
            vertices.push(p2.y as f32);
        }
    }
}
