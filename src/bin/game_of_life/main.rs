use glium::framebuffer::SimpleFrameBuffer;
use glium::glutin::dpi::{LogicalSize, PhysicalSize};
use glium::glutin::event::{DeviceEvent, Event, StartCause, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::RawImage2d;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter, UniformValue, Uniforms};
use glium::{
    implement_vertex, uniform, Display, DrawParameters, Program, Surface, Texture2d, VertexBuffer,
};
use rand::Rng;
use rust_fractal_lab::shader_builder::build_shader;
use static_assertions::const_assert_eq;
use std::mem::swap;
use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2], // <- this is new
}

implement_vertex!(Vertex, position, tex_coords); // don't forget to add `tex_coords` here
                                                 //
                                                 // impl From<[f32; 2]> for Vertex {
                                                 //     fn from(x: [f32; 2]) -> Self {
                                                 //         Vertex { position: x, tex_coords: [] }
                                                 //     }
                                                 // }
                                                 //

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;
const SCALE: u32 = 4;

const_assert_eq!(WINDOW_WIDTH % SCALE, 0);
const_assert_eq!(WINDOW_HEIGHT % SCALE, 0);

pub struct Dt {
    textures: [Texture2d; 2],
}

#[ouroboros::self_referencing]
struct Data {
    dt: Dt,
    #[borrows(dt)]
    #[covariant]
    buffs: (
        glium::framebuffer::SimpleFrameBuffer<'this>,
        glium::framebuffer::SimpleFrameBuffer<'this>,
        &'this Dt,
    ),
}

fn main() {
    let event_loop = EventLoop::new();
    let mut rng = rand::thread_rng();

    let wb = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let vertices: [Vertex; 4] = [
        Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
    ];

    // TODO improve
    let mut pixels = Vec::with_capacity((WINDOW_HEIGHT / SCALE) as usize);
    for _ in 0..WINDOW_HEIGHT / SCALE {
        let mut row = Vec::with_capacity((WINDOW_WIDTH / SCALE) as usize);
        for i in 0..WINDOW_WIDTH / SCALE {
            if rng.gen_bool(0.5) {
                row.push((255.0, 255.0, 255.0, 255.0));
            } else {
                row.push((0.0, 0.0, 0.0, 255.0));
            }
        }

        pixels.push(row);
    }

    let back_texture =
        Texture2d::with_mipmaps(&display, pixels, glium::texture::MipmapsOption::NoMipmap).unwrap();

    let last_texture = Texture2d::empty_with_mipmaps(
        &display,
        glium::texture::MipmapsOption::NoMipmap,
        WINDOW_WIDTH / SCALE,
        WINDOW_HEIGHT / SCALE,
    )
    .unwrap();

    let mut tenants = DataBuilder {
        dt: Dt {
            textures: [back_texture, last_texture],
        },
        buffs_builder: |dt| {
            let a = SimpleFrameBuffer::new(&display, &dt.textures[0]).unwrap();
            let b = SimpleFrameBuffer::new(&display, &dt.textures[1]).unwrap();

            (a, b, dt)
        },
    }
    .build();

    let program = Program::from_source(
        &display,
        r##"#version 140
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
	v_tex_coords = tex_coords;
}
"##,
        &build_shader(include_str!("shaders/fragment.glsl")),
        None,
    )
    .unwrap();

    let program2 = Program::from_source(
        &display,
        r##"#version 140
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
	v_tex_coords = tex_coords;
}
"##,
        &build_shader(include_str!("shaders/fragment-2.glsl")),
        None,
    )
    .unwrap();

    let mut a = 0;
    let mut b = 1;

    event_loop.run(move |ev, _, control_flow| {
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
            Event::NewEvents(s) => match s {
                StartCause::ResumeTimeReached { .. } => {}
                StartCause::Init => {}
                _ => return,
            },
            _ => return,
        }

        *control_flow = ControlFlow::WaitUntil(Instant::now().add(Duration::from_millis(100)));

        eprintln!("drawing...");
        tenants.with_mut(|fields| {
            let dt = fields.dt;

            // Input is a
            let draw_params = uniform! {
                state: glium::uniforms::Sampler::new(&dt.textures[a]).magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest),
                scale: [WINDOW_WIDTH / SCALE, WINDOW_HEIGHT / SCALE],
            };

            let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
            let indices = glium::IndexBuffer::new(
                &display,
                PrimitiveType::TriangleStrip,
                &[1 as u16, 2, 0, 3],
            )
                .unwrap();

            // Compute b from a
            dt.textures[b]
                .as_surface()
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &draw_params,
                    &Default::default(),
                )
                .unwrap();

            let draw_params = uniform! {
                state: glium::uniforms::Sampler::new(&dt.textures[b]).magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest),
                scale: [WINDOW_WIDTH / SCALE, WINDOW_HEIGHT / SCALE],
            };

            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);

            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program2,
                    &draw_params,
                    &Default::default(),
                )
                .unwrap();
            target.finish().unwrap();

            swap(&mut a, &mut b);
        });
    });
}
