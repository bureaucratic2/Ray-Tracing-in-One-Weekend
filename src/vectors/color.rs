use std::{
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

use super::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub fn new<T: Into<f64>, U: Into<f64>, W: Into<f64>>(e0: T, e1: U, e2: W) -> Self {
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

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
