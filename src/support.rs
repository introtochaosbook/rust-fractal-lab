use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

pub fn init(title: &str) -> System {


    System {
        event_loop,
        display,
        imgui,
        platform,
        renderer,
        font_size,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;

    }
}
