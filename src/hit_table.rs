extern crate cgmath;

use cgmath::{Vector3, Zero};

pub struct HitRecord<T> {
    t: T,
    p: Vector3<T>,
    normal: Vector3<T>,
}

impl<T: std::marker::Copy> HitRecord<T> {
    pub fn get_t(&self) -> T {
        self.t
    }

    pub fn set_t(&mut self, t: T) {
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
            t: T::zero(),
            p: Vector3::zero(),
            normal: Vector3::zero(),
        }
    }
}

pub trait HitTable<T> {
    fn hit(&self, r: &super::ray::Ray<T>, t: std::ops::Range<T>, rec: &mut HitRecord<T>) -> bool;
}

struct HitTableList<T> {
    list: std::vec::Vec<Box<HitTable<T>>>,
}

//impl<T: cgmath::BaseFloat> HitTableList<T> {
//    pub fn new() -> Self {
//        HitTableList { list: vec![] }
//    }
//
//    pub fn hit(
//        &self,
//        r: &super::ray::Ray<T>,
//        t: std::ops::Range<T>,
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
