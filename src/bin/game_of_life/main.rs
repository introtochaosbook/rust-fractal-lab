use std::mem::swap;
use std::ops::Add;
use std::time::{Duration, Instant};

use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};
use glium::glutin::event::{
    DeviceEvent, ElementState, Event, MouseButton, StartCause, WindowEvent,
};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::RawImage2d;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{uniform, Display, Program, Rect, Surface, Texture2d, VertexBuffer};
use rand::distributions::Bernoulli;
use rand::distributions::Distribution;
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::utils::winit::WindowBuilderHelpers;
use rust_fractal_lab::vertex::Vertex;
use static_assertions::const_assert_eq;

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;
const SCALE: u32 = 8;

// Height and width should be divisible by scale
const_assert_eq!(WINDOW_WIDTH % SCALE, 0);
const_assert_eq!(WINDOW_HEIGHT % SCALE, 0);

// Height and width must be powers of 2 for wraparound to work
const_assert_eq!(WINDOW_WIDTH & (WINDOW_WIDTH - 1), 0);
const_assert_eq!(WINDOW_HEIGHT & (WINDOW_HEIGHT - 1), 0);

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size_centered(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT), &event_loop)
        .with_title("Hello world")
        .with_resizable(false);

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

    let row_count = WINDOW_HEIGHT / SCALE;
    let col_count = WINDOW_WIDTH / SCALE;

    let dist = Bernoulli::new(0.3).unwrap();
    let mut pixels = Vec::with_capacity((row_count * col_count * 4) as usize);
    let rng = rand::thread_rng();
    let samples = (&dist)
        .sample_iter(rng)
        .take((row_count * col_count) as usize)
        .flat_map(|sample| {
            if sample {
                [255.0, 255.0, 255.0, 255.0]
            } else {
                [0.0, 0.0, 0.0, 255.0]
            }
        });
    pixels.extend(samples);

    let image = RawImage2d::from_raw_rgba(pixels, (row_count, col_count));
    let texture1 =
        Texture2d::with_mipmaps(&display, image, glium::texture::MipmapsOption::NoMipmap).unwrap();

    let texture2 = Texture2d::empty_with_mipmaps(
        &display,
        glium::texture::MipmapsOption::NoMipmap,
        WINDOW_WIDTH / SCALE,
        WINDOW_HEIGHT / SCALE,
    )
    .unwrap();

    let textures = [texture1, texture2];

    let vertex_shader = r##"#version 140
in vec2 position;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
}
"##;

    let program = Program::from_source(
        &display,
        vertex_shader,
        &build_shader(include_str!("shaders/fragment.glsl")),
        None,
    )
    .unwrap();

    let program2 = Program::from_source(
        &display,
        vertex_shader,
        &build_shader(include_str!("shaders/fragment-2.glsl")),
        None,
    )
    .unwrap();

    let mut a = 0;
    let mut b = 1;

    let mut cursor_position: Option<PhysicalPosition<i32>> = None;
    let mut pressed = false;

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                    match state {
                        ElementState::Pressed => pressed = true,
                        ElementState::Released => { pressed = false;
                            let mut data = Vec::new();
                            for _ in 0..1 {
                                data.push((255.0, 255.0, 255.0, 255.0));
                            }
                            let mut data2 = Vec::new();
                            for _ in 0..1 {
                                data2.push(data.clone());
                            }
                            
                            textures[a].write(Rect { left: (cursor_position.as_ref().unwrap().x as u32) / SCALE, bottom: (WINDOW_HEIGHT - cursor_position.as_ref().unwrap().y as u32) / SCALE, width: 1, height: 1 }, data2)
                        },
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    cursor_position = Some(position.cast::<i32>());
                    if pressed {
                        let mut data = Vec::new();
                        for _ in 0..1 {
                            data.push((255.0, 255.0, 255.0, 255.0));
                        }
                        let mut data2 = Vec::new();
                        for _ in 0..1 {
                            data2.push(data.clone());
                        }

                        textures[a].write(Rect { left: (cursor_position.as_ref().unwrap().x as u32) / SCALE, bottom: (WINDOW_HEIGHT - cursor_position.as_ref().unwrap().y as u32) / SCALE, width: 1, height: 1 }, data2)
                    }

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
            Event::RedrawRequested(..) => {}
            _ => return,
        }

        *control_flow = ControlFlow::WaitUntil(Instant::now().add(Duration::from_millis(10)));

        eprintln!("drawing...");

            // Input is a
            let draw_params = uniform! {
                state: glium::uniforms::Sampler::new(&textures[a]).magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest),
                scale: [WINDOW_WIDTH / SCALE, WINDOW_HEIGHT / SCALE],
            };

            let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
            let indices = NoIndices(PrimitiveType::TrianglesList);

            // Compute b from a
            textures[b]
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
                state: glium::uniforms::Sampler::new(&textures[b]).magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest),
                scale: [WINDOW_WIDTH, WINDOW_HEIGHT],
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
}
