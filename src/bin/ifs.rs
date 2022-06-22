use crate::ControlFlow::Wait;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};

use glium::{implement_vertex, Display, Program, Surface, VertexBuffer};

use ndarray::array;
use rand::distributions::{Distribution, WeightedIndex};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(768.0, 768.0))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let mut rng = rand::thread_rng();
    let mut vertices = vec![];

    // Initial starting point
    let mut x = 0.0;
    let mut y = 0.0;

    let d = array![
        [0.5, 0.0, 0.0, 0.5, -0.5, -0.5],
        [0.5, 0.0, 0.0, 0.5, 0.0, 0.5],
        [0.5, 0.0, 0.0, 0.5, 0.5, -0.5]
    ];

    let p: Vec<f64> = vec![0.33, 0.33, 0.33];
    let dist = WeightedIndex::new(p).unwrap();

    for i in 0..300000 {
        let r = d.row(dist.sample(&mut rng));
        x = r[0] * x + r[1] * y + r[4];
        y = r[2] * x + r[3] * y + r[5];

        // Skip first few iterations
        if i >= 10 {
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
