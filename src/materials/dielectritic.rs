use crate::{random_double, Color, HitRecord, Material, Ray};

pub struct Dielectritic {
    ir: f64,
}

impl Dielectritic {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectritic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            // inside to outside
            1.0 / self.ir
        } else {
            // outside to inside
            self.ir
        };

        let cos_theta = (-r_in.direction().unit_vector()).dot(&rec.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = sin_theta * refraction_ratio > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                // must reflect
                r_in.direction().reflect(&rec.normal)
            } else {
                r_in.direction()
                    .unit_vector()
                    .refract(&rec.normal, refraction_ratio)
            };

        Some((Color::new(1, 1, 1), Ray::new(rec.p, direction)))
    }
}

fn reflectance(consine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - consine).powi(5)
}
