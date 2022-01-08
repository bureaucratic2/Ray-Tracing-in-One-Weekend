pub use camera::Camera;
pub use objects::{HitRecord, Hittable, HittableList, Sphere};
pub use vectors::{Color, Point3, Ray, Vec3};

mod camera;
mod objects;
mod vectors;

use std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.max(min).min(max)
}
