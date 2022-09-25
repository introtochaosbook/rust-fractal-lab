use ndarray::{array, Array, Ix2};
use rust_fractal_lab::ifs::IfsProgram;
use rand::Rng;

fn main() {
    let d: Array<f32, Ix2> = array![
        [0.33, 0.0, 0.0, 0.33, 1.0, 1.0, 0.2],
        [0.33, 0.0, 0.0, 0.33, 10.0, 1.0, 0.2],
        [0.33, 0.0, 0.0, 0.33, 1.0, 10.0, 0.2],
        [0.33, 0.0, 0.0, 0.33, 10.0, 10.0, 0.2],
        [0.33, 0.0, 0.0, 0.33, 5.0, 5.0, 0.2],
    ];

    let mut program = IfsProgram::default();
    let mut rng = rand::thread_rng();

    for _ in 0..150 {
        let shift_x = rng.gen_range(-200.0..200.0);
        let shift_y = rng.gen_range(-200.0..200.0);
        let scale = rng.gen_range(0.1..1.0);

        program.sample_affine(&d, [0.5, 0.5, 0.5, 1.0], 2000, scale, shift_x, shift_y);
    }

    program.run(Some(1.3));
}
