use gl;
use vertex::Vertex;
use std::slice;

pub struct State {
    vertices: Vec<Vertex>,
    i: f32,
    direction: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            vertices: vec!(
                Vertex::new(0.0, 0.5, 1.0, 1.0, 1.0, 1.0),
                Vertex::new(0.5, -0.5, 1.0, 1.0, 1.0, 0.0),
                Vertex::new(-0.5, -0.5, 1.0, 1.0, 1.0, 1.0),
            ),
            i: 0.0,
            direction: true,
        }
    }
    pub fn update(&mut self) {
        if self.direction {
            self.i += 0.1
        } else {
            self.i -= 0.1
        }

        if self.i > 1.0 {
            self.direction = false
        }

        if self.i < 0.0 {
            self.direction = true;
        }
        self.vertices[0].position.x = self.i
    }

    pub fn to_ogl_buffer<'a>(&self) -> &'a[gl::types::GLfloat] {
        let fp = self.vertices.as_ptr() as *const gl::types::GLfloat;
        let buffer = unsafe { slice::from_raw_parts(fp, self.vertices.len() * Vertex::byte_size_of_vertex()) };
        buffer
    }
}
