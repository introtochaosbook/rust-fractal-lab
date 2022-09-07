// Initial code based on https://github.com/remexre/mandelbrot-rust-gl

use crate::ControlFlow::Wait;
use glium::framebuffer::{MultiOutputFrameBuffer, ToColorAttachment};
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{DeviceEvent, Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::pixel_buffer::PixelBuffer;
use glium::texture::UnsignedTexture2d;
use glium::uniforms::{UniformValue, Uniforms};
use glium::{Display, DrawParameters, Program, Surface, Texture2d, VertexBuffer};
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::vertex::Vertex;
use std::borrow::Borrow;
use glium::program::ShaderStage;

#[derive(Debug)]
struct DrawParams {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,

    width: f32,
    height: f32,
    max_colors: u32,
    ranges: [u32; 4],
    color: String,
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
            max_colors: 256,
            ranges: [0; 4],
            color: "ColorTurbo".into(),
        }
    }
}

impl Uniforms for DrawParams {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("xMin", UniformValue::Float(self.x_min));
        f("xMax", UniformValue::Float(self.x_max));
        f("yMin", UniformValue::Float(self.y_min));
        f("yMax", UniformValue::Float(self.y_max));
        f("width", UniformValue::Float(self.width));
        f("height", UniformValue::Float(self.height));
        f("maxColors", UniformValue::UnsignedInt(self.max_colors));
        f("ranges", UniformValue::UnsignedIntVec4(self.ranges));
        f(
            "Color",
            UniformValue::Subroutine(ShaderStage::Fragment, self.color.as_str()),
        );
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let vertices: [Vertex; 6] = [
        [1.0, -1.0].into(),
        [-1.0, 1.0].into(),
        [-1.0, -1.0].into(),
        [1.0, 1.0].into(),
        [1.0, -1.0].into(),
        [-1.0, 1.0].into(),
    ];

    let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
    let indices = NoIndices(PrimitiveType::TrianglesList);

    let program = Program::from_source(
        &display,
        r##"#version 140
in vec2 position;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
}
"##,
        &build_shader(include_str!("shaders/fragment.glsl")),
        None,
    )
    .unwrap();

    let program_step_2 = Program::from_source(
        &display,
        r##"#version 140
in vec2 position;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
}
"##,
        &build_shader(include_str!("shaders/fragment-step-2.glsl")),
        None,
    )
    .unwrap();

    let iteration_texture = UnsignedTexture2d::empty_with_format(
        &display,
        glium::texture::UncompressedUintFormat::U32U32,
        glium::texture::MipmapsOption::NoMipmap,
        1024,
        768,
    ).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        *control_flow = Wait;

        match ev {
            Event::RedrawRequested(_) => {
                let mut draw_params = DrawParams::new(display.get_framebuffer_dimensions());

                // building the framebuffer
                let mut framebuffer =
                    glium::framebuffer::SimpleFrameBuffer::new(&display, &iteration_texture).unwrap();

                framebuffer
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &draw_params,
                        &Default::default(),
                    )
                    .unwrap();

                let p: Vec<Vec<(u32, u32)>> = unsafe { iteration_texture.unchecked_read() };

                let mut p: Vec<_> = p
                    .into_iter()
                    .flatten()
                    .filter(|b| b.1 != 1)
                    .map(|b| b.0)
                    .collect();
                p.sort_unstable();

                draw_params.ranges = [
                    p[0],
                    p[p.len() * 3 / 4 - 1],
                    p[p.len() * 7 / 8 - 1],
                    *p.last().unwrap(),
                ];

                eprintln!("{:?}", draw_params.ranges);

                let mut target = display.draw();
                target.clear_color(255.0, 255.0, 255.0, 1.0);
                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program_step_2,
                        &draw_params,
                        &Default::default(),
                    )
                    .unwrap();
                target.finish().unwrap();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
