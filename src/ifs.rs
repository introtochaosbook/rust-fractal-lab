use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{NoIndices, PrimitiveType};

use glium::glutin::event_loop::ControlFlow::Wait;
use glium::uniforms::{UniformValue, Uniforms};
use glium::{implement_vertex, Display, Program, Surface, VertexBuffer};

use ndarray::{s, Array, Ix2};
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;

#[derive(Copy, Clone)]
struct ColoredVertex {
    position: [f32; 2],
    color: [f32; 4],
}

implement_vertex!(ColoredVertex, position, color);

#[derive(Copy, Clone, Default, Debug)]
struct MapParams {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

#[derive(Default)]
pub struct IfsProgram {
    uniforms: MapParams,
    vertices: Vec<ColoredVertex>,
}

impl IfsProgram {
    pub fn sample_affine(
        &mut self,
        d: &Array<f32, Ix2>,
        color: [f32; 4],
        iters: usize,
        scale: f32,
        shift_x: f32,
        shift_y: f32,
    ) {
        let probs: Vec<f32> = d.slice(s![.., -1]).to_vec();
        let dist = WeightedIndex::new(probs).unwrap();
        let mut rng = rand::thread_rng();

        // Initial starting point
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        for i in 0..iters {
            let r = d.row(dist.sample(&mut rng));
            x = r[0] * x + r[1] * y + r[4];
            y = r[2] * x + r[3] * y + r[5];

            let scaled_x = (x + shift_x) * scale;
            let scaled_y = (y + shift_y) * scale;

            self.uniforms.x_min = self.uniforms.x_min.min(scaled_x);
            self.uniforms.x_max = self.uniforms.x_max.max(scaled_x);
            self.uniforms.y_min = self.uniforms.y_min.min(scaled_y);
            self.uniforms.y_max = self.uniforms.y_max.max(scaled_y);

            if i >= 10 {
                // Skip first few iterations
                self.vertices.push(ColoredVertex {
                    position: [scaled_x, scaled_y],
                    color,
                })
            }
        }
    }

    pub fn sample(&mut self, d: &Array<f32, Ix2>, color: [f32; 4], iters: usize) {
        self.sample_affine(d, color, iters, 1.0, 0.0, 0.0);
    }

    pub fn run(&self) {
        let event_loop = EventLoop::new();

        let wb = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(768.0_f32, 768.0_f32))
            .with_title("Hello world");

        let cb = ContextBuilder::new();

        let display = Display::new(wb, cb, &event_loop).unwrap();

        let vertex_buffer = VertexBuffer::new(&display, &self.vertices).unwrap();
        let indices = NoIndices(PrimitiveType::Points);

        let program = Program::from_source(
            &display,
            r##"#version 140
uniform float x_min;
uniform float x_max;
uniform float y_min;
uniform float y_max;

in vec4 color;
out vec4 v_color;

float map(float x, float in_min, float in_max, float out_min, float out_max) {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

in vec2 position;
void main() {
	gl_Position = vec4(map(position.x, x_min, x_max, -1.0, 1.0), map(position.y, y_min, y_max, -1.0, 1.0), 0.0, 1.0);
	v_color = color;
}
"##,
            r##"#version 130
in vec4 v_color;
out vec4 color;
void main() {
	color = v_color;
}
"##,
            None,
        )
            .unwrap();

        let uniforms = self.uniforms;
        event_loop.run(move |ev, _, control_flow| {
            *control_flow = Wait;

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                _ => (),
            }

            let mut target = display.draw();
            target.clear_color(255.0, 255.0, 255.0, 1.0);
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
}

impl Uniforms for MapParams {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("x_max", UniformValue::Float(self.x_max));
        f("y_min", UniformValue::Float(self.y_min));
        f("x_min", UniformValue::Float(self.x_min));
        f("y_max", UniformValue::Float(self.y_max));
    }
}
