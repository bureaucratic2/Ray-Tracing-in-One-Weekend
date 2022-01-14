use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use crate::{Color, HitRecord, Material, Ray, LEN_OF_RNG};
use lazy_static::lazy_static;
use rand::prelude::*;

// Shard the mutex.
lazy_static! {
    static ref RAND: Arc<Vec<Mutex<StdRng>>> = {
        let mut v = vec![];
        for _ in 0..LEN_OF_RNG {
            v.push(Mutex::new(StdRng::from_rng(thread_rng()).unwrap()));
        }
        Arc::new(v)
    };
}

static ID: AtomicUsize = AtomicUsize::new(0);

pub struct Dielectritic {
    ir: f64,
    id: usize,
}

impl Dielectritic {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
            id: ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    #[inline]
    fn hash(&self) -> usize {
        self.id % LEN_OF_RNG
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

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio)
                > RAND[self.hash()].lock().unwrap().gen_range(0.0..1.0)
        {
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
