use ndarray::{array, Array, Ix2};
use rust_fractal_lab::ifs::IfsProgram;

fn main() {
    let inc = 0.66;
    let d: Array<f32, Ix2> = array![
        [0.33, 0.0, 0.0, 0.33, -inc, inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, 0.0, inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, inc, inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, -inc, 0.0, 0.125],
        [0.33, 0.0, 0.0, 0.33, inc, 0.0, 0.125],
        [0.33, 0.0, 0.0, 0.33, -inc, -inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, 0.0, -inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, inc, -inc, 0.125],
    ];

    let mut program = IfsProgram::default();
    program.sample(&d, [0.0, 0.0, 0.0, 1.0], 200000);
    program.run();
}
