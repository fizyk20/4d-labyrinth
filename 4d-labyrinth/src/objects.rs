use super::KeyboardState;
use graph4d::renderer::Renderer;
use graph4d::camera::Camera;
use graph4d::geometry::{Vector, Matrix, Hyperplane};
use graph4d::primitive::Color;
use glium::glutin::VirtualKeyCode;

pub trait GameObject {
    fn draw(&self, renderer: &mut Renderer);
    fn handle_input(&mut self, keyboard: &KeyboardState, time: f64);
}

const SIZE: f64 = 0.2;

pub struct Player {
    up: Vector,
    front: Vector,
    right: Vector,
    ana: Vector,
    position: Vector,
    orientation: Matrix
}

impl Player {
    pub fn new() -> Player {
        Player {
            up: Vector::new(0.0, 1.0, 0.0, 0.0),
            front: Vector::new(0.0, 0.0, 1.0, 0.0),
            right: Vector::new(1.0, 0.0, 0.0, 0.0),
            ana: Vector::new(0.0, 0.0, 0.0, 1.0),
            position: Vector::new(0.0, 0.0, 0.0, 0.0),
            orientation: Matrix::identity()
        }
    }

    pub fn go(&mut self, dir: Vector) {
        self.position = self.position + self.up*dir.y() + self.right*dir.x() + self.front*dir.z() + self.ana*dir.w();
    }

    fn apply_matrix(&mut self, matrix: Matrix) {
        self.up = (matrix * self.up).normalized();
        self.right = (matrix * self.right).normalized();
        self.front = (matrix * self.front).normalized();
        self.ana = (matrix * self.ana).normalized();
        self.orientation = matrix * self.orientation;
    }

    pub fn rotate_xy(&mut self, phi: f64) {
        let matrix = Matrix::rotation(self.right, self.up, phi);
        self.apply_matrix(matrix);
    }

    pub fn rotate_xz(&mut self, phi: f64) {
        let matrix = Matrix::rotation(self.right, self.front, phi);
        self.apply_matrix(matrix);
    }

    pub fn rotate_xw(&mut self, phi: f64) {
        let matrix = Matrix::rotation(self.right, self.ana, phi);
        self.apply_matrix(matrix);
    }

    pub fn rotate_yz(&mut self, phi: f64) {
        let matrix = Matrix::rotation(self.up, self.front, phi);
        self.apply_matrix(matrix);
    }

    pub fn rotate_yw(&mut self, phi: f64) {
        let matrix = Matrix::rotation(self.up, self.ana, phi);
        self.apply_matrix(matrix);
    }

    pub fn rotate_zw(&mut self, phi: f64) {
        let matrix = Matrix::rotation(self.front, self.ana, phi);
        self.apply_matrix(matrix);
    }
}

impl GameObject for Player {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.set_color(Color::rgb(0.6, 0.6, 0.0));
        renderer.push_matrix();
        renderer.apply_matrix(self.orientation);
        renderer.apply_matrix(Matrix::translation(self.position));
        renderer.tesseract(2.0*SIZE);
        renderer.pop_matrix();
    }

    fn handle_input(&mut self, keyboard: &KeyboardState, time: f64) {
        let vel = 3.2;
        let angvel = 1.05;

        let distance = vel * time;
        let angle = angvel * time;

        // movements
        if keyboard.is_pressed(VirtualKeyCode::W) {
            self.go(Vector::new(0.0, 0.0, distance, 0.0));
        }
        if keyboard.is_pressed(VirtualKeyCode::S) {
            self.go(Vector::new(0.0, 0.0, -distance, 0.0));
        }
        if keyboard.is_pressed(VirtualKeyCode::A) {
            self.go(Vector::new(-distance, 0.0, 0.0, 0.0));
        }
        if keyboard.is_pressed(VirtualKeyCode::D) {
            self.go(Vector::new(distance, 0.0, 0.0, 0.0));
        }
        if keyboard.is_pressed(VirtualKeyCode::Q) {
            self.go(Vector::new(0.0, distance, 0.0, 0.0));
        }
        if keyboard.is_pressed(VirtualKeyCode::E) {
            self.go(Vector::new(0.0, -distance, 0.0, 0.0));
        }

        // rotations
        if keyboard.is_pressed(VirtualKeyCode::T) {
            self.rotate_xw(angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::G) {
            self.rotate_xw(-angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::F) {
            self.rotate_zw(angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::H) {
            self.rotate_zw(-angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::R) {
            self.rotate_yw(-angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::Y) {
            self.rotate_yw(angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::U) {
            self.rotate_xy(-angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::J) {
            self.rotate_xy(angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::C) {
            self.rotate_xz(angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::V) {
            self.rotate_xz(-angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::B) {
            self.rotate_yz(angle);
        }
        if keyboard.is_pressed(VirtualKeyCode::N) {
            self.rotate_yz(-angle);
        }
    }
}

impl Camera for Player {
    fn get_hyperplane(&self) -> Hyperplane {
        Hyperplane::new(self.ana, -self.ana.dot(self.position))
    }

    fn calculate_local(&self, vec: Vector) -> Vector {
        let dir = vec - self.position - self.orientation * Vector::new(0.0, 0.7, -3.0, 0.0);
        let x = dir.dot(self.right);
        let y = dir.dot(self.up);
        let z = dir.dot(self.front);
        let w = dir.dot(self.ana);
        Vector::new(x, y, z, w)
    }
}

