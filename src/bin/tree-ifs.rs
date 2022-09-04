use ndarray::{array, Array, Ix2};
use rust_fractal_lab::ifs::IfsProgram;

fn main() {
    let d: Array<f32, Ix2> = array![
        [0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.05],
        [0.42, -0.42, 0.42, 0.42, 0.0, 0.2, 0.40],
        [0.42, 0.42, -0.42, 0.42, 0.0, 0.2, 0.40],
        [0.1, 0.0, 0.0, 0.1, 0.0, 0.2, 0.15],
    ];

    let mut program = IfsProgram::default();
    program.sample(&d, [0.0, 100.0 / 255.0, 0.0, 1.0], 30000);
    program.run(Some(2.0));
}
