use super::geometry::{Hyperplane, Vector};

pub trait Camera {
    fn get_hyperplane(&self) -> Hyperplane;
    fn calculate_local(&self, vector: Vector) -> Vector;
}
