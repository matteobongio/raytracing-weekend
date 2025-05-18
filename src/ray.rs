use nalgebra::{SimdRealField, Vector3};
use std::ops::Add;
use std::ops::Mul;

pub struct Ray<T> {
    origin: Vector3<T>,
    direction: Vector3<T>,
}

impl<T> Ray<T> {
    pub fn new(origin: Vector3<T>, direction: Vector3<T>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Vector3<T> {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3<T> {
        &self.direction
    }
}

impl<T> Ray<T>
where
    T: Mul<T, Output = T> + Copy + SimdRealField,
    Vector3<T>: Mul<T, Output = Vector3<T>> + Add<Vector3<T>, Output = Vector3<T>>,
{
    pub fn at(&self, t: T) -> Vector3<T> {
        self.origin + self.direction.scale(t)
    }
}
