use crate::ControlFlow::Wait;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use std::iter::{self};

use glium::{implement_vertex, Display, DrawParameters, Program, Surface, VertexBuffer};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Vertex {
    fn x(&self) -> f32 {
        self.position[0]
    }

    fn y(&self) -> f32 {
        self.position[1]
    }
}

impl Into<Vertex> for [f32; 2] {
    fn into(self) -> Vertex {
        Vertex { position: self }
    }
}

struct Line(Vertex, Vertex);

impl Line {
    fn into_vertices(self) -> impl Iterator<Item = Vertex> {
        iter::once(self.0).chain(iter::once(self.1))
    }
}

fn cantor<L, R>(lines: &mut Vec<Line>, left: L, right: R, depth: u8)
where
    L: Into<Vertex>,
    R: Into<Vertex>,
{
    let left = left.into();
    let right = right.into();
    lines.push(Line(left, right));

    // Keep track of recursion depth
    if depth >= 6 {
        return;
    }

    // Shift subsequent lines down a bit
    let y = left.y() - 0.05;
    // Calculate third of line segment
    let delta = (right.x() - left.x()) / 3.0;
    // Draw left third
    cantor(lines, [left.x(), y], [left.x() + delta, y], depth + 1);
    // Draw right third
    cantor(lines, [right.x() - delta, y], [right.x(), y], depth + 1);
}

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(768.0 as f32, 768.0 as f32))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let mut lines = vec![];
    cantor(&mut lines, [-1.0, 0.95], [1.0, 0.95], 0);

    let vertices: Vec<_> = lines
        .into_iter()
        .map(Line::into_vertices)
        .flatten()
        .collect();

    let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
    let indices = NoIndices(PrimitiveType::LinesList);

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
        let mut params = DrawParameters::default();
        params.line_width = Some(4.0);
        target
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();
        target.finish().unwrap();
    });
}
