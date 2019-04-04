use super::KeyboardState;
use glium::glutin::VirtualKeyCode;
use graph4d::camera::Camera;
use graph4d::geometry::{Hyperplane, Matrix, Vector};
use graph4d::primitive::Color;
use graph4d::renderer::Renderer;
use std::f64::consts::PI;

pub trait GameObject {
    fn draw(&self, renderer: &mut Renderer);
}

pub trait Collidable {
    fn collides(&self, action: &AdditionalAction) -> bool;
}

const SIZE: f64 = 0.4;

pub enum AdditionalAction {
    None,
    MoveTo(Vector),
}

pub struct Player {
    up: Vector,
    front: Vector,
    right: Vector,
    ana: Vector,
    position: Vector,
    orientation: Matrix,
}

impl Player {
    pub fn new() -> Player {
        Player {
            up: Vector::new(0.0, 1.0, 0.0, 0.0),
            front: Vector::new(0.0, 0.0, 1.0, 0.0),
            right: Vector::new(1.0, 0.0, 0.0, 0.0),
            ana: Vector::new(0.0, 0.0, 0.0, 1.0),
            position: Vector::new(0.0, 0.0, 0.0, 0.0),
            orientation: Matrix::identity(),
        }
    }

    pub fn go(&mut self, dir: Vector) {
        self.position = self.position
            + self.up * dir.y()
            + self.right * dir.x()
            + self.front * dir.z()
            + self.ana * dir.w();
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

    pub fn handle_input(&mut self, keyboard: &KeyboardState, time: f64) -> AdditionalAction {
        let vel = 3.2;
        let angvel = 1.05;

        let distance = vel * time;
        let angle = angvel * time;

        let old_position = self.position;

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

        if self.position != old_position {
            let result = AdditionalAction::MoveTo(self.position);
            self.position = old_position;
            result
        } else {
            AdditionalAction::None
        }
    }

    pub fn perform_action(&mut self, action: AdditionalAction) {
        match action {
            AdditionalAction::MoveTo(pos) => self.position = pos,
            _ => (),
        }
    }
}

impl GameObject for Player {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.set_color(Color::rgb(0.6, 0.6, 0.0));
        renderer.push_matrix();
        renderer.apply_matrix(self.orientation);
        renderer.apply_matrix(Matrix::translation(self.position));
        renderer.tesseract(SIZE);
        renderer.pop_matrix();
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

pub struct Wall {
    middle: Vector,
    size: Vector,
    transformation_matrix: Matrix,
}

impl Wall {
    pub fn new(middle: Vector, size: Vector) -> Wall {
        let pi_2 = PI / 2.0;
        let rotation_matrix = if size.w() == 0.0 {
            Matrix::identity()
        } else if size.x() == 0.0 {
            Matrix::rotation_xw(pi_2)
        } else if size.y() == 0.0 {
            Matrix::rotation_yw(pi_2)
        } else if size.z() == 0.0 {
            Matrix::rotation_zw(pi_2)
        } else {
            panic!("Wall without a zero dimension!")
        };
        let scale_matrix = if size.w() == 0.0 {
            Matrix::scale(size.x(), size.y(), size.z(), 1.0)
        } else if size.x() == 0.0 {
            Matrix::scale(size.w(), size.y(), size.z(), 1.0)
        } else if size.y() == 0.0 {
            Matrix::scale(size.x(), size.w(), size.z(), 1.0)
        } else if size.z() == 0.0 {
            Matrix::scale(size.x(), size.y(), size.w(), 1.0)
        } else {
            panic!("Wall without a zero dimension!")
        };
        let translation_matrix = Matrix::translation(middle);
        Wall {
            middle: middle,
            size: size,
            transformation_matrix: translation_matrix * rotation_matrix * scale_matrix,
        }
    }
}

impl GameObject for Wall {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.set_color(Color::rgba(0.6, 0.6, 0.6, 0.2));
        renderer.push_matrix();
        renderer.apply_matrix(self.transformation_matrix);
        renderer.cube(1.0);
        renderer.pop_matrix();
    }
}

impl Collidable for Wall {
    fn collides(&self, action: &AdditionalAction) -> bool {
        match *action {
            AdditionalAction::MoveTo(pos) => {
                let x = (pos.x() - self.middle.x()).abs() < (self.size.x() + SIZE) / 2.0;
                let y = (pos.y() - self.middle.y()).abs() < (self.size.y() + SIZE) / 2.0;
                let z = (pos.z() - self.middle.z()).abs() < (self.size.z() + SIZE) / 2.0;
                let w = (pos.w() - self.middle.w()).abs() < (self.size.w() + SIZE) / 2.0;

                x && y && z && w
            }
            _ => false,
        }
    }
}

pub struct Target {
    position: Vector,
    size: f64,
}

impl Target {
    pub fn new(position: Vector, size: f64) -> Target {
        Target {
            position: position,
            size: size,
        }
    }
}

impl GameObject for Target {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.set_color(Color::rgba(0.0, 0.2, 1.0, 0.4));
        renderer.push_matrix();
        renderer.apply_matrix(Matrix::translation(self.position));
        renderer.tesseract(self.size);
        renderer.pop_matrix();
    }
}

impl Collidable for Target {
    fn collides(&self, action: &AdditionalAction) -> bool {
        match *action {
            AdditionalAction::MoveTo(pos) => {
                let x = (pos.x() - self.position.x()).abs() < (self.size + SIZE) / 2.0;
                let y = (pos.y() - self.position.y()).abs() < (self.size + SIZE) / 2.0;
                let z = (pos.z() - self.position.z()).abs() < (self.size + SIZE) / 2.0;
                let w = (pos.w() - self.position.w()).abs() < (self.size + SIZE) / 2.0;

                x && y && z && w
            }
            _ => false,
        }
    }
}
