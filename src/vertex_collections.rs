use gl;
use gl::types::GLuint;

pub type VertexArray = GLuint;
pub type VertexBuffer = GLuint;

pub fn make_vertex_array() -> VertexArray {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        vao
    }
}

pub fn make_vertex_buffer() -> VertexBuffer {
    unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        vbo
    }
}

pub fn bind_array_and_vertex_buffer(va: VertexArray, vb: VertexBuffer) {
    unsafe {
        gl::BindVertexArray(va);
        gl::BindBuffer(gl::ARRAY_BUFFER, vb);
    }
}
