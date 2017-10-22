use gl;
use vertex::Vertex;
use std::slice;
use std::mem;

pub struct State {
    vertices: Vec<Vertex>,
}

impl State {
    pub fn new() -> Self {
        State {
            vertices: vec!(
                Vertex::new(0.0, 0.5, 1.0, 1.0, 1.0, 1.0),
                Vertex::new(0.5, -0.5, 1.0, 1.0, 1.0, 0.0),
                Vertex::new(-0.5, -0.5, 1.0, 1.0, 1.0, 1.0),
            )
        }
    }
    pub fn update(&self) {
    }

    pub fn to_ogl_buffer<'a>(&self) -> &'a[gl::types::GLfloat] {
        let fp = self.vertices.as_ptr() as *const gl::types::GLfloat;
        let buffer = unsafe { slice::from_raw_parts(fp, self.vertices.len() * mem::size_of::<Vertex>()) };
        buffer
    }
}
