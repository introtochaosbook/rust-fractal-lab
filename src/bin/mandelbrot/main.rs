// Initial code based on https://github.com/remexre/mandelbrot-rust-gl

use std::borrow::Borrow;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{DeviceEvent, Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::pixel_buffer::PixelBuffer;
use glium::uniforms::{UniformValue, Uniforms};
use glium::{Display, DrawParameters, Program, Surface, VertexBuffer};
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::vertex::Vertex;

#[derive(Debug)]
struct DrawParams {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,

    width: f32,
    height: f32,
    max_colors: u32,
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

    let color1 = glium::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        1024,
        768,
    )
    .unwrap();
    color1.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    let depth = glium::framebuffer::DepthRenderBuffer::new(
        &display,
        glium::texture::DepthFormat::F32,
        1024, 768,
    ).unwrap();

    let texture = glium::texture::DepthTexture2d::empty(&display, 1024, 768)
        .unwrap();

    // building the framebuffer
    let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color1, &texture).unwrap();

    let draw_params = DrawParams::new(display.get_framebuffer_dimensions());

    framebuffer
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &draw_params,
            &Default::default(),
        )
        .unwrap();

    display.assert_no_error(None);

    let p: Vec<Vec<(u8, u8, u8, u8)>> = color1.read();
    let p: Vec<_> = p.iter().flatten().collect();

    let recover = &(
        1.0f32,
        1.0f32 / 255.0,
        1.0f32 / 65025.0,
        1.0f32 / 16581375.0,
    );

    //p.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for pixel in p.iter() {
        if pixel.0 == 0 {
            continue;
        }
        assert_eq!(pixel.0, 2);
    }

    event_loop.run(move |ev, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { .. } | DeviceEvent::Motion { .. },
                ..
            } => return,
            _ => (),
        }

        let draw_params = DrawParams::new(display.get_framebuffer_dimensions());

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
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
