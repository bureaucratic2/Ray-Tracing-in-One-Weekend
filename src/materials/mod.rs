use std::rc::Rc;

pub use lambertian::Lambertian;
pub use list::HittableList;
pub use metal::Metal;
pub use sphere::Sphere;

mod lambertian;
mod list;
mod metal;
mod sphere;

use crate::{Color, Point3, Ray, Vec3};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Rc<Box<dyn Material>>>,
}

impl HitRecord {
    fn set_front_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
