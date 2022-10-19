use glium::glutin::dpi::{PhysicalPosition, Size};
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;

pub trait WindowBuilderHelpers<T: 'static>: Sized {
    fn with_inner_size_centered<S: Into<Size>>(self, size: S, event_loop: &EventLoop<T>) -> Self;
}

impl<T: 'static> WindowBuilderHelpers<T> for WindowBuilder {
    fn with_inner_size_centered<S: Into<Size>>(self, size: S, event_loop: &EventLoop<T>) -> Self {
        let size = size.into();
        let mut ret = self.with_inner_size(size);

        let physical_size = match size {
            Size::Physical(physical_size) => physical_size,
            Size::Logical(_) => unimplemented!(),
        };

        if let Some(monitor_size) = event_loop.primary_monitor().map(|m| m.size()) {
            ret = ret.with_position(PhysicalPosition::new(
                (monitor_size.width - physical_size.width) / 2,
                (monitor_size.height - physical_size.height) / 2,
            ));
        }

        ret
    }
}
