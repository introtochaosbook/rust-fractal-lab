use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn x(&self) -> f32 {
        self.position[0]
    }

    pub fn y(&self) -> f32 {
        self.position[1]
    }
}

impl From<[f32; 2]> for Vertex {
    fn from(x: [f32; 2]) -> Self {
        Vertex { position: x }
    }
}
