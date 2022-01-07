use std::ops;

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
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for (i, e) in self.e.iter_mut().enumerate() {
            *e -= rhs[i];
        }
        self
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
