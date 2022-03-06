// Initial code based on https://github.com/remexre/mandelbrot-rust-gl

mod shaders;

use crate::ControlFlow::WaitUntil;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{IndicesSource, NoIndices, PrimitiveType};
use glium::uniforms::{UniformValue, Uniforms};
use glium::{implement_vertex, Display, Program, Surface, VertexBuffer};
use std::ops::Range;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[derive(Debug)]
struct DrawParams {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,

    width: f32,
    height: f32,
    zoom: f32,
    shift_r: f32,
    shift_x: f32,
}

impl DrawParams {
    fn new(dims: (u32, u32)) -> DrawParams {
        DrawParams {
            x_min: -2.0,
            x_max: 1.0,
            y_min: -1.0,
            y_max: 1.0,
            width: dims.0 as f32,
            height: dims.1 as f32,
            zoom: 1.0,
            shift_x: 0.5,
            shift_r: 1.0,
        }
    }
}

impl Uniforms for DrawParams {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("xMin", UniformValue::Double(self.x_min));
        f("xMax", UniformValue::Double(self.x_max));
        f("yMin", UniformValue::Double(self.y_min));
        f("yMax", UniformValue::Double(self.y_max));
        f("width", UniformValue::Float(self.width));
        f("height", UniformValue::Float(self.height));
        f("zoom", UniformValue::Float(self.zoom));
        f("shift_r", UniformValue::Float(self.shift_r));
        f("shift_x", UniformValue::Float(self.shift_x));
    }
}

fn main() {
    let mut event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();
    let screen_width = display.get_framebuffer_dimensions().0 as i32;

    dbg!(screen_width);

    let d = 2.0 / (screen_width as f32);
    let mut shape = (-screen_width..screen_width)
        .into_iter()
        .map(|x| Vertex {
            position: [d * (x as f32), 0.0],
        })
        .collect::<Vec<_>>();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let program = Program::from_source(
        &display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/fragment.glsl"),
        Some(include_str!("shaders/geometry.glsl")),
    )
    .unwrap();

    let mut draw_params = DrawParams::new(display.get_framebuffer_dimensions());

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);
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
        target.clear_color(0.0, 1.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &draw_params,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
