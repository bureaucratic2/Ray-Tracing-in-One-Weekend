use crate::{clamp, random_in_unit_sphere, Color, HitRecord, Ray};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(c: Color, f: f64) -> Self {
        Self {
            albedo: c,
            fuzz: clamp(f, 0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = r_in.direction().reflect(&rec.normal);
        if scatter_direction.dot(&rec.normal) > 0.0 {
            Some((
                self.albedo,
                Ray::new(
                    rec.p,
                    scatter_direction + self.fuzz * random_in_unit_sphere(),
                ),
            ))
        } else {
            None
        }
    }
}
