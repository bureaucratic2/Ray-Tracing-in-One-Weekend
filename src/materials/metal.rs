use crate::{Color, HitRecord, Ray};

use super::Material;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(c: Color) -> Self {
        Self { albedo: c }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = r_in.direction().reflect(&rec.normal);
        if scatter_direction.dot(&rec.normal) > 0.0 {
            Some((self.albedo, Ray::new(rec.p, scatter_direction)))
        } else {
            None
        }
    }
}
