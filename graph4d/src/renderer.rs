use super::camera::Camera;
use super::primitive::{Primitive, Vertex, Color};
use super::geometry::{Vector, Matrix};

pub struct Renderer {
    current_transform: Matrix,
    matrix_stack: Vec<Matrix>,
    prim_queue: Vec<Primitive>,
    current_color: Color
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            current_transform: Matrix::identity(),
            matrix_stack: Vec::new(),
            prim_queue: Vec::new(),
            current_color: Color::rgb(1.0, 1.0, 1.0)
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

    pub fn render<C: Camera>(&mut self, camera: C) {
        let mut local_queue = Vec::new();
        self.matrix_stack.clear();

        for prim in self.prim_queue.iter() {
            if let Some(prim) = prim.intersect(camera.get_hyperplane()) {
                local_queue.push(prim.map(|v| Vertex::new(camera.calculate_local(v.point()), v.color())));
            }
        }
        self.prim_queue.clear();
    }
}
