use std::cmp::PartialEq;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vector {
    coords: [f64; 5],
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector {
        Vector {
            coords: [x, y, z, w, 1.0],
        }
    }

    pub fn from_array(arr: [f64; 5]) -> Vector {
        Vector { coords: arr }
    }

    pub fn projective_normalize(&mut self) {
        if self.coords[4] == 1.0 {
            return;
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

    #[inline]
    pub fn coord(&self, i: usize) -> f64 {
        self.coords[i]
    }

    pub fn dot(&self, other: Vector) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }

    #[inline]
    pub fn len(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn normalized(&self) -> Vector {
        *self / self.len()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.coords[0] /= len;
        self.coords[1] /= len;
        self.coords[2] /= len;
        self.coords[3] /= len;
    }

    pub fn cross3(arg1: Vector, arg2: Vector) -> Vector {
        Vector {
            coords: [
                arg1.y() * arg2.z() - arg1.z() * arg2.y(),
                arg1.z() * arg2.x() - arg1.x() * arg2.z(),
                arg1.x() * arg2.y() - arg1.y() * arg2.x(),
                0.0,
                1.0,
            ],
        }
    }

    pub fn cross4(arg1: Vector, arg2: Vector, arg3: Vector) -> Vector {
        Vector {
            coords: [
                arg1.y() * arg2.z() * arg3.w()
                    + arg1.z() * arg2.w() * arg3.y()
                    + arg1.w() * arg2.y() * arg3.z()
                    - arg1.y() * arg2.w() * arg3.z()
                    - arg1.z() * arg2.y() * arg3.w()
                    - arg1.w() * arg2.z() * arg3.y(),
                arg1.z() * arg2.w() * arg3.x()
                    + arg1.w() * arg2.x() * arg3.z()
                    + arg1.x() * arg2.z() * arg3.w()
                    - arg1.z() * arg2.x() * arg3.w()
                    - arg1.w() * arg2.z() * arg3.x()
                    - arg1.x() * arg2.w() * arg3.z(),
                arg1.w() * arg2.x() * arg3.y()
                    + arg1.x() * arg2.y() * arg3.w()
                    + arg1.y() * arg2.w() * arg3.x()
                    - arg1.w() * arg2.y() * arg3.x()
                    - arg1.x() * arg2.w() * arg3.y()
                    - arg1.y() * arg2.x() * arg3.w(),
                arg1.x() * arg2.y() * arg3.z()
                    + arg1.y() * arg2.z() * arg3.x()
                    + arg1.z() * arg2.x() * arg3.y()
                    - arg1.x() * arg2.z() * arg3.y()
                    - arg1.y() * arg2.x() * arg3.z()
                    - arg1.z() * arg2.y() * arg3.x(),
                1.0,
            ],
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(mut self, other: Vector) -> Vector {
        self.projective_normalize();
        self.coords[0] += other.coords[0] / other.coords[4];
        self.coords[1] += other.coords[1] / other.coords[4];
        self.coords[2] += other.coords[2] / other.coords[4];
        self.coords[3] += other.coords[3] / other.coords[4];
        self
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(mut self, other: Vector) -> Vector {
        self.projective_normalize();
        self.coords[0] -= other.coords[0] / other.coords[4];
        self.coords[1] -= other.coords[1] / other.coords[4];
        self.coords[2] -= other.coords[2] / other.coords[4];
        self.coords[3] -= other.coords[3] / other.coords[4];
        self
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
        let c = a + b;
        assert_eq!(c.x(), 2.0);
        assert_eq!(c.y(), 4.0);
        assert_eq!(c.z(), 10.0);
        assert_eq!(c.w(), 10.0);
    }

    #[test]
    fn test_sub_vectors() {
        let a = Vector::new(0.0, 1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 8.0, 7.0);
        let c = a - b;
        assert_eq!(c.x(), -2.0);
        assert_eq!(c.y(), -2.0);
        assert_eq!(c.z(), -6.0);
        assert_eq!(c.w(), -4.0);
    }

    #[test]
    fn test_mul_vec_f64() {
        let a = Vector::new(0.0, 1.0, 2.0, 3.0);
        let c = a * 3.0;
        assert_eq!(c.x(), 0.0);
        assert_eq!(c.y(), 3.0);
        assert_eq!(c.z(), 6.0);
        assert_eq!(c.w(), 9.0);
    }

    #[test]
    fn test_div_vec_f64() {
        let a = Vector::new(0.0, 1.0, 2.0, 3.0);
        let c = a / 2.0;
        assert_eq!(c.x(), 0.0);
        assert_eq!(c.y(), 0.5);
        assert_eq!(c.z(), 1.0);
        assert_eq!(c.w(), 1.5);
    }

    #[test]
    fn test_dot_product() {
        let a = Vector::new(0.0, 1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 8.0, 7.0);
        let c = a.dot(b);
        assert_eq!(c, 40.0);
    }
}
