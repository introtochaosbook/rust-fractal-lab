use clap::Parser;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::{DeviceEvent, Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::program::ShaderStage;
use glium::uniforms::{UniformValue, Uniforms};
use glium::{Display, Program, Surface, VertexBuffer};
use rust_fractal_lab::args::{ColorScheme, JuliaFunction};
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::vertex::Vertex;

#[derive(Parser)]
pub struct JuliaArgs {
    #[arg(value_enum)]
    julia_function: JuliaFunction,

    #[arg(value_enum, default_value_t = ColorScheme::Turbo, short, long)]
    color_scheme: ColorScheme,
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
    f: String,
    color_map: String,
}

impl DrawParams {
    fn new(dims: (u32, u32), args: &JuliaArgs) -> DrawParams {
        DrawParams {
            x_min: -2.0,
            x_max: 2.0,
            y_min: -2.0,
            y_max: 2.0,
            width: dims.0 as f32,
            height: dims.1 as f32,
            max_colors: 10,
            f: args.julia_function.subroutine_name(),
            color_map: args.color_scheme.subroutine_name(),
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
        f(
            "F",
            UniformValue::Subroutine(ShaderStage::Fragment, self.f.as_str()),
        );
        f(
            "ColorMap",
            UniformValue::Subroutine(ShaderStage::Fragment, self.color_map.as_str()),
        );
        f(
            "Colorize",
            UniformValue::Subroutine(ShaderStage::Fragment, {
                match self.f.as_str() {
                    "FCloud" => "ColorizeCloud",
                    "FSnowflakes" => "ColorizeSnowflakes",
                    _ => "ColorizeDefault",
                }
            }),
        );
    }
}

fn main() {
    let args = JuliaArgs::parse();

    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1024.0, 768.0))
        .with_title("Julia set");

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

        let draw_params = DrawParams::new(display.get_framebuffer_dimensions(), &args);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                indices,
                &program,
                &draw_params,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
