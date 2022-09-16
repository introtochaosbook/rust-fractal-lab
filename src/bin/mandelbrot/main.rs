// Initial code based on https://github.com/remexre/mandelbrot-rust-gl

use crate::ControlFlow::{Wait, WaitUntil};
use glium::framebuffer::{ColorAttachment, MultiOutputFrameBuffer, ToColorAttachment};
use glium::glutin::dpi::{LogicalSize, PhysicalSize};
use glium::glutin::event::{DeviceEvent, Event, StartCause, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::pixel_buffer::PixelBuffer;
use glium::texture::{Texture1d, UnsignedTexture2d};
use glium::uniforms::{UniformValue, Uniforms};
use glium::{Display, DrawParameters, Program, Surface, Texture2d, VertexBuffer};
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::vertex::Vertex;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use std::time::{Duration, Instant};
use glium::program::ShaderStage;

use ouroboros::self_referencing;

pub struct Dt {
    color_texture: Texture2d,
    iteration_texture: UnsignedTexture2d,
}

#[self_referencing]
struct Data {
    dt: Dt,
    #[borrows(dt)]
    #[covariant]
    buffs: (glium::framebuffer::MultiOutputFrameBuffer<'this>, &'this Dt),
}

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
    ranges_2: [u32; 4],
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
            max_colors: 1024,
            ranges: [0; 4],
            ranges_2: [0; 4],
            color: "ColorInferno".into(),
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
        f("ranges_2", UniformValue::UnsignedIntVec4(self.ranges_2));
        f(
            "Color",
            UniformValue::Subroutine(ShaderStage::Fragment, self.color.as_str()),
        );
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1024.0, 768.0))
        .with_resizable(false)
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

    let iteration_texture = UnsignedTexture2d::empty_with_format(
        &display,
        glium::texture::UncompressedUintFormat::U32U32,
        glium::texture::MipmapsOption::NoMipmap,
        1024,
        768,
    )
    .unwrap();

    iteration_texture
        .as_surface()
        .clear_color(0.0, 0.0, 0.0, 0.0);

    let color_texture = Texture2d::empty(
        &display, 1024, 768
    ).unwrap();

    let mut tenants = DataBuilder {
        dt: Dt {
            color_texture,
            iteration_texture,
        },
        buffs_builder: |dt| {
            let output =  [("color", dt.color_texture.to_color_attachment()), ("depth", dt.iteration_texture.to_color_attachment())];
            let framebuffer = MultiOutputFrameBuffer::new(&display, output).unwrap();
            (framebuffer, dt)
        }
    }.build();

    let dim = display.get_framebuffer_dimensions();
    eprintln!("{:?}", dim);
    let mut draw_params = DrawParams::new(display.get_framebuffer_dimensions());

    event_loop.run(move |ev, _, control_flow| {
        tenants.with_mut(|fields| {
            *control_flow = ControlFlow::WaitUntil(Instant::now().add(Duration::from_millis(100)));

            match ev {
                Event::RedrawRequested(_) | Event::NewEvents(StartCause::ResumeTimeReached {..}) => {
                    let framebuffer = &mut fields.buffs.0;
                    let dt = fields.dt;

                    framebuffer.draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &draw_params,
                        &Default::default(),
                    )
                        .unwrap();

                    display.assert_no_error(None);

                    let p: Vec<Vec<(u32, u32)>> = unsafe { dt.iteration_texture.unchecked_read() };

                    let mut p: Vec<_> = p
                        .into_iter()
                        .flatten()
                        .filter(|b| b.1 != 1)
                        .map(|b| b.0)
                        .collect();
                    p.sort_unstable();

                    draw_params.ranges = [
                        p[0],
                        p[p.len() * 1 / 7 - 1] + 1,
                        p[p.len() * 2 / 7 - 1] + 2,
                        p[p.len() * 3 / 7 - 1] + 3,
                    ];

                    draw_params.ranges_2 = [
                        p[p.len() * 4 / 7 - 1] + 3,
                        p[p.len() * 5 / 7 - 1] + 3,
                        p[p.len() * 6 / 7 - 1],
                        *p.last().unwrap(),
                    ];

                    eprintln!("{:?}", draw_params.ranges);
                    eprintln!("{:?}", draw_params.ranges_2);

                    let target = display.draw();
                    dt.color_texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);
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
    });
}
