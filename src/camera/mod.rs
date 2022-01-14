use rand::prelude::*;

use crate::{degrees_to_radians, random_unit_in_disk, Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    defocus: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            Point3::new(0, 0, 0),
            Point3::new(0, 0, -1),
            Vec3::new(0, 1, 0),
            90.0,
            16.0 / 9.0,
            0.0,
            1.0,
        )
    }
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64, // vertical fov in degree
        aspect_ratio: f64,
        aperture: f64,
        focus_disk: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_disk * viewport_width * u;
        let vertical = focus_disk * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - focus_disk * w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius: aperture / 2.0,
            defocus: aperture != 0.0,
        }
    }

    #[inline]
    pub fn get_ray(&self, rng: &mut StdRng, u: f64, v: f64) -> Ray {
        if self.defocus {
            let rd = self.lens_radius * random_unit_in_disk(rng);
            let offset = self.u * rd.x() + self.v * rd.y();
            Ray::new(
                self.origin + offset,
                self.lower_left_corner + u * self.horizontal + v * self.vertical
                    - self.origin
                    - offset,
            )
        } else {
            Ray::new(
                self.origin,
                self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
            )
        }
    }
}
