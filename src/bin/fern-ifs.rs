use ndarray::{Array, array, Ix2};
use rust_fractal_lab::ifs::ifs_main;

fn main() {
    let d: Array<f32, Ix2> = array![
        [0.0, 0.0, 0.0, 0.16, 0.0, 0.0, 0.01],
        [0.85, 0.04, -0.04, 0.85, 0.0, 1.6, 0.85],
        [0.2, -0.26, 0.23, 0.22, 0.0, 1.6, 0.07],
        [-0.15, 0.28, 0.26, 0.24, 0.0, 0.44, 0.07],
    ];

    ifs_main(d);
}