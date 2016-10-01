use std::ops;
use std::borrow::Borrow;

pub struct Vector {
    coords: [f64; 5]
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector {
        Vector {
            coords: [x, y, z, w, 1.0]
        }
    }

    pub fn projective_normalize(&mut self) {
        self.coords[0] /= self.coords[4];
        self.coords[1] /= self.coords[4];
        self.coords[2] /= self.coords[4];
        self.coords[3] /= self.coords[4];
        self.coords[4] = 1.0;
    }

    pub fn x(&self) -> f64 {
        self.coords[0] / self.coords[4]
    }

    pub fn y(&self) -> f64 {
        self.coords[1] / self.coords[4]
    }

    pub fn z(&self) -> f64 {
        self.coords[2] / self.coords[4]
    }

    pub fn w(&self) -> f64 {
        self.coords[3] / self.coords[4]
    }
}

pub struct Matrix {
    coords: [[f64; 5]; 5]
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            coords: [
                [1.0, 0.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 1.0]
            ]
        }
    }
}

impl<T: Borrow<Vector>> ops::Add<T> for Vector {
    type Output = Vector;

    fn add(mut self, other: T) -> Vector {
        let other = other.borrow();
        self.projective_normalize();
        self.coords[0] += other.coords[0] / other.coords[4];
        self.coords[1] += other.coords[1] / other.coords[4];
        self.coords[2] += other.coords[2] / other.coords[4];
        self.coords[3] += other.coords[3] / other.coords[4];
        self
    }
}

impl<'a, T: Borrow<Vector>> ops::Add<T> for &'a Vector {
    type Output = Vector;

    fn add(self, other: T) -> Vector {
        let other = other.borrow();
        Vector {
            coords: [
                self.coords[0]/self.coords[4] + other.coords[0]/other.coords[4],
                self.coords[1]/self.coords[4] + other.coords[1]/other.coords[4],
                self.coords[2]/self.coords[4] + other.coords[2]/other.coords[4],
                self.coords[3]/self.coords[4] + other.coords[3]/other.coords[4],
                1.0
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vector;

    #[test]
    fn test_add_vectors() {
        let a = Vector::new(0.0, 1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 8.0, 7.0);
        let c = a + &b;
        assert_eq!(c.x(), 2.0);
        assert_eq!(c.y(), 4.0);
        assert_eq!(c.z(), 10.0);
        assert_eq!(c.w(), 10.0);
    }

}
