use std::ops::{self, Deref, DerefMut};

const S: f64 = 1e-8;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    /// awkawrd walkthrough to allow usage like Vec3::new(1.3, 2, 5/4)
    pub fn new<T: Into<f64>, U: Into<f64>, W: Into<f64>>(e0: T, e1: U, e2: W) -> Self {
        Self {
            e: [e0.into(), e1.into(), e2.into()],
        }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        self.e[0].abs() < S && self.e[1].abs() < S && self.e[2].abs() < S
    }

    #[must_use]
    #[inline]
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * *n
    }

    #[must_use]
    #[inline]
    /// Before call fn refract(), caller vector should be a unit vector
    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -self.dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }
}

impl Deref for Vec3 {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.e
    }
}

impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.e
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(mut self) -> Self::Output {
        for e in self.e.iter_mut() {
            *e = -*e;
        }
        self
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for (i, e) in self.e.iter_mut().enumerate() {
            *e += rhs[i];
        }
    }
}

impl<T: Into<f64>> ops::MulAssign<T> for Vec3 {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for e in self.e.iter_mut() {
            *e *= rhs;
        }
    }
}

impl<T: Into<f64>> ops::DivAssign<T> for Vec3 {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        *self *= 1.0 / rhs;
    }
}
