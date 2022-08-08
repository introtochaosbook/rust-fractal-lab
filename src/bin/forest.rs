use ndarray::{array, Array, Ix2};
use rand::Rng;
use rust_fractal_lab::ifs::IfsProgram;

fn main() {
    let mut program = IfsProgram::default();
    let mut rng = rand::thread_rng();

    let d: Array<f32, Ix2> = array![
        [0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.05],
        [0.42, -0.42, 0.42, 0.42, 0.0, 0.2, 0.40],
        [0.42, 0.42, -0.42, 0.42, 0.0, 0.2, 0.40],
        [0.1, 0.0, 0.0, 0.1, 0.0, 0.2, 0.15],
    ];

    for _ in 0..150 {
        let shift_x = rng.gen_range(-0.5..0.5);
        let shift_y = rng.gen_range(-0.5..0.5);
        let scale = rng.gen_range(1.0..10.0);

        let color = {
            match rng.gen_range(0..=9) {
                0..=7 => [0.0, 100.0 / 255.0, 0.0, 1.0],
                8 => [204.0 / 255.0, 244.0 / 255.0, 0.0, 1.0],
                9 => [165.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0, 1.0],
                _ => unreachable!(),
            }
        };

        program.sample_affine(&d, color, 1000, scale, shift_x, shift_y);
    }

    program.run();
}
