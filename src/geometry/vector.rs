use crate::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod test_vectors {
    use super::*;

    const POS_123: Vector = Vector {
        x: 1.,
        y: 2.,
        z: 3.,
    };

    const NEG_123: Vector = Vector {
        x: -1.,
        y: -2.,
        z: -3.,
    };

    const NULL: Vector = Vector {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    const NEG_111: Vector = Vector {
        x: -1.,
        y: -1.,
        z: -1.,
    };

    const NEG_149: Vector = Vector {
        x: -1.,
        y: -4.,
        z: -9.,
    };

    #[test]
    fn test_add() {
        assert_eq!(POS_123 + NEG_123, NULL);
    }

    #[test]
    fn test_add_assign() {
        let mut vector = POS_123.clone();
        vector += NEG_123;
        assert_eq!(vector, NULL);
    }

    #[test]
    fn test_sub() {
        assert_eq!(POS_123 - POS_123, NULL);
    }

    #[test]
    fn test_sub_assign() {
        let mut vector = POS_123.clone();
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
        let mut vector = POS_123.clone();
        vector *= NEG_123;
        assert_eq!(vector, NEG_149);
    }

    #[test]
    fn test_div() {
        assert_eq!(POS_123 / NEG_123, NEG_111);
    }

    #[test]
    fn test_div_assign() {
        let mut vector = POS_123.clone();
        vector /= NEG_123;
        assert_eq!(vector, NEG_111);
    }

    #[test]
    fn test_div_zero() {
        assert_eq!(
            POS_123 / NULL,
            Vector {
                x: core::f64::INFINITY,
                y: core::f64::INFINITY,
                z: core::f64::INFINITY
            }
        );
    }
}
