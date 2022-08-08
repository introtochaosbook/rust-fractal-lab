// Initial code based on https://github.com/remexre/mandelbrot-rust-gl


use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{DeviceEvent, Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::UniformBuffer;
use glium::{implement_uniform_block, uniform, Display, Program, Surface, VertexBuffer};
use rust_fractal_lab::vertex::Vertex;

#[derive(Copy, Clone)]
struct UniformBlock2 {
    colors_r: [f32; 256],
    _padding: [f32; 512],
    _padding2: [f32; 256],
    colors_g: [f32; 256],
    _padding3: [f32; 512],
    _padding4: [f32; 256],
    colors_b: [f32; 256],
}

impl UniformBlock2 {
    fn new(colors_r: [f32; 256], colors_g: [f32; 256], colors_b: [f32; 256]) -> Self {
        Self {
            colors_r,
            colors_g,
            colors_b,
            _padding: [0.0; 512],
            _padding2: [0.0; 256],
            _padding3: [0.0; 512],
            _padding4: [0.0; 256],
        }
    }
}

implement_uniform_block!(UniformBlock2, colors_r, colors_g, colors_b);

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let vertices = [
        Vertex {
            position: [1.0, -1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
        },
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
        include_str!("shaders/fragment.glsl"),
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
                WindowEvent::MouseInput { .. } => return,
                _ => return,
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { .. } | DeviceEvent::Motion { .. },
                ..
            } => return,
            _ => (),
        }

        let dims = display.get_framebuffer_dimensions();

        let gradient = colorous::INFERNO;
        let max_colors: usize = 256;
        let mut colors_r: [f32; 256] = [0.0; 256];
        let mut colors_g: [f32; 256] = [0.0; 256];
        let mut colors_b: [f32; 256] = [0.0; 256];
        for i in 0..max_colors {
            let color = gradient.eval_rational(i, max_colors + 1);
            colors_r[i] = (color.r as f32) / 255.0;
            colors_g[i] = (color.g as f32) / 255.0;
            colors_b[i] = (color.b as f32) / 255.0;
        }

        let buffer =
            UniformBuffer::new(&display, UniformBlock2::new(colors_r, colors_g, colors_b)).unwrap();

        let uniforms = uniform! {
            Block: &buffer,
            xMin: -2.0,
            xMax: 1.0,
            yMin: -1.0,
            yMax: 1.0,
            width: dims.0 as f64,
            height: dims.1 as f64,
            max_colors: max_colors as u16,
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
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
