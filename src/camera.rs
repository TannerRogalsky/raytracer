extern crate cgmath;

use crate::Ray;
use cgmath::{vec3, Vector3};

pub struct Camera<T> {
    origin: Vector3<T>,
    lower_left_corner: Vector3<T>,
    horizontal: Vector3<T>,
    vertical: Vector3<T>,
}

impl Camera<f64> {
    pub fn new() -> Self {
        Self {
            origin: vec3(0.0, 0.0, 0.0),
            lower_left_corner: vec3(-2.0, -1.0, -1.0),
            horizontal: vec3(4.0, 0.0, 0.0),
            vertical: vec3(0.0, 2.0, 0.0),
        }
    }
}

impl<T: cgmath::BaseNum> Camera<T> {
    pub fn ray(&self, u: T, v: T) -> Ray<T> {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
