use rand::Rng;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};

fn gen(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut x = rng.gen_range(-1.0..=1.0);
    let mut y = rng.gen_range(-1.0..=1.0);

    b.iter(|| {
        let mut vertices = vec![];

        for i in 0..2000000 {
            match rng.gen_range(0..=2) {
                0 => {
                    x /= 2.0;
                    y = (1.0 + y) / 2.0;
                }
                1 => {
                    x = (1.0 + x) / 2.0;
                    y = (-1.0 + y) / 2.0;
                }
                2 => {
                    x = (-1.0 + x) / 2.0;
                    y = (-1.0 + y) / 2.0;
                }
                _ => unreachable!(),
            }

            if i > 1000 {
                vertices.push(Vertex { position: [x, y] })
            }
        }

        vertices
    });
}

benchmark_group!(benches, gen);
benchmark_main!(benches);
