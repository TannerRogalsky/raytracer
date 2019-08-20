extern crate cgmath;

use super::{HitRecord, HitTable, Material, Ray};
use cgmath::InnerSpace;
use std::ops::Range;
use std::sync::Arc;

pub struct Sphere<T> {
    center: cgmath::Vector3<T>,
    radius: T,
    // this could, theoretically, be a reference but doing the lifetimes sounds unfun
    material: Arc<dyn Material<T> + Sync + Send>,
}

impl<T> Sphere<T> {
    pub fn new(
        center: cgmath::Vector3<T>,
        radius: T,
        material: Arc<dyn Material<T> + Sync + Send>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<T: cgmath::BaseNum> Sphere<T> {
    pub fn hit_record(&self, ray: &Ray<T>, t: T) -> HitRecord<T> {
        let p = ray.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        HitRecord::new(t, p, normal, Arc::clone(&self.material))
    }
}

impl<T: cgmath::BaseFloat> HitTable<T> for Sphere<T> {
    fn hit(&self, r: &Ray<T>, t: Range<T>) -> Option<HitRecord<T>> {
        let oc = r.origin() - self.center;
        let a = r.direction().magnitude2();
        let b = oc.dot(*r.direction());
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        // todo: making 0 a constant would be an improvement https://github.com/rust-num/num-traits/issues/54
        if discriminant > T::zero() {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t.end && temp > t.start {
                return Some(self.hit_record(r, temp));
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t.end && temp > t.start {
                return Some(self.hit_record(r, temp));
            }
        }
        None
    }
}
