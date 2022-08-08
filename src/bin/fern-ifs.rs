use ndarray::{array, Array, Ix2};
use rust_fractal_lab::ifs::IfsProgram;

fn main() {
    let d: Array<f32, Ix2> = array![
        [0.0, 0.0, 0.0, 0.16, 0.0, 0.0, 0.01],
        [0.85, 0.04, -0.04, 0.85, 0.0, 1.6, 0.85],
        [0.2, -0.26, 0.23, 0.22, 0.0, 1.6, 0.07],
        [-0.15, 0.28, 0.26, 0.24, 0.0, 0.44, 0.07],
    ];

    let mut program = IfsProgram::default();
    program.sample(d, [0.0, 100.0/255.0, 0.0, 1.0], 30000);
    program.run();
}
