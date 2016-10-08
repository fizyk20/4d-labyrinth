#![feature(conservative_impl_trait)]

extern crate graph4d;
extern crate glium;
extern crate regex;

mod objects;
mod levels;

use objects::{Player, Wall, GameObject, Collidable};

use glium::{DisplayBuild, Surface};
use glium::glutin::{ElementState, VirtualKeyCode};
use graph4d::geometry::{Vector, Matrix};
use graph4d::primitive::Color;
use std::collections::HashSet;
use std::time::SystemTime;

pub struct KeyboardState {
    pressed_keys: HashSet<VirtualKeyCode>
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            pressed_keys: HashSet::new()
        }
    }

    fn pressed(&mut self, key: VirtualKeyCode) {
        self.pressed_keys.insert(key);
    }

    fn released(&mut self, key: VirtualKeyCode) {
        self.pressed_keys.remove(&key);
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
}

fn generate_level() -> Vec<Wall> {
    vec! [
        Wall::new(Vector::new(3.0, 0.0, 4.0, 0.0), Vector::new(0.0, 5.0, 10.0, 5.0))
    ]
}

fn main() {
    let display = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let mut renderer = graph4d::renderer::Renderer::new(&display);
    let mut t = 0.0;
    let mut keyboard = KeyboardState::new();

    let mut player = Player::new();
    player.go(Vector::new(0.0, 0.0, -10.0, 0.0));
    let walls = generate_level();

    let mut now = SystemTime::now();

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        player.draw(&mut renderer);
        renderer.set_color(Color::rgba(1.0, 0.0, 0.0, 0.5));
        renderer.push_matrix();
        renderer.rotate_yz(t);
        renderer.rotate_xw(t*0.67);
        t += 0.01;
        renderer.apply_matrix(Matrix::translation(Vector::new(0.0, 0.0, 3.0, 0.0)));
        renderer.tesseract(1.0);
        renderer.pop_matrix();
        renderer.render(&display, &player, &mut target);
        for w in walls.iter() {
            w.draw(&mut renderer);
        }
        target.finish().unwrap();

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
             match ev {
                 glium::glutin::Event::Closed => return,   // the window has been closed by the user
                 glium::glutin::Event::KeyboardInput(state, _, Some(key)) => match state {
                     ElementState::Pressed => keyboard.pressed(key),
                     ElementState::Released => keyboard.released(key)
                 },
                 _ => ()
             }
        }

        let frame_time = now.elapsed().unwrap();
        let frame_time = frame_time.as_secs() as f64 + (frame_time.subsec_nanos() as f64) / 1e9;
        now = SystemTime::now();

        let action = player.handle_input(&keyboard, frame_time);
        let mut let_move = true;
        for w in walls.iter() {
            if w.collides(&action) {
                let_move = false;
            }
        }
        if let_move {
            player.perform_action(action);
        }
    }
}
