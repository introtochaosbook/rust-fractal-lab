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