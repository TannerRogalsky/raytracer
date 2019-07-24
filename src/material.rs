use super::{HitRecord, Ray};
use cgmath::{vec3, InnerSpace, Vector3};
use rand::distributions::Standard;
use rand::prelude::*;

fn rand_in_unit_sphere<T>() -> Vector3<T>
where
    T: cgmath::BaseFloat,
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen::<T>();
        let y = rng.gen::<T>();
        let z = rng.gen::<T>();
        let one = T::one();
        let p = vec3(x, y, z) * T::from(2.0).unwrap() - vec3(one, one, one);
        if p.magnitude2() < one {
            return p;
        }
    }
}

pub trait Material<T> {
    fn scatter(&self, r: &Ray<T>, rec: &HitRecord<T>) -> Option<(Vector3<T>, Ray<T>)>;
}

pub struct Lambertian<T> {
    albedo: Vector3<T>,
}

impl<T> Lambertian<T> {
    pub fn new(albedo: Vector3<T>) -> Self {
        Self { albedo }
    }
}

impl<T> Material<T> for Lambertian<T>
where
    T: cgmath::BaseFloat,
    Standard: Distribution<T>,
{
    fn scatter(&self, _r: &Ray<T>, rec: &HitRecord<T>) -> Option<(Vector3<T>, Ray<T>)> {
        let target = rec.get_p() + rec.get_normal() + rand_in_unit_sphere();
        let scattered = Ray::new(*rec.get_p(), target - rec.get_p());
        Some((self.albedo, scattered))
    }
}

pub struct Metal<T> {
    albedo: Vector3<T>,
}

impl<T> Metal<T> {
    pub fn new(albedo: Vector3<T>) -> Self {
        Self { albedo }
    }
}

fn reflect<T>(v: Vector3<T>, n: Vector3<T>) -> Vector3<T>
where
    T: cgmath::BaseFloat,
{
    v - n * v.dot(n) * T::from(2.0).unwrap()
}

impl<T> Material<T> for Metal<T>
where
    T: cgmath::BaseFloat,
{
    fn scatter(&self, r: &Ray<T>, rec: &HitRecord<T>) -> Option<(Vector3<T>, Ray<T>)> {
        let reflected = reflect(r.direction().normalize(), *rec.get_normal());
        let scattered = Ray::new(*rec.get_p(), reflected);
        if scattered.direction().dot(*rec.get_normal()) > T::zero() {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
