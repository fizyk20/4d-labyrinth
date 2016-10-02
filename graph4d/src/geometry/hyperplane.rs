use super::Vector;

pub struct Hyperplane {
    normal: Vector,
    param: f64
}

impl Hyperplane {
    pub fn new(v: Vector, p: f64) -> Hyperplane {
        Hyperplane {
            normal: v,
            param: p
        }
    }

    pub fn dot(&self, v: Vector) -> f64 {
        self.normal.dot(v) + self.param
    }
}
