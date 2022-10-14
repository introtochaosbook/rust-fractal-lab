use glium::glutin::dpi::{LogicalSize, PhysicalSize};
use glium::glutin::event::{DeviceEvent, Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::{UniformValue, Uniforms};
use glium::{Display, Program, Surface, Texture2d, uniform, VertexBuffer};
use glium::texture::RawImage2d;
use imgui::StyleColor::Text;
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::vertex::Vertex;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
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

    // let back_texture = Texture2d::empty(
    //     &display,
    //     WINDOW_WIDTH,
    //     WINDOW_HEIGHT,
    // )
    //     .unwrap();

    let mut row = Vec::with_capacity((WINDOW_WIDTH) as usize);
    for i in 0..WINDOW_WIDTH {
        if i % 2 == 0 {
            row.push((255.0, 255.0, 255.0, 255.0));
        } else {
            row.push((0.0, 0.0, 0.0, 255.0));
        }
    }

    let mut pixels = Vec::with_capacity((WINDOW_HEIGHT) as usize);
    for _ in 0..WINDOW_HEIGHT {
        pixels.push(row.clone());
    }

    let back_texture = Texture2d::new(&display, pixels).unwrap();

    let last_texture = Texture2d::empty(&display, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

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

    let program2 = Program::from_source(
        &display,
        r##"#version 140
in vec2 position;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
}
"##,
        &build_shader(include_str!("shaders/fragment-2.glsl")),
        None,
    )
        .unwrap();

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

        let draw_params = uniform! {
            state: &back_texture,
            scale: [WINDOW_WIDTH / 4, WINDOW_HEIGHT / 4],
        };

        last_texture.as_surface().draw(
            &vertex_buffer,
            &indices,
            &program,
            &draw_params,
            &Default::default(),
        )
            .unwrap();
        last_texture.sync_shader_writes_for_surface();

        let draw_params = uniform! {
            state: &last_texture,
            scale: [WINDOW_WIDTH / 4, WINDOW_HEIGHT / 4],
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
    });
}
