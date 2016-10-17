use super::geometry::{Vector, Hyperplane};
use super::shader::{GliumVertex, VertexInfo};
use std::ops;
use std::convert::From;

#[derive(Clone, Copy)]
pub struct Color(Vector);

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color(Vector::new(r, g, b, 1.0))
    }

    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color(Vector::new(r, g, b, a))
    }

    pub fn r(&self) -> f64 {
        self.0.x()
    }

    pub fn g(&self) -> f64 {
        self.0.y()
    }

    pub fn b(&self) -> f64 {
        self.0.z()
    }

    pub fn a(&self) -> f64 {
        self.0.w()
    }
}

impl ops::Deref for Color {
    type Target = Vector;

    fn deref(&self) -> &Vector {
        &self.0
    }
}

impl From<Vector> for Color {
    fn from(v: Vector) -> Color {
        Color::rgba(v.x(), v.y(), v.z(), v.w())
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    point: Vector,
    color: Color,
}

impl Vertex {
    pub fn new(p: Vector, c: Color) -> Vertex {
        Vertex {
            point: p,
            color: c,
        }
    }

    pub fn point(&self) -> Vector {
        self.point
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Clone, Copy)]
pub enum Primitive {
    Point(Vertex),
    Line(Vertex, Vertex),
    Triangle(Vertex, Vertex, Vertex),
    Quad(Vertex, Vertex, Vertex, Vertex),
    Tetra(Vertex, Vertex, Vertex, Vertex),
}

const EPSILON: f64 = 1e-15;

impl Primitive {
    pub fn intersect(&self, hplane: Hyperplane) -> Option<Primitive> {
        match *self {
            Primitive::Point(v) => Primitive::intersect_point(v, hplane),
            Primitive::Line(v1, v2) => Primitive::intersect_line(v1, v2, hplane),
            Primitive::Triangle(v1, v2, v3) => Primitive::intersect_triangle(v1, v2, v3, hplane),
            Primitive::Quad(v1, v2, v3, v4) => Primitive::intersect_quad(v1, v2, v3, v4, hplane),
            Primitive::Tetra(v1, v2, v3, v4) => Primitive::intersect_tetra(v1, v2, v3, v4, hplane),
        }
    }

    fn intersect_point(p: Vertex, hplane: Hyperplane) -> Option<Primitive> {
        if hplane.dot(p.point()).abs() < EPSILON {
            Some(Primitive::Point(p))
        } else {
            None
        }
    }

    fn intersect_line(p1: Vertex, p2: Vertex, hplane: Hyperplane) -> Option<Primitive> {
        let dot1 = hplane.dot(p1.point());
        let dot2 = hplane.dot(p2.point());

        if dot1.abs() < EPSILON && dot2.abs() < EPSILON {
            Some(Primitive::Line(p1, p2))
        } else if dot1.abs() < EPSILON {
            Some(Primitive::Point(p1))
        } else if dot2.abs() < EPSILON {
            Some(Primitive::Point(p2))
        } else if dot1 * dot2 > 0.0 {
            None
        } else {
            let coeff1 = dot2 / (dot2 - dot1);
            let coeff2 = -dot1 / (dot2 - dot1);
            let pos = p1.point() * coeff1 + p2.point() * coeff2;
            let col = (*p1.color()) * coeff1 + (*p2.color()) * coeff2;
            Some(Primitive::Point(Vertex::new(pos, From::from(col))))
        }
    }

    fn intersect_triangle(p1: Vertex,
                          p2: Vertex,
                          p3: Vertex,
                          hplane: Hyperplane)
                          -> Option<Primitive> {
        let mut tmp = Vec::new();

        // collect intersections of the three sides of the triangle

        if let Some(prim) = Primitive::intersect_line(p1, p2, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p2, p3, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p3, p1, hplane) {
            tmp.push(prim);
        }

        match tmp.len() {
            // no intersections - return None
            0 => None,
            // one intersection - return it (should never happen, actually)
            1 => Some(tmp[0]),
            // two intersections - should be 2 points
            2 => {
                if let (Primitive::Point(v1), Primitive::Point(v2)) = (tmp[0], tmp[1]) {
                    if v1.point() == v2.point() {
                        Some(Primitive::Point(v1))
                    } else {
                        Some(Primitive::Line(v1, v2))
                    }
                } else {
                    unreachable!()
                }
            }
            // 3 intersections - either 3 lines, or a line and 2 points
            _ => {
                if let (Primitive::Line(_, _), Primitive::Line(_, _), Primitive::Line(_, _)) =
                       (tmp[0], tmp[1], tmp[2]) {
                    Some(Primitive::Triangle(p1, p2, p3))
                } else {
                    // a line and 2 points - find the line and return it
                    if let Some(l) = tmp.iter().find(|&&x| if let Primitive::Line(_, _) = x {
                        true
                    } else {
                        false
                    }) {
                        Some(*l)
                    } else {
                        // this case should be impossible
                        unreachable!()
                    }
                }
            }
        }
    }

    fn intersect_quad(p1: Vertex,
                      p2: Vertex,
                      p3: Vertex,
                      p4: Vertex,
                      hplane: Hyperplane)
                      -> Option<Primitive> {
        let mut tmp = Vec::new();

        if let Some(prim) = Primitive::intersect_line(p1, p2, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p2, p3, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p3, p4, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p4, p1, hplane) {
            tmp.push(prim);
        }

        match tmp.len() {
            0 => None,
            2 => {
                if let (Primitive::Point(v1), Primitive::Point(v2)) = (tmp[0], tmp[1]) {
                    if v1.point() == v2.point() {
                        Some(Primitive::Point(v1))
                    } else {
                        Some(Primitive::Line(v1, v2))
                    }
                } else {
                    unreachable!()
                }
            }
            // a line and 2 points - find the line and return it
            3 => {
                if let Some(l) = tmp.iter().find(|&&x| if let Primitive::Line(_, _) = x {
                    true
                } else {
                    false
                }) {
                    Some(*l)
                } else {
                    // this case (no line amongst the 3 intersections) should be impossible
                    unreachable!()
                }
            }
            4 => Some(Primitive::Quad(p1, p2, p3, p4)),
            _ => unreachable!(),
        }
    }

    fn intersect_tetra(p1: Vertex,
                       p2: Vertex,
                       p3: Vertex,
                       p4: Vertex,
                       hplane: Hyperplane)
                       -> Option<Primitive> {
        let mut tmp = Vec::new();

        if let Some(prim) = Primitive::intersect_line(p1, p2, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p2, p3, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p3, p4, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p1, p4, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p1, p3, hplane) {
            tmp.push(prim);
        }

        if let Some(prim) = Primitive::intersect_line(p2, p4, hplane) {
            tmp.push(prim);
        }

        match tmp.len() {
            0 => None,
            3 => {
                if let (Primitive::Point(v1), Primitive::Point(v2), Primitive::Point(v3)) = (tmp[0],
                                                                                             tmp[1],
                                                                                             tmp[2]) {
                    if v1.point() == v2.point() && v2.point() == v3.point() {
                        Some(Primitive::Point(v1))
                    } else {
                        Some(Primitive::Triangle(v1, v2, v3))
                    }
                } else {
                    unreachable!()
                }
            }
            4 => {
                if let (Primitive::Point(v1),
                        Primitive::Point(v2),
                        Primitive::Point(v3),
                        Primitive::Point(v4)) = (tmp[0], tmp[1], tmp[2], tmp[3]) {
                    Some(Primitive::Quad(v1, v2, v3, v4))
                } else {
                    unreachable!()
                }
            }
            // a line and 4 points - find the line and return it
            5 => {
                if let Some(l) = tmp.iter().find(|&&x| if let Primitive::Line(_, _) = x {
                    true
                } else {
                    false
                }) {
                    Some(*l)
                } else {
                    // this case (no line amongst the 5 intersections) should be impossible
                    unreachable!()
                }
            }
            6 => {
                let mut tmp2 = Vec::new();
                for p in tmp.iter() {
                    if let Primitive::Point(v) = *p {
                        tmp2.push(v);
                    }
                }
                if tmp2.len() == 0 {
                    Some(Primitive::Tetra(p1, p2, p3, p4))
                } else if tmp2.len() == 3 {
                    Some(Primitive::Triangle(tmp2[0], tmp2[1], tmp2[2]))
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }

    fn vertexinfo_point(p1: Vertex) -> VertexInfo {
        VertexInfo::new(vec![GliumVertex::new(p1.point(),
                                              Vector::new(0.0, 0.0, 0.0, 0.0),
                                              p1.color())],
                        vec![0])
    }

    fn vertexinfo_line(p1: Vertex, p2: Vertex) -> VertexInfo {
        VertexInfo::new(vec![GliumVertex::new(p1.point(),
                                              Vector::new(0.0, 0.0, 0.0, 0.0),
                                              p1.color()),
                             GliumVertex::new(p2.point(),
                                              Vector::new(0.0, 0.0, 0.0, 0.0),
                                              p2.color())],
                        vec![0, 1])
    }

    fn vertexinfo_triangle(p1: Vertex, p2: Vertex, p3: Vertex) -> VertexInfo {
        let v1 = p2.point() - p1.point();
        let v2 = p3.point() - p1.point();
        let normal = Vector::cross3(v2, v1).normalized();
        VertexInfo::new(vec![GliumVertex::new(p1.point(), normal, p1.color()),
                             GliumVertex::new(p2.point(), normal, p2.color()),
                             GliumVertex::new(p3.point(), normal, p3.color())],
                        vec![0, 1, 2])
    }

    fn vertexinfo_quad(p1: Vertex, p2: Vertex, p3: Vertex, p4: Vertex) -> VertexInfo {
        let v1 = (p2.point() - p1.point()).normalized();
        let v2 = (p3.point() - p1.point()).normalized();
        let v3 = (p4.point() - p1.point()).normalized();
        let dot = [v1.dot(v2), v1.dot(v3), v2.dot(v3)];
        let mindot = if dot[0] < dot[1] { 0 } else { 1 };
        let mindot = if dot[mindot] < dot[2] { mindot } else { 2 };

        let mut indices = Vec::new();

        match mindot {
            0 => {
                indices.push(0);
                indices.push(1);
                indices.push(2);
                indices.push(1);
                indices.push(2);
                indices.push(3);
            }
            1 => {
                indices.push(0);
                indices.push(1);
                indices.push(3);
                indices.push(1);
                indices.push(3);
                indices.push(2);
            }
            2 => {
                indices.push(0);
                indices.push(2);
                indices.push(3);
                indices.push(2);
                indices.push(3);
                indices.push(1);
            }
            _ => unreachable!(),
        };

        let normal = Vector::cross3(v2, v1).normalized();

        VertexInfo::new(vec![GliumVertex::new(p1.point(), normal, p1.color()),
                             GliumVertex::new(p2.point(), normal, p2.color()),
                             GliumVertex::new(p3.point(), normal, p3.color()),
                             GliumVertex::new(p4.point(), normal, p4.color())],
                        indices)
    }

    fn vertexinfo_tetra(p1: Vertex, p2: Vertex, p3: Vertex, p4: Vertex) -> VertexInfo {
        let v21 = p2.point() - p1.point();
        let v31 = p3.point() - p1.point();
        let v41 = p4.point() - p1.point();
        let v23 = p2.point() - p3.point();
        let v43 = p4.point() - p3.point();
        let normal1 = Vector::cross3(v31, v21).normalized();
        let normal2 = Vector::cross3(v41, v31).normalized();
        let normal3 = Vector::cross3(v21, v41).normalized();
        let normal4 = Vector::cross3(v43, v23).normalized();
        VertexInfo::new(vec![GliumVertex::new(p1.point(), normal1, p1.color()),
                             GliumVertex::new(p2.point(), normal1, p2.color()),
                             GliumVertex::new(p3.point(), normal1, p3.color()),
                             GliumVertex::new(p1.point(), normal2, p1.color()),
                             GliumVertex::new(p3.point(), normal2, p3.color()),
                             GliumVertex::new(p4.point(), normal2, p4.color()),
                             GliumVertex::new(p1.point(), normal3, p1.color()),
                             GliumVertex::new(p4.point(), normal3, p4.color()),
                             GliumVertex::new(p2.point(), normal3, p2.color()),
                             GliumVertex::new(p3.point(), normal4, p3.color()),
                             GliumVertex::new(p2.point(), normal4, p2.color()),
                             GliumVertex::new(p4.point(), normal4, p4.color())],
                        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
    }

    pub fn get_vertexinfo(&self) -> VertexInfo {
        match *self {
            Primitive::Point(p1) => Primitive::vertexinfo_point(p1),
            Primitive::Line(p1, p2) => Primitive::vertexinfo_line(p1, p2),
            Primitive::Triangle(p1, p2, p3) => Primitive::vertexinfo_triangle(p1, p2, p3),
            Primitive::Quad(p1, p2, p3, p4) => Primitive::vertexinfo_quad(p1, p2, p3, p4),
            Primitive::Tetra(p1, p2, p3, p4) => Primitive::vertexinfo_tetra(p1, p2, p3, p4),
        }
    }

    pub fn map<F: Fn(Vertex) -> Vertex>(self, f: F) -> Primitive {
        match self {
            Primitive::Point(v1) => Primitive::Point(f(v1)),
            Primitive::Line(v1, v2) => Primitive::Line(f(v1), f(v2)),
            Primitive::Triangle(v1, v2, v3) => Primitive::Triangle(f(v1), f(v2), f(v3)),
            Primitive::Tetra(v1, v2, v3, v4) => Primitive::Tetra(f(v1), f(v2), f(v3), f(v4)),
            Primitive::Quad(v1, v2, v3, v4) => Primitive::Quad(f(v1), f(v2), f(v3), f(v4)),
        }
    }
}
