use std::ops;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
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
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];

        self
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(mut self) -> Self::Output {
        self.e[0] = -self.e[0];
        self.e[1] = -self.e[1];
        self.e[2] = -self.e[2];
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
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
