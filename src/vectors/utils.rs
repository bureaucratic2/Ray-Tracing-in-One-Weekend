use crate::vectors::vec3::Vec3;
use std::{
    fmt::{self, Display},
    ops,
};

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])?;
        Ok(())
    }
}

impl<'a, 'b> ops::Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'b Vec3) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<'a, 'b> ops::Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &'b Vec3) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<'a, 'b> ops::Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &'b Vec3) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<'a> ops::Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl<'a> ops::Mul<&'a Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &'a Vec3) -> Self::Output {
        rhs * self
    }
}

impl<'a> ops::Div<f64> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Vec3 {
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    #[inline]
    #[must_use]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }

    #[inline]
    #[must_use]
    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }
}
