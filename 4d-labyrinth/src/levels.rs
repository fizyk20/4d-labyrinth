use super::objects::{AdditionalAction, Collidable, GameObject, Player, Target, Wall};
use graph4d::geometry::Vector;
use regex::Regex;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::iter::{once, Iterator};
use std::path::Path;

enum ReadState {
    NoLines,
    OneLine(String),
    TwoLines(String, String),
}

enum LineResult {
    Target(Target),
    Walls(Vec<Wall>),
    Error,
}

pub struct Level {
    walls: Vec<Wall>,
    target: Target,
    player: Player,
}

impl Level {
    pub fn from_file<P: AsRef<Path> + Display>(file_name: P) -> Result<Level> {
        let f = File::open(file_name)?;
        let reader = BufReader::new(&f);
        let mut state = ReadState::NoLines;
        let mut target = None;
        let mut walls = Vec::new();

        for line in reader.lines() {
            match state {
                ReadState::NoLines => state = ReadState::OneLine(line?),
                ReadState::OneLine(s) => state = ReadState::TwoLines(s, line?),
                ReadState::TwoLines(s1, s2) => {
                    let s3 = line?;
                    match Level::process_lines(s1, s2, s3) {
                        LineResult::Target(t) => target = Some(t),
                        LineResult::Walls(mut w) => walls.append(&mut w),
                        LineResult::Error => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Invalid level definition",
                            ))
                        }
                    }
                    state = ReadState::NoLines;
                }
            }
        }
        Ok(Level {
            player: Player::new(),
            target: target.unwrap(),
            walls: walls,
        })
    }

    fn process_lines(s1: String, s2: String, s3: String) -> LineResult {
        let coord_line = Regex::new(r"^(?P<x>-?\d+(\.\d+)?)\s+(?P<y>-?\d+(\.\d+)?)\s+(?P<z>-?\d+(\.\d+)?)\s+(?P<w>-?\d+(\.\d+)?)").unwrap();
        let cap1 = coord_line.captures(&s1).unwrap();
        let cap2 = coord_line.captures(&s2).unwrap();
        let (x1, y1, z1, w1): (f64, f64, f64, f64) = (
            cap1.name("x").unwrap().parse().unwrap(),
            cap1.name("y").unwrap().parse().unwrap(),
            cap1.name("z").unwrap().parse().unwrap(),
            cap1.name("w").unwrap().parse().unwrap(),
        );
        let (x2, y2, z2, w2): (f64, f64, f64, f64) = (
            cap2.name("x").unwrap().parse().unwrap(),
            cap2.name("y").unwrap().parse().unwrap(),
            cap2.name("z").unwrap().parse().unwrap(),
            cap2.name("w").unwrap().parse().unwrap(),
        );

        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        let (z1, z2) = if z1 < z2 { (z1, z2) } else { (z2, z1) };
        let (w1, w2) = if w1 < w2 { (w1, w2) } else { (w2, w1) };

        if s3 == "T" {
            return LineResult::Target(Target::new(
                Vector::new(
                    (x1 + x2) / 2.0,
                    (y1 + y2) / 2.0,
                    (z1 + z2) / 2.0,
                    (w1 + w2) / 2.0,
                ),
                x2 - x1,
            ));
        }

        let mut walls = Vec::new();

        for c in s3.chars() {
            let (middle, size) = match c {
                'x' => (
                    Vector::new(x1, (y2 + y1) / 2.0, (z2 + z1) / 2.0, (w2 + w1) / 2.0),
                    Vector::new(0.0, y2 - y1, z2 - z1, w2 - w1),
                ),
                'X' => (
                    Vector::new(x2, (y2 + y1) / 2.0, (z2 + z1) / 2.0, (w2 + w1) / 2.0),
                    Vector::new(0.0, y2 - y1, z2 - z1, w2 - w1),
                ),
                'y' => (
                    Vector::new((x2 + x1) / 2.0, y1, (z2 + z1) / 2.0, (w2 + w1) / 2.0),
                    Vector::new(x2 - x1, 0.0, z2 - z1, w2 - w1),
                ),
                'Y' => (
                    Vector::new((x2 + x1) / 2.0, y2, (z2 + z1) / 2.0, (w2 + w1) / 2.0),
                    Vector::new(x2 - x1, 0.0, z2 - z1, w2 - w1),
                ),
                'z' => (
                    Vector::new((x2 + x1) / 2.0, (y2 + y1) / 2.0, z1, (w2 + w1) / 2.0),
                    Vector::new(x2 - x1, y2 - y1, 0.0, w2 - w1),
                ),
                'Z' => (
                    Vector::new((x2 + x1) / 2.0, (y2 + y1) / 2.0, z2, (w2 + w1) / 2.0),
                    Vector::new(x2 - x1, y2 - y1, 0.0, w2 - w1),
                ),
                'w' => (
                    Vector::new((x2 + x1) / 2.0, (y2 + y1) / 2.0, (z2 + z1) / 2.0, w1),
                    Vector::new(x2 - x1, y2 - y1, z2 - z1, 0.0),
                ),
                'W' => (
                    Vector::new((x2 + x1) / 2.0, (y2 + y1) / 2.0, (z2 + z1) / 2.0, w2),
                    Vector::new(x2 - x1, y2 - y1, z2 - z1, 0.0),
                ),
                _ => return LineResult::Error,
            };
            let wall = Wall::new(middle, size);
            walls.push(wall);
        }

        LineResult::Walls(walls)
    }

    pub fn game_objects<'a>(&'a mut self) -> impl Iterator<Item = &'a mut GameObject> {
        once(&mut self.player as &mut GameObject)
            .chain(once(&mut self.target as &mut GameObject))
            .chain(self.walls.iter_mut().map(|x| x as &mut GameObject))
    }

    pub fn collidables<'a>(&'a self) -> impl Iterator<Item = &'a Collidable> {
        once(&self.target as &Collidable).chain(self.walls.iter().map(|x| x as &Collidable))
    }

    pub fn wins(&self, action: &AdditionalAction) -> bool {
        self.target.collides(action)
    }

    pub fn player(&mut self) -> &mut Player {
        &mut self.player
    }
}
