use ndarray::{array, Array, Ix2};
use rand::Rng;
use rust_fractal_lab::ifs::IfsProgram;
use rust_fractal_lab::trees::draw_forest;

fn main() {
    let mut program = IfsProgram::default();
    let mut rng = rand::thread_rng();

    // Draw 75 trees behind the redwoods
    draw_forest(&mut program, &mut rng, 75);
    // Keep the trees to the bottom half of the screen
    program.normalize_points_to_ranges(-1.0, 1.0, -1.0, 0.5);

    // Draw the redwoods themselves
    let inc = 6.0;
    let redwood_d: Array<f32, Ix2> = array![
        [0.33, 0.0, 0.0, 0.5, 1.0, 0.0, 0.125],
        [0.33, 0.0, 0.0, 0.5, inc, 0.0, 0.125],
        [0.33, 0.0, 0.0, 0.5, 1.0, -inc, 0.125],
        [0.33, 0.0, 0.0, 0.5, inc, -inc, 0.125],
        [0.33, 0.0, 2.0, 0.5, inc / 2.0, 1.0, 0.125],
        [0.33, 0.0, 2.0, 0.5, inc, inc / 2.0, 0.125],
        [0.33, 0.0, 3.0, 0.5, 1.0, inc / 2.0, 0.125],
        [0.33, 0.0, 2.0, 0.5, inc / 2.0, inc, 0.125],
    ];

    program.sample(&redwood_d, [101.0 / 255.0, 2.0 / 255.0, 0.0, 1.0], 300000);
    program.normalize_points();

    // Draw 75 trees in front of the redwoods
    draw_forest(&mut program, &mut rng, 75);
    // Keep the trees to the bottom half of the screen
    program.normalize_points_to_ranges(-1.0, 1.0, -1.0, 0.5);

    program.run(Some(1.5));
}
