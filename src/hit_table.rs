extern crate cgmath;

use cgmath::{Vector3, Zero};

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

impl<T: cgmath::BaseNum> Default for HitRecord<T> {
    fn default() -> Self {
        Self {
            t: 0.0,
            p: Vector3::zero(),
            normal: Vector3::zero(),
        }
    }
}

pub trait HitTable<T> {
    type Vector3 = cgmath::Vector3<T>;

    fn hit(&self, r: &super::ray::Ray<T>, t: std::ops::Range<f64>, rec: &mut HitRecord<T>) -> bool;
}

struct HitTableList<T> {
    list: std::vec::Vec<Box<HitTable<T>>>,
}

//impl<T: cgmath::BaseNum> HitTableList<T> {
//    pub fn new() -> Self {
//        HitTableList { list: vec![] }
//    }
//
//    pub fn hit(
//        &self,
//        r: &super::ray::Ray<T>,
//        t: std::ops::Range<f64>,
//        rec: &mut HitRecord<T>,
//    ) -> bool {
//        let mut temp_rec = HitRecord::default();
//        let mut hit_anything = false;
//        let mut closest_so_far = t.end;
//
//        for hit_table in self.list {
//            if hit_table.hit(r, t.start..closest_so_far, &mut temp_rec) {
//                hit_anything = true;
//                closest_so_far = temp_rec.get_t();
//                *rec = temp_rec;
//            }
//        }
//        hit_anything
//    }
//}
