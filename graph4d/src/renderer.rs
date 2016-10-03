use super::camera::Camera;
use super::primitive::{Primitive, Vertex, Color};
use super::geometry::{Vector, Matrix};
use super::shader::{VERTEX_SHADER, FRAGMENT_SHADER};
use glium;
use glium::{Surface, Program, VertexBuffer, IndexBuffer, Blend};
use glium::index::PrimitiveType;
use glium::backend::Facade;

pub struct Renderer {
    current_transform: Matrix,
    matrix_stack: Vec<Matrix>,
    prim_queue: Vec<Primitive>,
    current_color: Color,
    shader: Program
}

impl Renderer {
    pub fn new<F: Facade>(display: &F) -> Renderer {
        Renderer {
            current_transform: Matrix::identity(),
            matrix_stack: Vec::new(),
            prim_queue: Vec::new(),
            current_color: Color::rgb(1.0, 1.0, 1.0),
            shader: Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap()
        }
    }

    pub fn push_matrix(&mut self) {
        self.matrix_stack.push(self.current_transform);
    }

    pub fn pop_matrix(&mut self) {
        if let Some(matrix) = self.matrix_stack.pop() {
            self.current_transform = matrix;
        }
    }

    pub fn apply_matrix(&mut self, matrix: Matrix) {
        self.current_transform = matrix * self.current_transform;
    }

    pub fn rotate_xy(&mut self, phi: f64) {
        self.apply_matrix(Matrix::rotation_xy(phi));
    }

    pub fn rotate_xz(&mut self, phi: f64) {
        self.apply_matrix(Matrix::rotation_xz(phi));
    }

    pub fn rotate_xw(&mut self, phi: f64) {
        self.apply_matrix(Matrix::rotation_xw(phi));
    }

    pub fn rotate_yz(&mut self, phi: f64) {
        self.apply_matrix(Matrix::rotation_yz(phi));
    }

    pub fn rotate_yw(&mut self, phi: f64) {
        self.apply_matrix(Matrix::rotation_yw(phi));
    }

    pub fn rotate_zw(&mut self, phi: f64) {
        self.apply_matrix(Matrix::rotation_zw(phi));
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn tetrahedron(&mut self, v1: Vector, v2: Vector, v3: Vector, v4: Vector) {
        self.prim_queue.push(Primitive::Tetra(
            Vertex::new(self.current_transform * v1, self.current_color),
            Vertex::new(self.current_transform * v2, self.current_color),
            Vertex::new(self.current_transform * v3, self.current_color),
            Vertex::new(self.current_transform * v4, self.current_color)
        ));
    }

    pub fn cube(&mut self, a: f64) {
        let v = [
            Vector::new(-a/2.0, -a/2.0, -a/2.0, 0.0),
            Vector::new(-a/2.0, -a/2.0,  a/2.0, 0.0),
            Vector::new(-a/2.0, -a/2.0, -a/2.0, 0.0),
            Vector::new(-a/2.0,  a/2.0,  a/2.0, 0.0),
            Vector::new( a/2.0,  a/2.0, -a/2.0, 0.0),
            Vector::new( a/2.0, -a/2.0,  a/2.0, 0.0),
            Vector::new( a/2.0,  a/2.0, -a/2.0, 0.0),
            Vector::new( a/2.0,  a/2.0,  a/2.0, 0.0)
        ];

        self.tetrahedron(v[0], v[2], v[1], v[4]);
        self.tetrahedron(v[5], v[7], v[4], v[1]);
        self.tetrahedron(v[6], v[4], v[7], v[2]);
        self.tetrahedron(v[3], v[1], v[2], v[7]);
        self.tetrahedron(v[2], v[7], v[1], v[4]);
    }

    fn cube_vertex_array(&mut self, vertices: &[Vector], indices: [usize; 8]) {
        let mut v = Vec::new();
        for i in indices.iter() {
            v.push(vertices[*i]);
        }
        self.tetrahedron(v[0], v[2], v[1], v[4]);
        self.tetrahedron(v[5], v[7], v[4], v[1]);
        self.tetrahedron(v[6], v[4], v[7], v[2]);
        self.tetrahedron(v[3], v[1], v[2], v[7]);
        self.tetrahedron(v[2], v[7], v[1], v[4]);
    }

    pub fn tesseract(&mut self, a: f64) {
        let v = [
            Vector::new(-a/2.0, -a/2.0, -a/2.0, -a/2.0),
            Vector::new(-a/2.0, -a/2.0, -a/2.0,  a/2.0),
            Vector::new(-a/2.0, -a/2.0,  a/2.0, -a/2.0),
            Vector::new(-a/2.0, -a/2.0,  a/2.0,  a/2.0),
            Vector::new(-a/2.0,  a/2.0, -a/2.0, -a/2.0),
            Vector::new(-a/2.0,  a/2.0, -a/2.0,  a/2.0),
            Vector::new(-a/2.0,  a/2.0,  a/2.0, -a/2.0),
            Vector::new(-a/2.0,  a/2.0,  a/2.0,  a/2.0),
            Vector::new( a/2.0, -a/2.0, -a/2.0, -a/2.0),
            Vector::new( a/2.0, -a/2.0, -a/2.0,  a/2.0),
            Vector::new( a/2.0, -a/2.0,  a/2.0, -a/2.0),
            Vector::new( a/2.0, -a/2.0,  a/2.0,  a/2.0),
            Vector::new( a/2.0,  a/2.0, -a/2.0, -a/2.0),
            Vector::new( a/2.0,  a/2.0, -a/2.0,  a/2.0),
            Vector::new( a/2.0,  a/2.0,  a/2.0, -a/2.0),
            Vector::new( a/2.0,  a/2.0,  a/2.0,  a/2.0)
        ];

        self.cube_vertex_array(&v, [0,1,2,3,4,5,6,7]);
        self.cube_vertex_array(&v, [8,9,10,11,12,13,14,15]);
        self.cube_vertex_array(&v, [0,1,2,3,8,9,10,11]);
        self.cube_vertex_array(&v, [4,5,6,7,12,13,14,15]);
        self.cube_vertex_array(&v, [0,1,4,5,8,9,12,13]);
        self.cube_vertex_array(&v, [2,3,6,7,10,11,14,15]);
        self.cube_vertex_array(&v, [0,2,4,6,8,10,12,14]);
        self.cube_vertex_array(&v, [1,3,5,7,9,11,13,15]);
    }

    fn get_perspective_matrix<S: Surface>(&self, surface: &S) -> [[f32; 4]; 4] {
        let (width, height) = surface.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn render<F: Facade, C: Camera, S: Surface>(&mut self, facade: &F, camera: C, surface: &mut S) {
        let mut local_queue = Vec::new();
        self.matrix_stack.clear();

        for prim in self.prim_queue.iter() {
            if let Some(prim) = prim.intersect(camera.get_hyperplane()) {
                local_queue.push(prim.map(|v| Vertex::new(camera.calculate_local(v.point()), v.color())));
            }
        }
        self.prim_queue.clear();

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for prim in local_queue.iter() {
            let vertexinfo = prim.get_vertexinfo();
            let base = vertices.len() as u32;
            vertices.append(&mut vertexinfo.vertices());
            indices.append(&mut vertexinfo.indices(base));
        }

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        let vertices_buf = VertexBuffer::new(facade, &vertices).unwrap(); 
        let indices_buf = IndexBuffer::new(facade, PrimitiveType::TrianglesList, &indices).unwrap();
        let matrix = self.get_perspective_matrix(surface);

        surface.draw(&vertices_buf, &indices_buf, &self.shader,
                     &uniform! {
                         matrix: matrix,
                         u_light: [0.0, -1.0, -1.0f32]
                     }, &params).unwrap();

        self.current_transform = Matrix::identity();
    }
}
