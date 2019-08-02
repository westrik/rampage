use crate::geometry::vector::*;

pub fn random_vector_in_unit_sphere() -> Vector {
    // NO RANDOM 4 U
    let mut vector;
    loop {
        vector = Vector {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        };
        if vector.squared_len() < 1. {
            break;
        }
    }
    vector
}

pub fn random_vector_in_unit_disk() -> Vector {
    // NO RANDOM 4 U
    let mut vector;
    loop {
        vector = Vector {
            x: 0.5,
            y: 0.5,
            z: 0.,
        };
        if vector.dot(vector) < 1. {
            break;
        }
    }
    vector
}
