pub use camera::Camera;
pub use materials::{Dielectritic, HittableList, Lambertian, Material, Metal};
pub use objects::{HitRecord, Hittable, Sphere};
pub use vectors::{Color, Point3, Ray, Vec3};

mod camera;
mod materials;
mod objects;
mod vectors;

use rand::prelude::*;
use std::f64::consts::PI;

static mut RAND: Option<StdRng> = None;

/// # Safety
///
/// Initialize static thread rng
pub unsafe fn initialize_rng() {
    if RAND.is_none() {
        RAND = Some(StdRng::from_rng(thread_rng()).unwrap());
    }
}

/// # Safety
///
/// Return mutable thread-local rng reference to caculate random points
#[inline]
pub unsafe fn rng() -> &'static mut StdRng {
    RAND.as_mut().unwrap()
}

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.max(min).min(max)
}

#[inline]
pub fn random_double() -> f64 {
    let rng = unsafe { rng() };
    rng.gen_range(0.0..1.0)
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

pub fn random_unit_in_disk() -> Point3 {
    let rng = unsafe { rng() };
    loop {
        let p = Point3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0);
        if p.length() > 1.0 {
            continue;
        }
        return p;
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
