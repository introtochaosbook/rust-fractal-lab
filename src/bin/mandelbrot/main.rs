// Scaling code based on https://github.com/remexre/mandelbrot-rust-gl

use glium::framebuffer::{MultiOutputFrameBuffer, ToColorAttachment};
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::{
    ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent,
};
use glium::glutin::ContextBuilder;
use glium::{Display, Program, Surface, Texture2d, VertexBuffer};
use std::time::Instant;

use glium::texture::UnsignedTexture2d;
use glium::uniforms::{UniformValue, Uniforms};

use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::program::ShaderStage;
use imgui::{Condition, Context};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use ouroboros::self_referencing;
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::vertex::Vertex;

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
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,

    width: f32,
    height: f32,
    iterations: u32,
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
            iterations: 1024,
            ranges: [0; 4],
            ranges_2: [0; 4],
            color: "ColorInferno".into(),
        }
    }

    fn reset(&mut self) {
        self.x_min = -2.0;
        self.x_max = 1.0;
        self.y_min = -1.0;
        self.y_max = 1.0;
    }

    fn scroll(&mut self, x: f64, y: f64) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min += x * s_x;
        self.x_max += x * s_x;
        self.y_min += y * s_y;
        self.y_max += y * s_y;
    }

    fn pan(&mut self, x: f64, y: f64) {
        self.scroll(x / 100.0, y / 100.0)
    }

    fn zoom_in(&mut self) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min += s_x;
        self.x_max -= s_x;
        self.y_min += s_y;
        self.y_max -= s_y;
    }

    fn zoom_out(&mut self) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min -= s_x;
        self.x_max += s_x;
        self.y_min -= s_y;
        self.y_max += s_y;
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
        f("iterations", UniformValue::UnsignedInt(self.iterations));
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
    let main_display = Display::new(wb, cb, &event_loop).unwrap();

    let wb = WindowBuilder::new().with_title("Parameters");
    let cb = ContextBuilder::new();
    let params_display = Display::new(wb, cb, &event_loop).unwrap();

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    let gl_params_window = params_display.gl_window();
    let params_window = gl_params_window.window();
    platform.attach_window(imgui.io_mut(), params_window, HiDpiMode::Default);
    drop(gl_params_window);

    let vertices: [Vertex; 6] = [
        [1.0, -1.0].into(),
        [-1.0, 1.0].into(),
        [-1.0, -1.0].into(),
        [1.0, 1.0].into(),
        [1.0, -1.0].into(),
        [-1.0, 1.0].into(),
    ];

    let vertex_buffer = VertexBuffer::new(&main_display, &vertices).unwrap();
    let indices = NoIndices(PrimitiveType::TrianglesList);

    let program = Program::from_source(
        &main_display,
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
        &main_display,
        glium::texture::UncompressedUintFormat::U32U32,
        glium::texture::MipmapsOption::NoMipmap,
        1024,
        768,
    )
    .unwrap();

    iteration_texture
        .as_surface()
        .clear_color(0.0, 0.0, 0.0, 0.0);

    let color_texture = Texture2d::empty_with_format(
        &main_display,
        glium::texture::UncompressedFloatFormat::F16F16F16F16,
        glium::texture::MipmapsOption::NoMipmap,
        1024,
        768,
    )
    .unwrap();

    let mut tenants = DataBuilder {
        dt: Dt {
            color_texture,
            iteration_texture,
        },
        buffs_builder: |dt| {
            let output = [
                ("color", dt.color_texture.to_color_attachment()),
                ("depth", dt.iteration_texture.to_color_attachment()),
            ];
            let framebuffer = MultiOutputFrameBuffer::new(&main_display, output).unwrap();
            (framebuffer, dt)
        },
    }
    .build();

    let dim = main_display.get_framebuffer_dimensions();
    eprintln!("{:?}", dim);
    let mut draw_params = DrawParams::new(main_display.get_framebuffer_dimensions());

    // Input variables.
    let mut mouse_down = false;
    let mut mouse_last = (0f64, 0f64);

    let mut renderer =
        Renderer::init(&mut imgui, &params_display).expect("Failed to initialize renderer");
    let mut last_frame = Instant::now();

    let mut a = 1f32;

    event_loop.run(move |ev, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match &ev {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_params_window = params_display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_params_window.window())
                    .expect("Failed to prepare frame");
                gl_params_window.window().request_redraw();
            }
            Event::RedrawRequested(window_id) => {
                if *window_id == main_display.gl_window().window().id() {
                    tenants.with_mut(|fields| {
                        let framebuffer = &mut fields.buffs.0;
                        let dt = fields.dt;

                        framebuffer
                            .draw(
                                &vertex_buffer,
                                &indices,
                                &program,
                                &draw_params,
                                &Default::default(),
                            )
                            .unwrap();

                        main_display.assert_no_error(None);

                        let p: Vec<Vec<(u32, u32)>> =
                            unsafe { dt.iteration_texture.unchecked_read() };

                        let mut p: Vec<_> = p
                            .into_iter()
                            .flatten()
                            .filter(|b| b.1 != 1)
                            .map(|b| b.0)
                            .collect();
                        p.sort_unstable();

                        draw_params.ranges = [
                            p.first().copied().unwrap_or_default(),
                            p.get((p.len() / 7).saturating_sub(1))
                                .copied()
                                .unwrap_or_default(),
                            p.get((p.len() * 2 / 7).saturating_sub(1))
                                .copied()
                                .unwrap_or_default(),
                            p.get((p.len() * 3 / 7).saturating_sub(1))
                                .copied()
                                .unwrap_or_default(),
                        ];

                        draw_params.ranges_2 = [
                            p.get((p.len() * 4 / 7).saturating_sub(1))
                                .copied()
                                .unwrap_or_default(),
                            p.get((p.len() * 5 / 7).saturating_sub(1))
                                .copied()
                                .unwrap_or_default(),
                            p.get((p.len() * 6 / 7).saturating_sub(1))
                                .copied()
                                .unwrap_or_default(),
                            p.last().copied().unwrap_or_default(),
                        ];

                        framebuffer
                            .draw(
                                &vertex_buffer,
                                &indices,
                                &program,
                                &draw_params,
                                &Default::default(),
                            )
                            .unwrap();

                        eprintln!("{:?} {:?}", draw_params.ranges, draw_params.ranges_2);

                        let mut target = main_display.draw();
                        target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);

                        if cfg!(windows) {
                            // Blit the pixels to the surface
                            dt.color_texture
                                .as_surface()
                                .fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);
                        } else {
                            // TODO: at least on Ubuntu on VMware, blitting doesn't seem to be supported
                            // Workaround for Linux: re-execute the shader, this time targeting the surface
                            target
                                .draw(
                                    &vertex_buffer,
                                    &indices,
                                    &program,
                                    &draw_params,
                                    &Default::default(),
                                )
                                .unwrap();
                        }

                        target.finish().expect("Failed to swap buffers");
                    });
                } else {
                    let mut params_target = params_display.draw();
                    params_target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);

                    let ui = imgui.frame();

                    ui.window("Controls")
                        .size([300.0, 150.0], Condition::FirstUseEver)
                        .position([600.0, 50.0], Condition::FirstUseEver)
                        .build(|| {
                            ui.slider("pan_hor", -2.0, 2.0, &mut a);
                        });

                    let gl_params_window = params_display.gl_window();

                    platform.prepare_render(ui, gl_params_window.window());
                    let draw_data = imgui.render();

                    renderer
                        .render(&mut params_target, draw_data)
                        .expect("Rendering failed");

                    params_target.finish().expect("Failed to swap buffers");
                }
            }
            outer @ Event::WindowEvent { window_id, .. }
                if *window_id == params_display.gl_window().window().id() =>
            {
                let gl_window = params_display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), outer);
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput {
                    state,
                    button: MouseButton::Left,
                    ..
                } => {
                    mouse_down = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    main_display.gl_window().window().request_redraw();
                    if mouse_down {
                        draw_params.pan(mouse_last.0 - position.x, position.y - mouse_last.1);
                    }

                    mouse_last = (position.x, position.y);

                    if !mouse_down {}
                }
                WindowEvent::MouseWheel {
                    phase: TouchPhase::Moved,
                    delta: MouseScrollDelta::LineDelta(_x, y),
                    ..
                } => {
                    main_display.gl_window().window().request_redraw();
                    if *y < 0.0 {
                        draw_params.zoom_out()
                    } else {
                        draw_params.zoom_in()
                    }
                }
                WindowEvent::KeyboardInput { input, .. }
                    if input.state == ElementState::Pressed =>
                {
                    if let Some(keycode) = input.virtual_keycode {
                        match keycode {
                            VirtualKeyCode::Minus => draw_params.zoom_out(),
                            VirtualKeyCode::Equals => draw_params.zoom_in(),
                            VirtualKeyCode::Space => draw_params.reset(),
                            VirtualKeyCode::Up => draw_params.scroll(0.0, -1.0),
                            VirtualKeyCode::Left => draw_params.scroll(-1.0, 0.0),
                            VirtualKeyCode::Right => draw_params.scroll(1.0, 0.0),
                            VirtualKeyCode::Down => draw_params.scroll(0.0, 1.0),
                            _ => return,
                        }

                        main_display.gl_window().window().request_redraw();
                    }
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => (),
        }
    });
}
