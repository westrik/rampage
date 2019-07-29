use crate::geometry::vector::*;
use crate::Float;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

trait Point {
    fn point(&self, distance: Float) -> Vector;
}

impl Point for Ray {
    fn point(&self, distance: Float) -> Vector {
        self.origin + self.direction.scale(distance)
    }
}

#[cfg(test)]
mod test_rays {
    use super::*;
    use crate::geometry::vector::test_vectors::*;

    #[test]
    fn test_point() {
        let ray = Ray {
            origin: POS_100,
            direction: POS_123,
        };
        assert_eq!(ray.point(10.), make_vector(11., 20., 30.));
    }
}
