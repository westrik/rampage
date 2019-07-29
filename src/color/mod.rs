use crate::Float;
use std::ops::{AddAssign, Mul};

#[derive(Debug, PartialEq)]
pub struct RgbColor {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

pub type Color = RgbColor;

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

    const WHITE: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    const BLACK: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };

    #[test]
    fn test_add_assign() {
        let mut color = WHITE;
        color += BLACK;
        assert_eq!(color, BLACK);
    }

    #[test]
    fn test_mul() {
        assert_eq!(WHITE * WHITE, WHITE);
        assert_eq!(BLACK * WHITE, WHITE);
        assert_eq!(BLACK * BLACK, BLACK);
    }
}
