use ndarray::{array, Array, Ix2};
use rust_fractal_lab::ifs::IfsProgram;

fn main() {
    let x = 200.0;
    let d: Array<f32, Ix2> = array![
        [0.333, 0.0, 0.0, 0.333, 1.0, 1.0, 0.166],
        [0.333, 0.0, 0.0, 0.333, x / 2.0, 1.0, 0.166],
        [0.333, 0.0, 0.0, 0.333, 1.0, x / 2.0, 0.166],
        [0.333, 0.0, 0.0, 0.333, x / 2.0, x, 0.166],
        [0.333, 0.0, 0.0, 0.333, x, x, 0.166],
        [0.333, 0.0, 0.0, 0.333, 1.0, x, 0.166],
    ];

    let mut program = IfsProgram::default();
    program.sample(&d, [72.0 / 255.0, 72.0 / 255.0, 72.0 / 255.0, 1.0], 50000);
    program.run(Some(1.5));
}
