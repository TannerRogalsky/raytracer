extern crate cgmath;

use cgmath::{Vector3, Zero};

#[derive(Copy, Clone)]
pub struct HitRecord<T> {
    t: T,
    p: Vector3<T>,
    normal: Vector3<T>,
}

impl<T: std::marker::Copy> HitRecord<T> {
    pub fn new(t: T, p: Vector3<T>, normal: Vector3<T>) -> Self {
        Self { t, p, normal }
    }

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
    fn hit(
        &self,
        r: &super::ray::Ray<T>,
        t: std::ops::Range<T>,
        rec: &HitRecord<T>,
    ) -> Option<HitRecord<T>>;
}

pub struct HitTableList<T> {
    list: std::vec::Vec<Box<HitTable<T>>>,
}

impl<T> HitTableList<T> {
    pub fn new() -> Self {
        HitTableList { list: vec![] }
    }

    pub fn add(&mut self, ht: Box<HitTable<T>>) {
        self.list.push(ht)
    }
}

impl<T> HitTable<T> for HitTableList<T>
where
    T: cgmath::BaseNum + cgmath::Bounded,
{
    fn hit(
        &self,
        r: &super::ray::Ray<T>,
        t: std::ops::Range<T>,
        rec: &HitRecord<T>,
    ) -> Option<HitRecord<T>> {
        self.list
            .iter()
            .fold(
                (None, T::max_value(), *rec),
                |(hc, closest_so_far, rec), ht| match ht.hit(r, t.start..closest_so_far, &rec) {
                    None => (hc, closest_so_far, rec),
                    Some(rec) => (Some(rec), rec.get_t(), rec),
                },
            )
            .0
    }
}
