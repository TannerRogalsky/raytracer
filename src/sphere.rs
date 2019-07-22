extern crate cgmath;

use super::{HitRecord, HitTable, Ray};
use cgmath::InnerSpace;
use std::ops::Range;

struct Sphere<T> {
    center: cgmath::Vector3<T>,
    radius: T,
}

impl<T: cgmath::BaseFloat> HitTable<T> for Sphere<T> {
    fn hit(&self, r: &Ray<T>, t: Range<f64>, rec: &mut HitRecord<T>) -> bool where T: cgmath::BaseFloat, cgmath::Vector3<T>: cgmath::InnerSpace {
//        let oc = r.origin() - self.center;
//        let a = r.direction().dot(r.direction().to_owned());
        let a = r.direction().magnitude2();
        true
    }
}

//impl HitTable<f64> for Sphere<f64> {
//    fn hit(&self, r: &Ray<f64>, t: Range<f64>, rec: &mut HitRecord<f64>) -> bool {
//        let oc = r.origin() - self.center;
//        let a = r.direction().dot(r.direction().to_owned());
//        let b = 2.0 * oc.dot(r.direction().to_owned());
//        let c = oc.dot(oc) - self.radius * self.radius;
//        let discriminant = b * b - 4.0 * a * c;
//        if discriminant > 0.0 {
//            let temp = (-b - (b * b - a * c).sqrt()) / a;
//            if t.contains(&temp) {
//                rec.set_t(temp);
//                rec.set_p(r.point_at_parameter(rec.get_t()));
//                rec.set_normal((rec.get_p() - self.center) / self.radius);
//                return true;
//            }
//            let temp = (-b + (b * b - a * c).sqrt()) / a;
//            if t.contains(&temp) {
//                rec.set_t(temp);
//                rec.set_p(r.point_at_parameter(rec.get_t()));
//                rec.set_normal((rec.get_p() - self.center) / self.radius);
//                return true;
//            }
//        }
//        false
//    }
//}
