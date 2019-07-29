use crate::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait Length {
    fn squared_len(&self) -> Float;
    fn len(&self) -> Float;
    fn to_unit(&self) -> Self;
}

impl Length for Vector {
    fn squared_len(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn len(&self) -> Float {
        self.squared_len().sqrt()
    }

    fn to_unit(&self) -> Self {
        self.scale(1. / self.len())
    }
}

pub trait Scale {
    fn scale(&self, rhs: Float) -> Self;
    fn scale_mut(&mut self, rhs: Float);
}

impl Scale for Vector {
    fn scale(&self, rhs: Float) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }

    fn scale_mut(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

pub trait Dot {
    fn dot(&self, rhs: Self) -> Float;
}

impl Dot for Vector {
    fn dot(&self, rhs: Self) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

pub trait Cross {
    fn cross(&self, rhs: Self) -> Self;
}

impl Cross for Vector {
    fn cross(&self, rhs: Self) -> Self {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: -(self.x * rhs.z - self.z * rhs.x),
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

#[cfg(test)]
pub mod test_vectors {
    use super::*;
    use crate::float;

    pub const fn make_vector(x: Float, y: Float, z: Float) -> Vector {
        Vector { x, y, z }
    }

    pub const NULL: Vector = make_vector(0., 0., 0.);
    pub const POS_100: Vector = make_vector(1., 0., 0.);
    pub const POS_010: Vector = make_vector(0., 1., 0.);
    pub const POS_001: Vector = make_vector(0., 0., 1.);

    pub const POS_123: Vector = make_vector(1., 2., 3.);
    pub const NEG_123: Vector = make_vector(-1., -2., -3.);
    pub const NEG_111: Vector = make_vector(-1., -1., -1.);
    pub const NEG_149: Vector = make_vector(-1., -4., -9.);

    #[test]
    fn test_add() {
        assert_eq!(POS_123 + NEG_123, NULL);
    }

    #[test]
    fn test_add_assign() {
        let mut vector = POS_123;
        vector += NEG_123;
        assert_eq!(vector, NULL);
    }

    #[test]
    fn test_sub() {
        assert_eq!(POS_123 - POS_123, NULL);
    }

    #[test]
    fn test_sub_assign() {
        let mut vector = POS_123;
        vector -= POS_123;
        assert_eq!(vector, NULL);
    }

    #[test]
    fn test_neg() {
        assert_eq!(-POS_123, NEG_123);
    }

    #[test]
    fn test_mul() {
        assert_eq!(POS_123 * NEG_123, NEG_149);
    }

    #[test]
    fn test_mul_assign() {
        let mut vector = POS_123;
        vector *= NEG_123;
        assert_eq!(vector, NEG_149);
    }

    #[test]
    fn test_div() {
        assert_eq!(POS_123 / NEG_123, NEG_111);
    }

    #[test]
    fn test_div_assign() {
        let mut vector = POS_123;
        vector /= NEG_123;
        assert_eq!(vector, NEG_111);
    }

    #[test]
    fn test_div_zero() {
        assert_eq!(
            POS_123 / NULL,
            make_vector(float::INFINITY, float::INFINITY, float::INFINITY)
        );
    }

    #[test]
    fn test_len() {
        assert_eq!(NULL.len(), 0.);
        assert_eq!(POS_100.len(), 1.);
        assert_eq!(POS_123.len(), NEG_123.len());
    }

    #[test]
    fn test_scale() {
        assert_eq!(POS_100.scale(10.), make_vector(10., 0., 0.));
        assert_eq!(POS_010.scale(10.), make_vector(0., 10., 0.));
        assert_eq!(POS_001.scale(10.), make_vector(0., 0., 10.));
    }

    #[test]
    fn test_scale_mut() {
        let mut vector = POS_100;
        vector.scale_mut(10.);
        assert_eq!(vector, make_vector(10., 0., 0.));

        vector = POS_010;
        vector.scale_mut(10.);
        assert_eq!(vector, make_vector(0., 10., 0.));

        vector = POS_001;
        vector.scale_mut(10.);
        assert_eq!(vector, make_vector(0., 0., 10.));
    }

    #[test]
    fn test_to_unit() {
        assert_eq!(POS_100.scale(10.).to_unit(), POS_100);
        assert_eq!(POS_010.scale(10.).to_unit(), POS_010);
        assert_eq!(POS_001.scale(10.).to_unit(), POS_001);
    }

    #[test]
    fn test_dot() {
        assert_eq!(POS_123.dot(NEG_123), -14.);
        assert_eq!(POS_100.dot(POS_100), 1.);
        assert_eq!(POS_010.dot(POS_010), 1.);
        assert_eq!(POS_001.dot(POS_001), 1.);
    }

    #[test]
    fn test_cross() {
        assert_eq!(POS_100.cross(POS_001), make_vector(0., -1., 0.));
        assert_eq!(POS_010.cross(POS_010), NULL);
        assert_eq!(POS_123.cross(NEG_123), NULL);
    }
}
