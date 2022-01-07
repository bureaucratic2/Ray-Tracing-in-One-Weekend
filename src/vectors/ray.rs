use super::Vec3;
use super::Vec3 as Point3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[inline]
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Point3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(t * &self.direction)
    }
}
