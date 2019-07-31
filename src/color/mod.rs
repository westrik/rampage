use crate::Float;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, PartialEq)]
pub struct RgbColor {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

pub type Color = RgbColor;

pub const BLACK: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
};
pub const WHITE: Color = Color {
    r: 1.,
    g: 1.,
    b: 1.,
};

impl Add for RgbColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for RgbColor {
    fn add_assign(&mut self, rhs: Self) {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
    }
}

impl Mul for RgbColor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

#[cfg(test)]
pub mod test_colors {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(BLACK + WHITE, WHITE);
    }

    #[test]
    fn test_add_assign() {
        let mut color = BLACK;
        color += WHITE;
        assert_eq!(color, WHITE);
    }

    #[test]
    fn test_mul() {
        assert_eq!(WHITE * WHITE, WHITE);
        assert_eq!(BLACK * WHITE, BLACK);
        assert_eq!(BLACK * BLACK, BLACK);
    }
}
