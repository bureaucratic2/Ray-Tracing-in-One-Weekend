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

static LEN_OF_RNG: usize = 5;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.max(min).min(max)
}

pub fn random_in_unit_sphere(rng: &mut StdRng) -> Point3 {
    loop {
        let tmp = Point3::random(rng, -1, 1);
        if tmp.length_squared() < 1.0 {
            return tmp;
        }
    }
}

pub fn random_unit_vector(rng: &mut StdRng) -> Vec3 {
    random_in_unit_sphere(rng).unit_vector()
}

pub fn random_in_hemisphere(rng: &mut StdRng, normal: &Vec3) -> Point3 {
    let in_unit_sphere = random_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_unit_in_disk(rng: &mut StdRng) -> Point3 {
    loop {
        let p = Point3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0);
        if p.length() > 1.0 {
            continue;
        }
        return p;
    }
}
