use std::ops;
use std::cmp::PartialEq;
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
        if self.coords[4] == 1.0 {
            return
        }
        self.coords[0] /= self.coords[4];
        self.coords[1] /= self.coords[4];
        self.coords[2] /= self.coords[4];
        self.coords[3] /= self.coords[4];
        self.coords[4] = 1.0;
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.coords[0] / self.coords[4]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.coords[1] / self.coords[4]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.coords[2] / self.coords[4]
    }

    #[inline]
    pub fn w(&self) -> f64 {
        self.coords[3] / self.coords[4]
    }

    pub fn dot<T: Borrow<Vector>>(&self, other: T) -> f64 {
        let other = other.borrow();
        self.x()*other.x() + self.y()*other.y() + self.z()*other.z() + self.w()*other.w()
    }

    #[inline]
    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Vector {
        self / self.len()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.coords[0] /= len;
        self.coords[1] /= len;
        self.coords[2] /= len;
        self.coords[3] /= len;
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

impl<T: Borrow<Vector>> ops::Sub<T> for Vector {
    type Output = Vector;

    fn sub(mut self, other: T) -> Vector {
        let other = other.borrow();
        self.projective_normalize();
        self.coords[0] -= other.coords[0] / other.coords[4];
        self.coords[1] -= other.coords[1] / other.coords[4];
        self.coords[2] -= other.coords[2] / other.coords[4];
        self.coords[3] -= other.coords[3] / other.coords[4];
        self
    }
}

impl<'a, T: Borrow<Vector>> ops::Sub<T> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: T) -> Vector {
        let other = other.borrow();
        Vector {
            coords: [
                self.coords[0]/self.coords[4] - other.coords[0]/other.coords[4],
                self.coords[1]/self.coords[4] - other.coords[1]/other.coords[4],
                self.coords[2]/self.coords[4] - other.coords[2]/other.coords[4],
                self.coords[3]/self.coords[4] - other.coords[3]/other.coords[4],
                1.0
            ]
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(mut self, other: f64) -> Vector {
        self.projective_normalize();
        self.coords[0] *= other;
        self.coords[1] *= other;
        self.coords[2] *= other;
        self.coords[3] *= other;
        self
    }
}

impl<'a> ops::Mul<f64> for &'a Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            coords: [
                self.coords[0] / self.coords[4] * other,
                self.coords[1] / self.coords[4] * other,
                self.coords[2] / self.coords[4] * other,
                self.coords[3] / self.coords[4] * other,
                1.0
            ]
        }
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(mut self, other: f64) -> Vector {
        self.projective_normalize();
        self.coords[0] /= other;
        self.coords[1] /= other;
        self.coords[2] /= other;
        self.coords[3] /= other;
        self
    }
}

impl<'a> ops::Div<f64> for &'a Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            coords: [
                self.coords[0] / self.coords[4] / other,
                self.coords[1] / self.coords[4] / other,
                self.coords[2] / self.coords[4] / other,
                self.coords[3] / self.coords[4] / other,
                1.0
            ]
        }
    }
}

const EPSILON: f64 = 0.0001;

impl PartialEq for Vector {
    fn eq(&self, rhs: &Vector) -> bool {
        let xeq = (self.x() - rhs.x()).abs() < EPSILON;
        let yeq = (self.y() - rhs.y()).abs() < EPSILON;
        let zeq = (self.z() - rhs.z()).abs() < EPSILON;
        let weq = (self.w() - rhs.w()).abs() < EPSILON;
        xeq && yeq && zeq && weq
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
