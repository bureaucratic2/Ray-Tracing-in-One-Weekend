pub use color::Color;
pub use ray::Ray;
pub use vec3::Vec3;
pub use vec3::Vec3 as Point3;

mod color;
mod ray;
mod utils;
mod vec3;

#[cfg(test)]
mod test {
    use crate::vectors::Vec3;

    #[test]
    fn div() {
        let v = Vec3::new(2, 4, 6);
        let divided = &v / 2;
        assert_eq!(divided, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 2.0;
        assert_eq!(v, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let mut v2 = Vec3::new(1.0, 2.0, 3.0);
        v1 += Vec3::new(1.0, 2.0, 3.0);
        v2 *= 2.0;
        assert_eq!(v1, v2);
    }

    #[test]
    fn add_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 2.0);
        assert_eq!(v[1], 4.0);
        assert_eq!(v[2], 6.0);
    }

    #[test]
    fn index() {
        let mut vector = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vector[0], 1.0);
        assert_eq!(vector[1], 2.0);
        assert_eq!(vector[2], 3.0);

        vector[2] *= 5.0;

        assert_eq!(vector[2], 15.0);
    }

    #[test]
    fn neg() {
        assert_eq!(-Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(1.0 - 2.0, 2.0 - 4.0, 3.0 - 6.0)
        );
    }
}
