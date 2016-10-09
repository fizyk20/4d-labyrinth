use super::primitive::Color;
use super::geometry::Vector;

#[derive(Clone, Copy)]
pub struct GliumVertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4]
}

impl GliumVertex {
    pub fn new(p: Vector, n: Vector, c: Color) -> GliumVertex {
        GliumVertex {
            position: [p.x() as f32, p.y() as f32, p.z() as f32],
            normal: [n.x() as f32, n.y() as f32, n.z() as f32],
            color: [c.r() as f32, c.g() as f32, c.b() as f32, c.a() as f32]
        }
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }
}

implement_vertex!(GliumVertex, position, normal, color);

pub struct VertexInfo {
    vertices: Vec<GliumVertex>,
    indices: Vec<u32>
}

impl VertexInfo {
    pub fn new(v: Vec<GliumVertex>, i: Vec<u32>) -> VertexInfo {
        VertexInfo {
            vertices: v,
            indices: i
        }
    }

    pub fn vertices(&self) -> Vec<GliumVertex> {
        self.vertices.clone()
    }

    pub fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}

pub const VERTEX_SHADER: &'static str = r#"
#version 140

in vec3 position;
in vec3 normal;
in vec4 color;

out vec3 v_normal;
out vec4 v_color;

uniform mat4 matrix;

void main() {
    v_normal = normal;
    v_color = color;
    gl_Position = matrix * vec4(position, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &'static str = r#"
#version 140

in vec3 v_normal;
in vec4 v_color;
out vec4 color;
uniform vec3 u_light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color3 = normalize(vec3(v_color[0], v_color[1], v_color[2]))*0.1;
    vec4 dark_color = vec4(dark_color3, v_color[3]);
    color = vec4(mix(dark_color, v_color, abs(brightness)));
}
"#;
