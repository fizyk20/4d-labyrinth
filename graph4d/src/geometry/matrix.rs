use std::ops;
use std::borrow::Borrow;
use super::vector::Vector;

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

    pub fn rotation<T, U>(n1: T, n2: U, phi: f64) -> Matrix
        where T: Borrow<Vector>, U: Borrow<Vector>
    {
        let n1 = n1.borrow().normalized();
        let n2 = n2.borrow().normalized();
        let dot = n1.dot(&n2);
        let n2 = (n2 - (&n1 * dot)).normalized();
        let sinf = phi.sin();
        let cosf = phi.cos();
        let cosf1 = 1.0 - cosf;
        Matrix {
            coords: [
                [(n1.x()*n1.x() + n2.x()*n2.x())*cosf1 + cosf,
                 (n1.y()*n1.x() + n2.y()*n2.x())*cosf1 - (n1.z()*n2.w() - n1.w()*n2.z())*sinf,
                 (n1.z()*n1.x() + n2.z()*n2.x())*cosf1 + (n1.y()*n2.w() - n1.w()*n2.y())*sinf,
                 (n1.w()*n1.x() + n2.w()*n2.x())*cosf1 - (n1.y()*n2.z() - n1.z()*n2.y())*sinf,
                 0.0],
                [(n1.x()*n1.y() + n2.x()*n2.y())*cosf1 + (n1.z()*n2.w() - n1.w()*n2.z())*sinf,
                 (n1.y()*n1.y() + n2.y()*n2.y())*cosf1 + cosf,
                 (n1.z()*n1.y() + n2.z()*n2.y())*cosf1 - (n1.x()*n2.w() - n1.w()*n2.x())*sinf,
                 (n1.w()*n1.y() + n2.w()*n2.y())*cosf1 + (n1.x()*n2.z() - n1.z()*n2.x())*sinf,
                 0.0],
                [(n1.x()*n1.z() + n2.x()*n2.z())*cosf1 - (n1.y()*n2.w() - n1.w()*n2.y())*sinf,
                 (n1.y()*n1.z() + n2.y()*n2.z())*cosf1 + (n1.x()*n2.w() - n1.w()*n2.x())*sinf,
                 (n1.z()*n1.z() + n2.z()*n2.z())*cosf1 + cosf,
                 (n1.w()*n1.z() + n2.w()*n2.z())*cosf1 - (n1.x()*n2.y() - n1.y()*n2.x())*sinf,
                 0.0],
                [(n1.x()*n1.w() + n2.x()*n2.w())*cosf1 + (n1.y()*n2.z() - n1.z()*n2.y())*sinf,
                 (n1.y()*n1.w() + n2.y()*n2.w())*cosf1 - (n1.x()*n2.z() - n1.z()*n2.x())*sinf,
                 (n1.z()*n1.w() + n2.z()*n2.w())*cosf1 + (n1.x()*n2.y() - n1.y()*n2.x())*sinf,
                 (n1.w()*n1.w() + n2.w()*n2.w())*cosf1 + cosf,
                 0.0],
                [0.0, 0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn translation<T: Borrow<Vector>>(v: T) -> Matrix {
        let mut result = Matrix::identity();
        let v = v.borrow();
        result.coords[0][4] = v.x();
        result.coords[1][4] = v.y();
        result.coords[2][4] = v.z();
        result.coords[3][4] = v.w();
        result
    }

    pub fn scale(x: f64, y: f64, z: f64, w: f64) -> Matrix {
        Matrix {
            coords: [
                [x, 0.0, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0, 0.0],
                [0.0, 0.0, z, 0.0, 0.0],
                [0.0, 0.0, 0.0, w, 0.0],
                [0.0, 0.0, 0.0, 0.0, 1.0]
            ]
        }
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
                new_coords[i] += self.coords[i][j] * other.coord(j);
            }
        }
        Vector::from_array(new_coords)
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
    use super::Matrix;
    use super::super::Vector;
    
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
        assert_eq!(c.coord(0), 13.0);
        assert_eq!(c.coord(1), 0.0);
        assert_eq!(c.coord(2), -17.0);
        assert_eq!(c.coord(3), 1.0);
        assert_eq!(c.coord(4), 4.0);
    }
}
