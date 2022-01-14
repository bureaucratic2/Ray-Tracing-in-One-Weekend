use std::{mem, sync::Arc};

use crate::{HitRecord, Hittable, Ray};

#[derive(Default)]
pub struct HittableList(Vec<Arc<dyn Hittable>>);

impl HittableList {
    pub fn new(hittable: Arc<dyn Hittable>) -> Self {
        let mut list = HittableList::default();
        list.add(hittable);
        list
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.0.push(hittable)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut closet_so_far = t_max;
        let mut hit_anything = false;

        for object in self.0.iter() {
            if object.hit(r, t_min, closet_so_far, &mut temp_rec) {
                closet_so_far = temp_rec.t;
                mem::swap(rec, &mut temp_rec);
                hit_anything = true;
            }
        }

        hit_anything
    }
}
