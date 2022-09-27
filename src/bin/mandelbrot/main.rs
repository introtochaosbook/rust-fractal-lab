// Initial code based on https://github.com/remexre/mandelbrot-rust-gl

use glium::{Display, Program, Surface, Texture2d, VertexBuffer};
use glium::framebuffer::{MultiOutputFrameBuffer, ToColorAttachment};
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::{ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::program::ShaderStage;
use glium::texture::UnsignedTexture2d;
use glium::uniforms::{Uniforms, UniformValue};

use ouroboros::self_referencing;
use rust_fractal_lab::shader_builder::build_shader;
use rust_fractal_lab::vertex::Vertex;

use crate::ControlFlow::Wait;

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
    max_colors: u32,
    ranges: [u32; 4],
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
            max_colors: 256,
            ranges: [0; 4],
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
        self.scroll(x / 100.0,
                    y / 100.0)
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
        f("maxColors", UniformValue::UnsignedInt(self.max_colors));
        f("ranges", UniformValue::UnsignedIntVec4(self.ranges));
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

    let iteration_texture = UnsignedTexture2d::empty_with_format(
        &display,
        glium::texture::UncompressedUintFormat::U32U32,
        glium::texture::MipmapsOption::NoMipmap,
        1024,
        768,
    )
    .unwrap();

    iteration_texture
        .as_surface()
        .clear_color(0.0, 0.0, 0.0, 0.0);

    let color_texture = Texture2d::empty(
        &display, 1024, 768
    ).unwrap();

    let mut tenants = DataBuilder {
        dt: Dt {
            color_texture,
            iteration_texture,
        },
        buffs_builder: |dt| {
            let output =  [("color", dt.color_texture.to_color_attachment()), ("depth", dt.iteration_texture.to_color_attachment())];
            let framebuffer = MultiOutputFrameBuffer::new(&display, output).unwrap();
            (framebuffer, dt)
        }
    }.build();

    let dim = display.get_framebuffer_dimensions();
    eprintln!("{:?}", dim);
    let mut draw_params = DrawParams::new(display.get_framebuffer_dimensions());

    // Input variables.
    let mut mouse_down = false;
    let mut mouse_last = (0f64, 0f64);

    event_loop.run(move |ev, _, control_flow| {
        tenants.with_mut(|fields| {
            *control_flow = Wait;

            match ev {
                Event::RedrawRequested(_) => {
                    let framebuffer = &mut fields.buffs.0;
                    let dt = fields.dt;

                    framebuffer.draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &draw_params,
                        &Default::default(),
                    )
                        .unwrap();

                    display.assert_no_error(None);

                    let p: Vec<Vec<(u32, u32)>> = unsafe { dt.iteration_texture.unchecked_read() };

                    let mut p: Vec<_> = p
                        .into_iter()
                        .flatten()
                        .filter(|b| b.1 != 1)
                        .map(|b| b.0)
                        .collect();
                    p.sort_unstable();

                    draw_params.ranges = [
                        p[0],
                        p[p.len() * 3 / 4 - 1],
                        p[p.len() * 7 / 8 - 1],
                        *p.last().unwrap(),
                    ];

                    eprintln!("{:?}", draw_params.ranges);

                    let target = display.draw();
                    dt.color_texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);
                    target.finish().unwrap();
                }
                Event::WindowEvent {
                    event, ..
                } => {
                    match event {
                        WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => mouse_down = match state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        },
                        WindowEvent::CursorMoved { position, .. } => {
                            if mouse_down {
                                draw_params.pan(mouse_last.0 - position.x, mouse_last.1 - position.y);
                            }
                            mouse_last = (position.x, position.y);
                        },
                        WindowEvent::MouseWheel { phase: TouchPhase::Moved, delta: MouseScrollDelta::LineDelta(_x, y), .. } => {
                            if y < 0.0 {
                                draw_params.zoom_out()
                            } else {
                                draw_params.zoom_in()
                            }
                        }
                        WindowEvent::KeyboardInput { input, .. } if input.state == ElementState::Pressed => {
                            if let Some(keycode) = input.virtual_keycode {
                                match keycode {
                                    VirtualKeyCode::Minus => draw_params.zoom_out(),
                                    VirtualKeyCode::Equals => draw_params.zoom_in(),
                                    VirtualKeyCode::Space => draw_params.reset(),
                                    VirtualKeyCode::Up => draw_params.scroll(0.0, -1.0),
                                    VirtualKeyCode::Left => draw_params.scroll(-1.0, 0.0),
                                    VirtualKeyCode::Right => draw_params.scroll(1.0, 0.0),
                                    VirtualKeyCode::Down => draw_params.scroll(0.0, 1.0),
                                    _ => {}
                                }
                            }
                        },
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                        _ => { }
                    }
                }
                _ => (),
            }
        });
    });
}
