use std::ops::DerefMut;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use crate::{clamp, random_in_unit_sphere, Color, HitRecord, Ray, LEN_OF_RNG};
use lazy_static::lazy_static;
use rand::prelude::*;

use super::Material;

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

pub struct Metal {
    albedo: Color,
    fuzz: f64,
    id: usize,
}

impl Metal {
    pub fn new(c: Color, f: f64) -> Self {
        Self {
            albedo: c,
            fuzz: clamp(f, 0.0, 1.0),
            id: ID.fetch_add(0, Ordering::SeqCst),
        }
    }

    #[inline]
    fn hash(&self) -> usize {
        self.id % LEN_OF_RNG
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = r_in.direction().reflect(&rec.normal);
        if scatter_direction.dot(&rec.normal) > 0.0 {
            let mut rng = RAND[self.hash()].lock().unwrap();
            Some((
                self.albedo,
                Ray::new(
                    rec.p,
                    scatter_direction + self.fuzz * random_in_unit_sphere(rng.deref_mut()),
                ),
            ))
        } else {
            None
        }
    }
}
