use crate::{random_unit_vector, Color, Ray};

use super::{HitRecord, Material};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(c: Color) -> Self {
        Self { albedo: c }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scattered_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scattered_direction.near_zero() {
            scattered_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scattered_direction)))
    }
}
