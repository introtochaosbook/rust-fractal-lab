use std::time::Instant;

use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::{UniformValue, Uniforms};
use glium::{glutin, Program, VertexBuffer};
use glium::{Display, Surface};
use imgui::{Condition, Context, SliderFlags};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use rust_fractal_lab::vertex::Vertex;

#[derive(Debug)]
struct DrawParams {
    pan_hor: f32,
    pan_vert: f32,
    zoom: f32,
    c_range: [f32; 2],
}

impl Default for DrawParams {
    fn default() -> DrawParams {
        DrawParams {
            pan_vert: 0.0,
            zoom: 0.5,
            pan_hor: 1.0,
            c_range: [-2.0, -1.0],
        }
    }
}

impl Uniforms for DrawParams {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("pan_vert", UniformValue::Float(self.pan_vert));
        f("zoom", UniformValue::Float(self.zoom));
        f("pan_hor", UniformValue::Float(self.pan_hor));
        f("c_range", UniformValue::Vec2(self.c_range));
    }
}

fn main() {
    let title = "Bifurcation diagram";

    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title(title.to_owned())
        .with_inner_size(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    let gl_window = display.gl_window();
    let window = gl_window.window();
    platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);
    drop(gl_window);

    let screen_width = display.get_framebuffer_dimensions().0;

    // Create n points along the x-axis, where n = screen_width
    let vertices: Vec<Vertex> = (0..screen_width)
        .map(|x| Vertex {
            // The OpenGL coordinate system has all coordinates between
            // -1.0 and 1.0, so we need to do some scaling.
            position: [-1.0 + 2.0 * (x as f32) / (screen_width as f32), 0.0],
        })
        .collect::<Vec<_>>();

    // Also valid:
    // let mut vertices = vec![];
    // for x in 0..screen_width {
    //     vertices.push(Vertex {
    //         position: [-1.0 + 2.0 * (x as f32) / (screen_width as f32), 0.0],
    //     })
    // }

    let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
    let indices = NoIndices(PrimitiveType::Points);

    let program = Program::from_source(
        &display,
        r##"#version 140
in vec2 position;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
}
"##,
        include_str!("shaders/fragment.glsl"),
        Some(include_str!("shaders/geometry.glsl")),
    )
    .unwrap();

    let mut draw_params = DrawParams::default();

    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

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
            let ui = imgui.new_frame();

            ui.window("Controls")
                .size([300.0, 150.0], Condition::FirstUseEver)
                .position([600.0, 50.0], Condition::FirstUseEver)
                .build(|| {
                    ui.slider("pan_hor", -2.0, 2.0, &mut draw_params.pan_hor);
                    ui.slider("pan_vert", -2.0, 2.0, &mut draw_params.pan_vert);
                    ui.slider("zoom", 0.001, 10.0, &mut draw_params.zoom);
                    ui.slider_config("c min/max", -2.0_f32, 2.0_f32)
                        .flags(SliderFlags::ALWAYS_CLAMP)
                        .build_array(&mut draw_params.c_range);

                    if ui.button("Reset") {
                        draw_params = DrawParams::default();
                    }
                });

            let gl_window = display.gl_window();
            let mut target = display.draw();
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &draw_params,
                    &Default::default(),
                )
                .unwrap();
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
