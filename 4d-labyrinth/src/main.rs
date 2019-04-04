mod levels;
mod objects;

use levels::Level;

use glium::glutin::{ElementState, VirtualKeyCode};
use glium::{DisplayBuild, Surface};
use std::collections::HashSet;
use std::time::SystemTime;

pub struct KeyboardState {
    pressed_keys: HashSet<VirtualKeyCode>,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            pressed_keys: HashSet::new(),
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

fn main() {
    let levels = ["level1.dat", "level2.dat"];

    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();
    let mut renderer = graph4d::renderer::Renderer::new(&display);
    let mut keyboard = KeyboardState::new();

    let mut now = SystemTime::now();
    let mut level_num = 1u8;

    for level_file in &levels {
        let mut level = Level::from_file(level_file).unwrap();

        loop {
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            for object in level.game_objects() {
                object.draw(&mut renderer);
            }
            renderer.render(&display, &*level.player(), &mut target);
            target.finish().unwrap();

            // listing the events produced by the window and waiting to be received
            for ev in display.poll_events() {
                match ev {
                    glium::glutin::Event::Closed => return, // the window has been closed by the user
                    glium::glutin::Event::KeyboardInput(state, _, Some(key)) => match state {
                        ElementState::Pressed => keyboard.pressed(key),
                        ElementState::Released => keyboard.released(key),
                    },
                    _ => (),
                }
            }

            let frame_time = now.elapsed().unwrap();
            let frame_time = frame_time.as_secs() as f64 + (frame_time.subsec_nanos() as f64) / 1e9;
            now = SystemTime::now();

            let action = level.player().handle_input(&keyboard, frame_time);
            let mut let_move = true;
            for c in level.collidables() {
                if c.collides(&action) {
                    let_move = false;
                }
            }
            if level.wins(&action) {
                println!("Level {} completed!", level_num);
                level_num += 1;
                break;
            }
            if let_move {
                level.player().perform_action(action);
            }
        }
    }
}
