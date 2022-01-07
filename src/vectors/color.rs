use std::fmt::{self, Display};

use super::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self(Vec3::new(e0, e1, e2))
    }

    pub fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            (self.0[0] * 255.999) as u64,
            (self.0[1] * 255.999) as u64,
            (self.0[2] * 255.999) as u64
        )?;
        Ok(())
    }
}
