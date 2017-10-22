#![deny(warnings)]
extern crate gl;
extern crate sdl2;
extern crate gl_generator;

use gl::types::*;

use std::mem;
use std::str;
use vertex::Color;

mod shader;
mod blend;
mod program;
mod clear;
mod vertex_collections;
mod vertex;
mod state;

use state::State;
use vertex::Vertex;

// Shader sources
static VS_SRC: &'static str = include_str!("shader/programs/vertex.vert");
static FS_SRC: &'static str = include_str!("shader/programs/fragment.frag");

fn main() {
    let mut state = State::new();
    let s2 = State::new();
    let mut p = program::Program::new("hello triangle".into(), 800, 600).unwrap();

    let bg_color = Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0};

    blend::setup_blending();
    let program = shader::ShaderProgram::new(VS_SRC, FS_SRC).unwrap();
    let vb = vertex_collections::VertexArrayBufferPair::new();

    let res = program.with("out_color".into(), || {
        vb.with(|| {
            bind_attributes(&program);
            while p.is_alive() {
                p.check_exit_events();

                state.update();

                clear::clear_screen(&bg_color);

                send_and_draw_buffer(
                    state.to_ogl_buffer(),
                    gl::TRIANGLES,
                    Vertex::float_size_of_vertex()
                );

                send_and_draw_buffer(
                    s2.to_ogl_buffer(),
                    gl::TRIANGLES,
                    Vertex::float_size_of_vertex()
                );


                p.window.gl_swap_window();
            }

            Ok(())
        })
    });

    res.unwrap();
}

fn bind_attributes(program: &shader::ShaderProgram) {
    program.bind_attribute(
        "position".into(),
        Vertex::float_size_of_position(),
        Vertex::float_size_of_vertex(),
        Vertex::float_offset_of_position() as usize,
    );

    program.bind_attribute(
        "color".into(),
        Vertex::float_size_of_color(),
        Vertex::float_size_of_vertex(),
        Vertex::float_offset_of_color() as usize,
    );
}

fn send_and_draw_buffer(buffer: &[GLfloat], shape_type: gl::types::GLenum, vertex_stride: usize) {
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&buffer[0]),
            gl::DYNAMIC_DRAW,
        );

        // Draw a triangle from the 3 vertices
        gl::DrawArrays(
            shape_type,
            0,
            (buffer.len() as i32) / (vertex_stride as i32),
        );
    }
}
