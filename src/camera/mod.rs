use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_center: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::default();
        let horizontal = Vec3::new(viewport_width, 0, 0);
        let vertical = Vec3::new(0, viewport_height, 0);
        let lower_left_center =
            -origin - horizontal / 2 - vertical / 2 - Vec3::new(0, 0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_center,
        }
    }
}

impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }

    #[inline]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_center + u * self.horizontal + v * self.vertical,
        )
    }
}
