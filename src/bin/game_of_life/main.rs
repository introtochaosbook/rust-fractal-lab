use glium::framebuffer::SimpleFrameBuffer;
use glium::glutin::dpi::{LogicalSize, PhysicalSize};
use glium::glutin::event::{DeviceEvent, Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::RawImage2d;
use glium::uniforms::{UniformValue, Uniforms};
use glium::{uniform, Display, Program, Surface, Texture2d, VertexBuffer};
use imgui::StyleColor::Text;
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::vertex::Vertex;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

pub struct Dt {
    textures: [glium::texture::Texture2d; 2],
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

        tenants.with_mut(|fields| {
            let a = &mut fields.buffs.0;
            let b = &mut fields.buffs.1;
            let dt = fields.dt;

            let draw_params = uniform! {
                state: &dt.textures[0],
                scale: [WINDOW_WIDTH / 4, WINDOW_HEIGHT / 4],
            };

            b.draw(
                &vertex_buffer,
                &indices,
                &program,
                &draw_params,
                &Default::default(),
            )
            .unwrap();

            let draw_params = uniform! {
                state: &dt.textures[0],
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
    });
}
