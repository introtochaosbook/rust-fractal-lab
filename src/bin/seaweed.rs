use ndarray::{array, Array, Ix2};
use rust_fractal_lab::ifs::IfsProgram;

fn main() {
    let d: Array<f32, Ix2> = array![
        [0.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.25],
        [0.5, 0.0, 0.0, 0.5, 2.0, 0.0, 0.25],
        [0.4, 0.0, 1.0, 0.4, 0.0, 1.0, 0.25],
        [0.5, 0.0, 0.0, 0.5, 2.0, 1.0, 0.25],
    ];

    let mut program = IfsProgram::default();
    program.sample(&d, [0.0, 112.0 / 255.0, 0.0, 1.0], 30000);
    program.run(Some(2.0));
}
