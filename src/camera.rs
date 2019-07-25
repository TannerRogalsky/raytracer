extern crate cgmath;

use crate::Ray;
use cgmath::{InnerSpace, Vector3};

pub struct Camera<T> {
    origin: Vector3<T>,
    lower_left_corner: Vector3<T>,
    horizontal: Vector3<T>,
    vertical: Vector3<T>,
}

impl Camera<f64> {
    pub fn new(
        origin: Vector3<f64>,
        look_at: Vector3<f64>,
        up: Vector3<f64>,
        v_fov: f64,
        aspect: f64,
    ) -> Self {
        let theta = v_fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (origin - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        Self {
            origin,
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
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
