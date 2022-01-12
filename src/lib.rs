pub use camera::Camera;
pub use materials::{
    Dielectritic, HitRecord, Hittable, HittableList, Lambertian, Material, Metal, Sphere,
};
pub use vectors::{Color, Point3, Ray, Vec3};

mod camera;
mod materials;
mod vectors;

use lazy_static::lazy_static;
use rand::prelude::StdRng;
use rand::prelude::*;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref RAND: Arc<Mutex<StdRng>> = {
        let rng = StdRng::from_rng(thread_rng()).unwrap();
        Arc::new(Mutex::new(rng))
    };
}

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.max(min).min(max)
}

pub fn random_double() -> f64 {
    RAND.lock().unwrap().gen_range(0.0..1.0)
}

pub fn random_in_unit_sphere() -> Point3 {
    loop {
        let tmp = Point3::random(-1, 1);
        if tmp.length_squared() < 1.0 {
            return tmp;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Point3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

#[test]
fn random_in_unit_sphere_test() {
    let p = random_in_unit_sphere();
    let mut flag = false;
    if p.length_squared() < 1.0 {
        flag = true;
    }
    assert_eq!(flag, true);
}
