/// Vertex shader for world map rendering
pub const WORLD_VERTEX: &str = r"#version 300 es
precision highp float;

layout(location = 0) in vec2 a_position;

uniform vec2 u_resolution;

void main() {
    vec2 normalized = a_position / u_resolution;
    vec2 clip = normalized * 2.0 - 1.0;
    gl_Position = vec4(clip.x, -clip.y, 0.0, 1.0);
}
";

/// Fragment shader for world map rendering
pub const WORLD_FRAGMENT: &str = r"#version 300 es
precision highp float;

uniform vec4 u_color;

out vec4 fragColor;

void main() {
    fragColor = u_color;
}
";

/// Vertex shader for marker rendering with instancing
pub const MARKER_VERTEX: &str = r"#version 300 es
precision highp float;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_instance_pos;
layout(location = 2) in float a_intensity;
layout(location = 3) in float a_phase;
layout(location = 4) in vec4 a_color;

uniform mat4 u_projection;
uniform float u_time;
uniform float u_marker_size;

out vec4 v_color;
out vec2 v_uv;
out float v_intensity;

void main() {
    float pulse = 0.8 + 0.2 * sin(u_time + a_phase);
    float size = u_marker_size * pulse;

    vec2 pos = a_instance_pos + a_position * size;
    gl_Position = u_projection * vec4(pos, 0.0, 1.0);

    v_color = a_color;
    v_uv = a_position;
    v_intensity = a_intensity;
}
";

/// Fragment shader for marker rendering with glow effect
pub const MARKER_FRAGMENT: &str = r"#version 300 es
precision highp float;

in vec4 v_color;
in vec2 v_uv;
in float v_intensity;

out vec4 fragColor;

void main() {
    float dist = length(v_uv);

    if (dist > 1.0) {
        discard;
    }

    float alpha = smoothstep(1.0, 0.0, dist) * v_intensity;
    fragColor = vec4(v_color.rgb, v_color.a * alpha);
}
";
