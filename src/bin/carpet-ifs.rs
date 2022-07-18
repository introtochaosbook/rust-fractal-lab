use crate::ControlFlow::Wait;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};

use glium::{implement_vertex, Display, Program, Surface, VertexBuffer};

use ndarray::{array, s, Array, Ix2};
use rand::distributions::{Distribution, WeightedIndex};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f64; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(768.0 as f32, 768.0 as f32))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let inc = 0.66;
    let d: Array<f64, Ix2> = array![
        [0.33, 0.0, 0.0, 0.33, -inc, inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, 0.0, inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, inc, inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, -inc, 0.0, 0.125],
        [0.33, 0.0, 0.0, 0.33, inc, 0.0, 0.125],
        [0.33, 0.0, 0.0, 0.33, -inc, -inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, 0.0, -inc, 0.125],
        [0.33, 0.0, 0.0, 0.33, inc, -inc, 0.125],
    ];

    let probs: Vec<f64> = d.slice(s![.., -1]).to_vec();
    let dist = WeightedIndex::new(probs).unwrap();
    let mut rng = rand::thread_rng();

    // Initial starting point
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    let mut vertices = vec![];
    for i in 0..200000 {
        let r = d.row(dist.sample(&mut rng));
        x = r[0] * x + r[1] * y + r[4];
        y = r[2] * x + r[3] * y + r[5];

        if i >= 1000 {
            // Skip first 1000 iterations
            vertices.push(Vertex { position: [x, y] })
        }
    }

    let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
    let indices = NoIndices(PrimitiveType::Points);

    let program = Program::from_source(
        &display,
        r##"#version 140
in vec2 position;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
}
"##,
        r##"#version 130

out vec4 color;
void main() {
	color = vec4(0, 0, 0, 1);
}
"##,
        None,
    )
    .unwrap();

    let uniforms = glium::uniforms::EmptyUniforms;

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = Wait;

        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color(255.0, 255.0, 255.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
