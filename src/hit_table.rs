extern crate cgmath;

use cgmath::Vector3;

pub struct HitRecord<T> {
    t: f64,
    p: Vector3<T>,
    normal: Vector3<T>,
}

impl<T> HitRecord<T> {
    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    pub fn get_p(&self) -> &Vector3<T> {
        &self.p
    }

    pub fn set_p(&mut self, p: Vector3<T>) {
        self.p = p;
    }

    pub fn get_normal(&self) -> &Vector3<T> {
        &self.normal
    }

    pub fn set_normal(&mut self, normal: Vector3<T>) {
        self.normal = normal;
    }
}

pub trait HitTable<T> {
    fn hit(&self, r: &super::ray::Ray<T>, t: std::ops::Range<T>, rec: &mut HitRecord<T>) -> bool;
}