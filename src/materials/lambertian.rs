use lazy_static::lazy_static;
use rand::prelude::*;
use std::ops::DerefMut;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use crate::{random_unit_vector, Color, HitRecord, Material, Ray, LEN_OF_RNG};

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

pub struct Lambertian {
    albedo: Color,
    id: usize,
}

impl Lambertian {
    pub fn new(c: Color) -> Self {
        Self {
            albedo: c,
            id: ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    #[inline]
    fn hash(&self) -> usize {
        self.id % LEN_OF_RNG
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = RAND[self.hash()].lock().unwrap();
        let mut scattered_direction = rec.normal + random_unit_vector(rng.deref_mut());

        // Catch degenerate scatter direction
        if scattered_direction.near_zero() {
            scattered_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scattered_direction)))
    }
}
