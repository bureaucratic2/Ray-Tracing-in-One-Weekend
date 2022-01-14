use crate::vectors::vec3::Vec3;
use rand::{prelude::StdRng, Rng};

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

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: Vec3) -> Self::Output {
        for (i, e) in self.iter_mut().enumerate() {
            *e += rhs[i];
        }
        self
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for (i, e) in self.iter_mut().enumerate() {
            *e -= rhs[i];
        }
        self
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: Vec3) -> Self::Output {
        for (i, e) in self.iter_mut().enumerate() {
            *e *= rhs[i];
        }
        self
    }
}

impl<T: Into<f64>> ops::Mul<T> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        for (_, e) in self.iter_mut().enumerate() {
            *e *= rhs;
        }
        self
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
        let mut res = 0.0;
        for (i, e) in self.iter().enumerate() {
            res += *e * rhs[i];
        }
        res
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

    #[inline]
    pub fn random<T: Into<f64>, U: Into<f64>>(rng: &mut StdRng, min: T, max: U) -> Self {
        let min: f64 = min.into();
        let max: f64 = max.into();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
}
