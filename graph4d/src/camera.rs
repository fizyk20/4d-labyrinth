use super::geometry::{Vector, Hyperplane};

pub trait Camera {
    fn get_hyperplane(&self) -> Hyperplane;
    fn calculate_local(&self, vector: Vector) -> Vector;
}
