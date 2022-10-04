use ndarray::{array, Array, Ix2};
use rand::Rng;
use rust_fractal_lab::ifs::IfsProgram;

fn main() {
    let d: Array<f32, Ix2> = array![
        [0.5, 0.0, 0.0, 0.25, 1.0, 1.0, 0.25],
        [0.25, 0.0, 0.0, 0.7, 50.0, 1.0, 0.25],
        [0.25, 0.0, 0.0, 0.7, 1.0, 50.0, 0.25],
        [0.5, 0.0, 0.0, 0.25, 50.0, 50.0, 0.25],
    ];

    let mut program = IfsProgram::default();
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let r = rng.gen_range(24.0..35.0);
        let g = rng.gen_range(63.0..128.0);

        let shift_x = rng.gen_range(-200.0..200.0);
        let shift_y = rng.gen_range(-200.0..200.0);
        let scale = rng.gen_range(0.6..1.0);

        program.sample_affine(
            &d,
            [r / 255.0, g / 255.0, 0.0, 1.0],
            2000,
            scale,
            shift_x,
            shift_y,
        );
    }

    program.run(Some(1.5));
}
