use ndarray::{array, Array, Ix2};
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::ifs::IfsProgram;

pub fn draw_forest(program: &mut IfsProgram, rng: &mut ThreadRng, count: u32) {
    let d: Array<f32, Ix2> = array![
        [0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.05],
        [0.42, -0.42, 0.42, 0.42, 0.0, 0.2, 0.40],
        [0.42, 0.42, -0.42, 0.42, 0.0, 0.2, 0.40],
        [0.1, 0.0, 0.0, 0.1, 0.0, 0.2, 0.15],
    ];

    for _ in 0..count {
        let shift_x = rng.gen_range(-0.5..0.5);
        let shift_y = rng.gen_range(-0.5..0.5);
        let scale = rng.gen_range(1.0..10.0);

        let color = {
            match rng.gen_range(0..=9) {
                // Most trees are green
                0..=7 => [0.0, 0.39, 0.0, 1.0],
                // Some trees are yellow
                8 => [0.8, 0.95, 0.0, 1.0],
                // Some trees are dead (brown)
                9 => [0.64, 0.16, 0.16, 1.0],
                _ => unreachable!(),
            }
        };

        program.sample_affine(&d, color, 2000, scale, shift_x, shift_y);
    }
}
