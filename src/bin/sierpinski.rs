use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::{Display, Program, Surface, VertexBuffer};
use rand::Rng;
use rust_fractal_lab::vertex::Vertex;

use crate::ControlFlow::Wait;

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(768.0, 768.0))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let mut rng = rand::thread_rng();
    let mut vertices = vec![];

    // Initial starting point
    let mut x = rng.gen_range(-1.0..=1.0);
    let mut y = rng.gen_range(-1.0..=1.0);

    for i in 0..200000 {
        // Generate random number in range [0, 2]
        match rng.gen_range(0..=2) {
            0 => {
                // rule 1
                x = (-1.0 + x) / 2.0;
                y = (-1.0 + y) / 2.0;
            }
            1 => {
                // rule 2
                x /= 2.0;
                y = (1.0 + y) / 2.0;
            }
            2 => {
                // rule 3
                x = (1.0 + x) / 2.0;
                y = (-1.0 + y) / 2.0;
            }
            _ => unreachable!(),
        }

        // Skip first 1000 iterations
        if i >= 1000 {
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
        let _next_frame_time =
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
