use ndarray::array;
use rust_fractal_lab::ifs::IfsProgram;

fn main() {
    let d = array![
        [0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 0.33],
        [0.5, 0.0, 0.0, 0.5, 0.0, 0.5, 0.33],
        [0.5, 0.0, 0.0, 0.5, 0.5, -0.5, 0.33]
    ];

    let mut program = IfsProgram::default();
    program.sample(&d, [0.0, 0.0, 0.0, 1.0], 200000);
    program.run();
}