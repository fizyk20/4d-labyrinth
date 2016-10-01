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

    pub fn from_array(arr: [[f64; 5]; 5]) -> Matrix {
        Matrix { coords: arr }
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

impl<T: Borrow<Matrix>> ops::Add<T> for Matrix {
    type Output = Matrix;

    fn add(mut self, other: T) -> Matrix {
        let other = other.borrow();
        for i in 0..5 {
            for j in 0..5 {
                self.coords[i][j] += other.coords[i][j];
            }
        }
        self
    }
}

impl<'a, T: Borrow<Matrix>> ops::Add<T> for &'a Matrix {
    type Output = Matrix;

    fn add(self, other: T) -> Matrix {
        let other = other.borrow();
        let mut new_coords = [[0.0; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                new_coords[i][j] = self.coords[i][j] + other.coords[i][j];
            }
        }
        Matrix { coords: new_coords }
    }
}

impl<T: Borrow<Matrix>> ops::Sub<T> for Matrix {
    type Output = Matrix;

    fn sub(mut self, other: T) -> Matrix {
        let other = other.borrow();
        for i in 0..5 {
            for j in 0..5 {
                self.coords[i][j] -= other.coords[i][j];
            }
        }
        self
    }
}

impl<'a, T: Borrow<Matrix>> ops::Sub<T> for &'a Matrix {
    type Output = Matrix;

    fn sub(self, other: T) -> Matrix {
        let other = other.borrow();
        let mut new_coords = [[0.0; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                new_coords[i][j] = self.coords[i][j] - other.coords[i][j];
            }
        }
        Matrix { coords: new_coords }
    }
}

impl<'a, T: Borrow<Matrix>> ops::Mul<T> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, other: T) -> Matrix {
        let other = other.borrow();
        let mut new_coords = [[0.0; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..5 {
                    new_coords[i][j] += self.coords[i][k] * other.coords[k][j];
                }
            }
        }
        Matrix { coords: new_coords }
    }
}

impl<T: Borrow<Matrix>> ops::Mul<T> for Matrix {
    type Output = Matrix;

    fn mul(self, other: T) -> Matrix {
        &self * other
    }
}

// Borrow<Vector> is impossible due to conflicting traits - so just &Vector is used
impl<'a, 'b> ops::Mul<&'b Vector> for &'a Matrix {
    type Output = Vector;

    fn mul(self, other: &Vector) -> Vector {
        let mut new_coords = [0.0; 5];
        for i in 0..5 {
            for j in 0..5 {
                new_coords[i] += self.coords[i][j] * other.coords[j];
            }
        }
        Vector { coords: new_coords }
    }
}

impl<'b> ops::Mul<&'b Vector> for Matrix {
    type Output = Vector;

    fn mul(self, other: &Vector) -> Vector {
        &self * other
    }
}

#[cfg(test)]
mod test {
    use super::{Vector, Matrix};

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

    #[test]
    fn test_sub_vectors() {
        let a = Vector::new(0.0, 1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 8.0, 7.0);
        let c = a - &b;
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

    #[test]
    fn test_add_matrices() {
        let a = Matrix::from_array([[0.0, 1.0, 2.0, 3.0, 4.0],
                                    [4.0, 3.0, 2.0, -3.0, 5.0],
                                    [-2.0, 8.0, -1.0, 0.0, 4.0],
                                    [1.0, 6.0, 3.0, 2.0, 2.0],
                                    [2.0, 3.0, 3.0, 1.0, 0.0]]);
        let b = Matrix::identity();
        let c = a + b;
        assert_eq!(c.coords[0][0], 1.0);
        assert_eq!(c.coords[0][1], 1.0);
        assert_eq!(c.coords[0][2], 2.0);
        assert_eq!(c.coords[0][3], 3.0);
        assert_eq!(c.coords[0][4], 4.0);
        assert_eq!(c.coords[1][0], 4.0);
        assert_eq!(c.coords[1][1], 4.0);
        assert_eq!(c.coords[1][2], 2.0);
        assert_eq!(c.coords[1][3], -3.0);
        assert_eq!(c.coords[1][4], 5.0);
        assert_eq!(c.coords[2][0], -2.0);
        assert_eq!(c.coords[2][1], 8.0);
        assert_eq!(c.coords[2][2], 0.0);
        assert_eq!(c.coords[2][3], 0.0);
        assert_eq!(c.coords[2][4], 4.0);
        assert_eq!(c.coords[3][0], 1.0);
        assert_eq!(c.coords[3][1], 6.0);
        assert_eq!(c.coords[3][2], 3.0);
        assert_eq!(c.coords[3][3], 3.0);
        assert_eq!(c.coords[3][4], 2.0);
        assert_eq!(c.coords[4][0], 2.0);
        assert_eq!(c.coords[4][1], 3.0);
        assert_eq!(c.coords[4][2], 3.0);
        assert_eq!(c.coords[4][3], 1.0);
        assert_eq!(c.coords[4][4], 1.0);
    }

    #[test]
    fn test_sub_matrices() {
        let a = Matrix::from_array([[0.0, 1.0, 2.0, 3.0, 4.0],
                                    [4.0, 3.0, 2.0, -3.0, 5.0],
                                    [-2.0, 8.0, -1.0, 0.0, 4.0],
                                    [1.0, 6.0, 3.0, 2.0, 2.0],
                                    [2.0, 3.0, 3.0, 1.0, 0.0]]);
        let b = Matrix::identity();
        let c = a - b;
        assert_eq!(c.coords[0][0], -1.0);
        assert_eq!(c.coords[0][1], 1.0);
        assert_eq!(c.coords[0][2], 2.0);
        assert_eq!(c.coords[0][3], 3.0);
        assert_eq!(c.coords[0][4], 4.0);
        assert_eq!(c.coords[1][0], 4.0);
        assert_eq!(c.coords[1][1], 2.0);
        assert_eq!(c.coords[1][2], 2.0);
        assert_eq!(c.coords[1][3], -3.0);
        assert_eq!(c.coords[1][4], 5.0);
        assert_eq!(c.coords[2][0], -2.0);
        assert_eq!(c.coords[2][1], 8.0);
        assert_eq!(c.coords[2][2], -2.0);
        assert_eq!(c.coords[2][3], 0.0);
        assert_eq!(c.coords[2][4], 4.0);
        assert_eq!(c.coords[3][0], 1.0);
        assert_eq!(c.coords[3][1], 6.0);
        assert_eq!(c.coords[3][2], 3.0);
        assert_eq!(c.coords[3][3], 1.0);
        assert_eq!(c.coords[3][4], 2.0);
        assert_eq!(c.coords[4][0], 2.0);
        assert_eq!(c.coords[4][1], 3.0);
        assert_eq!(c.coords[4][2], 3.0);
        assert_eq!(c.coords[4][3], 1.0);
        assert_eq!(c.coords[4][4], -1.0);
    }

    #[test]
    fn test_mul_matrix_identity() {
        let a = Matrix::from_array([[0.0, 1.0, 2.0, 3.0, 4.0],
                                    [4.0, 3.0, 2.0, -3.0, 5.0],
                                    [-2.0, 8.0, -1.0, 0.0, 4.0],
                                    [1.0, 6.0, 3.0, 2.0, 2.0],
                                    [2.0, 3.0, 3.0, 1.0, 0.0]]);
        let b = Matrix::identity();
        let c = &a * &b;
        let d = &b * &a;
        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(c.coords[i][j], a.coords[i][j]);
                assert_eq!(d.coords[i][j], a.coords[i][j]);
            }
        }
    }

    #[test]
    fn test_mul_matrices() {
        let a = Matrix::from_array([[0.0, 1.0, 2.0, 3.0, 4.0],
                                    [4.0, 3.0, 2.0, -3.0, 5.0],
                                    [-2.0, 8.0, -1.0, 0.0, 4.0],
                                    [1.0, 6.0, 3.0, 2.0, 2.0],
                                    [2.0, 3.0, 3.0, 1.0, 0.0]]);
        let b = Matrix::from_array([[-7.0, 1.0, 2.0, 3.0, 2.0],
                                    [4.0, 6.0, 2.0, 3.0, 2.0],
                                    [-2.0, 8.0, 1.0, 0.0, 4.0],
                                    [1.0, 3.0, 3.0, 4.0, 2.0],
                                    [2.0, 1.0, 3.0, 1.0, 3.0]]);
        let c = a * b;
        assert_eq!(c.coords[0][0], 11.0);
        assert_eq!(c.coords[0][1], 35.0);
        assert_eq!(c.coords[0][2], 25.0);
        assert_eq!(c.coords[0][3], 19.0);
        assert_eq!(c.coords[0][4], 28.0);
        assert_eq!(c.coords[1][0], -13.0);
        assert_eq!(c.coords[1][1], 34.0);
        assert_eq!(c.coords[1][2], 22.0);
        assert_eq!(c.coords[1][3], 14.0);
        assert_eq!(c.coords[1][4], 31.0);
        assert_eq!(c.coords[2][0], 56.0);
        assert_eq!(c.coords[2][1], 42.0);
        assert_eq!(c.coords[2][2], 23.0);
        assert_eq!(c.coords[2][3], 22.0);
        assert_eq!(c.coords[2][4], 20.0);
        assert_eq!(c.coords[3][0], 17.0);
        assert_eq!(c.coords[3][1], 69.0);
        assert_eq!(c.coords[3][2], 29.0);
        assert_eq!(c.coords[3][3], 31.0);
        assert_eq!(c.coords[3][4], 36.0);
        assert_eq!(c.coords[4][0], -7.0);
        assert_eq!(c.coords[4][1], 47.0);
        assert_eq!(c.coords[4][2], 16.0);
        assert_eq!(c.coords[4][3], 19.0);
        assert_eq!(c.coords[4][4], 24.0);
    }

    #[test]
    fn test_mul_matrix_vector() {
        let a = Matrix::from_array([[0.0, 1.0, 2.0, 3.0, 4.0],
                                    [4.0, 3.0, 2.0, -3.0, 5.0],
                                    [-2.0, 8.0, -1.0, 0.0, 4.0],
                                    [1.0, 6.0, 3.0, 2.0, 2.0],
                                    [2.0, 3.0, 3.0, 1.0, 0.0]]);
        let b = Vector::new(2.0, -2.0, 1.0, 3.0);
        let c = a * &b;
        assert_eq!(c.coords[0], 13.0);
        assert_eq!(c.coords[1], 0.0);
        assert_eq!(c.coords[2], -17.0);
        assert_eq!(c.coords[3], 1.0);
        assert_eq!(c.coords[4], 4.0);
    }

}
