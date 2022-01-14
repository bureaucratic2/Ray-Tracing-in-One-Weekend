pub use dielectritic::Dielectritic;
pub use lambertian::Lambertian;
pub use list::HittableList;
pub use metal::Metal;

mod dielectritic;
mod lambertian;
mod list;
mod metal;

use crate::{Color, HitRecord, Ray};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
