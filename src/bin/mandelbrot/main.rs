// Initial code based on https://github.com/remexre/mandelbrot-rust-gl

use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::{UniformValue, Uniforms, UniformBuffer, UniformsStorage};
use glium::{Display, implement_uniform_block, Program, Surface, uniform, VertexBuffer};
use rust_fractal_lab::vertex::Vertex;
use colorous;

#[derive(Copy, Clone)]
struct UniformBlock2 {
    colors_r: [f32; 256],
    colors_g: [f32; 256],
    colors_b: [f32; 256],
}

impl UniformBlock2 {
    fn new(colors_r: [f32; 256], colors_g: [f32; 256], colors_b: [f32; 256]) -> Self {
        Self {
            colors_r,
            colors_g,
            colors_b,
        }
    }
}

implement_uniform_block!(UniformBlock2, colors_r, colors_g, colors_b);


fn main() {
    let mut event_loop = EventLoop::new();

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
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/fragment.glsl"),
        None,
    )
    .unwrap();

    let dims = display.get_framebuffer_dimensions();

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
                WindowEvent::MouseInput { .. } => return,
                _ => return,
            },
            _ => (),
        }

        let gradient = colorous::TURBO;
        let max_colors: usize = 80;
        let mut colors_r: [f32; 256] = [0.0; 256];
        let mut colors_g: [f32; 256] = [0.0; 256];
        let mut colors_b: [f32; 256] = [0.0; 256];
        for i in 0..max_colors {
            let color = gradient.eval_rational(i, max_colors + 1);
            colors_r[i] = (color.r as f32) / 255.0;
            colors_g[i] = (color.g as f32) / 255.0;
            colors_b[i] = (color.b as f32) / 255.0;
        }

        let buffer = UniformBuffer::new(
            &display,
            UniformBlock2::new(colors_r, colors_g, colors_b),
        ).unwrap();

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
