// Initial code based on https://github.com/remexre/mandelbrot-rust-gl

mod shaders;
mod support;

use std::time::Instant;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{IndicesSource, NoIndices, PrimitiveType};
use glium::{implement_vertex, Display, Program, Surface};
use glium::uniforms::{Uniforms, UniformValue};
use imgui_glium_renderer::Renderer;
use imgui::{Condition, Context};
use imgui_winit_support::{HiDpiMode, WinitPlatform};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[derive(Debug)]
struct DrawParams {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,

    width: f32,
    height: f32,
    zoom: f32,
    shift_r: f32,
    shift_x: f32,
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
            zoom: 1.0,
            shift_x: 0.5,
            shift_r: 1.0,
        }
    }
}

impl Uniforms for DrawParams {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("xMin", UniformValue::Double(self.x_min));
        f("xMax", UniformValue::Double(self.x_max));
        f("yMin", UniformValue::Double(self.y_min));
        f("yMax", UniformValue::Double(self.y_max));
        f("width", UniformValue::Float(self.width));
        f("height", UniformValue::Float(self.height));
        f("zoom", UniformValue::Float(self.zoom));
        f("shift_r", UniformValue::Float(self.shift_r));
        f("shift_x", UniformValue::Float(self.shift_x));
    }
}

fn main() {
    let mut event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024f64, 768f64))
        .with_title("Hello world");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();

        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), window, dpi_mode);
    }
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let screen_width = display.get_framebuffer_dimensions().0 as i32;
    dbg!(screen_width);

    let d = 2.0 / (screen_width as f32);
    let mut shape = (-screen_width..screen_width)
        .into_iter()
        .map(|x| Vertex {
            position: [d * (x as f32), 0.0],
        })
        .collect::<Vec<_>>();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let program = Program::from_source(
        &display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/fragment.glsl"),
        Some(include_str!("shaders/geometry.glsl")),
    )
    .unwrap();

    let mut draw_params = DrawParams::new(display.get_framebuffer_dimensions());

    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];

    let mut last_frame = Instant::now();
    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = Instant::now();
            imgui.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::MainEventsCleared => {
            let gl_window = display.gl_window();
            platform
                .prepare_frame(imgui.io_mut(), gl_window.window())
                .expect("Failed to prepare frame");
            gl_window.window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            // Draw GUI
            let ui = imgui.frame();
            ui.window("Hello world")
                .size([300.0, 110.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.text_wrapped("Hello world!");
                    ui.text_wrapped("こんにちは世界！");
                    if ui.button(choices[value]) {
                        value += 1;
                        value %= 2;
                    }

                    ui.button("This...is...imgui-rs!");
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &draw_params,
                    &Default::default(),
                )
                .unwrap();

            let data = imgui.render();
            renderer.render(&mut target, data).expect("Rendering failed");

            let gl_window = display.gl_window();
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
            platform.prepare_render(ui, gl_window.window());
            let draw_data = imgui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        event => {
            let gl_window = display.gl_window();
            platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
        }
    });
}
