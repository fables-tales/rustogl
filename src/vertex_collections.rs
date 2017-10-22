use gl;
use gl::types::GLuint;

type VertexArray = GLuint;
type VertexBuffer = GLuint;

pub struct VertexArrayBufferPair {
    vao: VertexArray,
    vbo: VertexBuffer,
}

impl VertexArrayBufferPair {
    pub fn new() -> Self {
        VertexArrayBufferPair {
            vao: make_vertex_array(),
            vbo: make_vertex_buffer(),
        }
    }

    pub fn with<F>(&self, callable: F) -> Result<(), String> where F: FnOnce() -> Result<(), String> {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }

        callable()
    }
}

impl Drop for VertexArrayBufferPair {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

fn make_vertex_array() -> VertexArray {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        vao
    }
}

fn make_vertex_buffer() -> VertexBuffer {
    unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        vbo
    }
}
