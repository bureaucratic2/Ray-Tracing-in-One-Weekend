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

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<T: Into<f64>> ops::Mul<T> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl<T: Into<f64>> ops::Div<T> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
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
        *self / self.length()
    }
}
