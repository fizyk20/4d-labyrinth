extern crate graph4d;
extern crate glium;

use glium::{DisplayBuild, Surface};
use graph4d::geometry::{Vector, Matrix, Hyperplane};
use graph4d::primitive::Color;

struct Camera;

impl graph4d::camera::Camera for Camera {
    fn get_hyperplane(&self) -> Hyperplane {
        Hyperplane::new(Vector::new(0.0, 0.0, 0.0, 1.0), 0.0)
    }

    fn calculate_local(&self, vec: Vector) -> Vector {
        vec
    }
}

fn main() {
    let display = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let mut renderer = graph4d::renderer::Renderer::new(&display);
    let mut t = 0.0;

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.1, 1.0), 1.0);
        renderer.set_color(Color::rgba(1.0, 0.0, 0.0, 0.5));
        renderer.rotate_yz(t);
        renderer.rotate_xw(t*0.67);
        t += 0.01;
        renderer.apply_matrix(Matrix::translation(Vector::new(0.0, 0.0, 3.0, 0.0)));
        renderer.tesseract(1.0);
        renderer.render(&display, Camera, &mut target);
        target.finish().unwrap();

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
             match ev {
                 glium::glutin::Event::Closed => return,   // the window has been closed by the user
                 _ => ()
             }
        }
    }
}
