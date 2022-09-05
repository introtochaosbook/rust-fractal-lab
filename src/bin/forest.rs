use rust_fractal_lab::ifs::IfsProgram;
use rust_fractal_lab::trees::draw_forest;

fn main() {
    let mut program = IfsProgram::default();
    let mut rng = rand::thread_rng();

    draw_forest(&mut program, &mut rng, 150);

    program.run(Some(1.5));
}
