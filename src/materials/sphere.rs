use std::rc::Rc;

use super::{HitRecord, Hittable, Material};
use crate::{Point3, Ray};

#[derive(Default, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Rc<Box<dyn Material>>>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Rc<Box<dyn Material>>) -> Self {
        Self {
            center: cen,
            radius: r,
            material: Some(m),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_front_normal(r, &outward_normal);
        rec.material = self.material.clone();

        true
    }
}
