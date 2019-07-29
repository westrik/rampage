use crate::geometry::vector::*;
use crate::Float;
use rand::prelude::*;

pub fn get_random_vector_in_unit_sphere() -> Vector {
    let mut rng = rand::thread_rng();
    let mut vector;
    loop {
        vector = Vector {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
        .scale(2.)
            - Vector {
                x: 1.,
                y: 1.,
                z: 1.,
            };
        if vector.squared_len() < 1. {
            break;
        }
    }
    vector
}

pub fn get_random_vector_in_unit_disk() -> Vector {
    let mut rng = rand::thread_rng();
    let mut vector;
    loop {
        vector = Vector {
            x: 2. * rng.gen::<Float>() - 1.,
            y: 2. * rng.gen::<Float>() - 1.,
            z: 0.,
        };
        if vector.dot(vector) < 1. {
            break;
        }
    }
    vector
}
