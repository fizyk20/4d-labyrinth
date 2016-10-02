use super::geometry::{Vector, Hyperplane};
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
    color: Color
}

impl Vertex {
    pub fn new(p: Vector, c: Color) -> Vertex {
        Vertex {
            point: p,
            color: c
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
    Tetra(Vertex, Vertex, Vertex, Vertex)
}

const EPSILON: f64 = 1e-15;

impl Primitive {
    pub fn intersect(&self, hplane: Hyperplane) -> Option<Primitive> {
        match *self {
            Primitive::Point(v) => Primitive::intersect_point(v, hplane),
            Primitive::Line(v1, v2) => Primitive::intersect_line(v1, v2, hplane),
            Primitive::Triangle(v1, v2, v3) => Primitive::intersect_triangle(v1, v2, v3, hplane),
            Primitive::Quad(v1, v2, v3, v4) => Primitive::intersect_quad(v1, v2, v3, v4, hplane),
            Primitive::Tetra(v1, v2, v3, v4) => Primitive::intersect_tetra(v1, v2, v3, v4, hplane)
        }
    }

    fn intersect_point(p: Vertex, hplane: Hyperplane) -> Option<Primitive> {
        if hplane.dot(p.point()).abs() < EPSILON {
            Some(Primitive::Point(p))
        }
        else {
            None
        }
    }

    fn intersect_line(p1: Vertex, p2: Vertex, hplane: Hyperplane) -> Option<Primitive> {
        let dot1 = hplane.dot(p1.point());
        let dot2 = hplane.dot(p2.point());

        if dot1.abs() < EPSILON && dot2.abs() < EPSILON {
            Some(Primitive::Line(p1, p2))
        }
        else if dot1.abs() < EPSILON {
            Some(Primitive::Point(p1))
        }
        else if dot2.abs() < EPSILON {
            Some(Primitive::Point(p2))
        }
        else if dot1 * dot2 > 0.0 {
            None
        }
        else {
            let coeff1 = dot2/(dot2-dot1);
            let coeff2 = -dot1/(dot2-dot1);
            let pos = p1.point()*coeff1 + p2.point()*coeff2;
            let col = (*p1.color())*coeff1 + (*p2.color())*coeff2;
            Some(Primitive::Point(Vertex::new(pos, From::from(col))))
        }
    }

    fn intersect_triangle(p1: Vertex, p2: Vertex, p3: Vertex, hplane: Hyperplane) -> Option<Primitive> {
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

        if tmp.len() == 0 {
            // no intersections - return None
            None
        }
        else if tmp.len() == 1 {
            // one intersection - return it (should never happen, actually)
            Some(tmp[0])
        }
        else if tmp.len() == 2 {
            // two intersections - should be 2 points
            if let (Primitive::Point(v1), Primitive::Point(v2)) = (tmp[0], tmp[1]) {
                if v1.point() == v2.point() {
                    Some(Primitive::Point(v1))
                }
                else {
                    Some(Primitive::Line(v1, v2))
                }
            }
            else {
                unreachable!()
            }
        }
        else {
            // 3 intersections - either 3 lines, or a line and 2 points
            if let (Primitive::Line(_, _), Primitive::Line(_, _), Primitive::Line(_, _)) = (tmp[0], tmp[1], tmp[2]) {
                Some(Primitive::Triangle(p1, p2, p3))
            }
            else {
                // a line and 2 points - find the line and return it
                if let Some(l) = tmp.iter().find(|&&x| if let Primitive::Line(_,_) = x { true } else { false }) {
                    Some(*l)
                }
                else {
                    // this case should be impossible
                    unreachable!()
                }
            }
        }
    }

    fn intersect_quad(p1: Vertex, p2: Vertex, p3: Vertex, p4: Vertex, hplane: Hyperplane) -> Option<Primitive> {
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

        if tmp.len() == 0 {
            None
        }
        else if tmp.len() == 2 {
            if let (Primitive::Point(v1), Primitive::Point(v2)) = (tmp[0], tmp[1]) {
                if v1.point() == v2.point() {
                    Some(Primitive::Point(v1))
                }
                else {
                    Some(Primitive::Line(v1, v2))
                }
            }
            else {
                unreachable!()
            }
        }
        else if tmp.len() == 3 {
            // a line and 2 points - find the line and return it
            if let Some(l) = tmp.iter().find(|&&x| if let Primitive::Line(_,_) = x { true } else { false }) {
                Some(*l)
            }
            else {
                // this case (no line amongst the 3 intersections) should be impossible
                unreachable!()
            }
        }
        else if tmp.len() == 4 {
            Some(Primitive::Quad(p1, p2, p3, p4))
        }
        else {
            unreachable!()
        }
    }

    fn intersect_tetra(p1: Vertex, p2: Vertex, p3: Vertex, p4: Vertex, hplane: Hyperplane) -> Option<Primitive> {
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

        if tmp.len() == 0 {
            None
        }
        else if tmp.len() == 3 {
            if let (Primitive::Point(v1), Primitive::Point(v2), Primitive::Point(v3)) = (tmp[0], tmp[1], tmp[2]) {
                if v1.point() == v2.point() && v2.point() == v3.point() {
                    Some(Primitive::Point(v1))
                }
                else {
                    Some(Primitive::Triangle(v1, v2, v3))
                }
            }
            else {
                unreachable!()
            }
        }
        else if tmp.len() == 4 {
            if let (Primitive::Point(v1), Primitive::Point(v2), Primitive::Point(v3), Primitive::Point(v4)) = (tmp[0], tmp[1], tmp[2], tmp[3]) {
                Some(Primitive::Quad(v1, v2, v3, v4))
            }
            else {
                unreachable!()
            }
        }
        else if tmp.len() == 5 {
            // a line and 4 points - find the line and return it
            if let Some(l) = tmp.iter().find(|&&x| if let Primitive::Line(_,_) = x { true } else { false }) {
                Some(*l)
            }
            else {
                // this case (no line amongst the 5 intersections) should be impossible
                unreachable!()
            }
        }
        else if tmp.len() == 6 {
            let mut tmp2 = Vec::new();
            for p in tmp.iter() {
                if let Primitive::Point(v) = *p {
                    tmp2.push(v);
                }
            }
            if tmp2.len() == 0 {
                Some(Primitive::Tetra(p1, p2, p3, p4))
            }
            else if tmp2.len() == 3 {
                Some(Primitive::Triangle(tmp2[0], tmp2[1], tmp2[2]))
            }
            else {
                unreachable!()
            }
        }
        else {
            unreachable!()
        }
    }
}
