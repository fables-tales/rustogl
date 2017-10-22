extern crate gl;
extern crate sdl2;
extern crate gl_generator;

use gl::types::*;

use std::mem;
use std::str;
use std::slice;

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
const FLOATS_FOR_POSITION: usize = 2;
const FLOATS_FOR_COLOR: usize = 4;
const ENTIRE_VERTEX_STRIDE: usize = FLOATS_FOR_POSITION + FLOATS_FOR_COLOR;

fn main() {
    let mut state = State::new();
    // Vertex data
    let mut vertex_data = [
        Vertex::new(0.0, 0.5, 1.0, 1.0, 1.0, 1.0),
        Vertex::new(0.5, -0.5, 1.0, 1.0, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, 1.0, 1.0, 1.0, 1.0),
    ];

    let mut p = program::Program::new("hello triangle".into(), 800, 600).unwrap();
    blend::setup_blending();

    let vs = shader::compile_vertex_shader(VS_SRC).unwrap();
    let fs = shader::compile_fragment_shader(FS_SRC).unwrap();
    let program = shader::link_shader_program(vs, fs).unwrap();

    shader::use_shader_program(program, "out_color".into());

    let vao = vertex_collections::make_vertex_array();
    let vbo = vertex_collections::make_vertex_buffer();

    vertex_collections::bind_array_and_vertex_buffer(vao, vbo);

    shader::program_bind_attribute(program, "position".into(), FLOATS_FOR_POSITION, ENTIRE_VERTEX_STRIDE, 0);
    shader::program_bind_attribute(program, "color".into(), FLOATS_FOR_COLOR, ENTIRE_VERTEX_STRIDE, FLOATS_FOR_POSITION);

    let mut i = 0.0;
    let mut going_up = true;
    while p.is_alive() {
        p.check_exit_events();

        state.update();

        clear::clear_screen(0.0, 0.0, 1.0, 1.0);

        send_and_draw_buffer(state.to_ogl_buffer(), gl::TRIANGLES, ENTIRE_VERTEX_STRIDE);
        p.window.gl_swap_window();
    }

    // Cleanup
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

fn update(i: f32, going_up: bool) -> (f32, bool) {
    let mut i = i;
    let mut going_up = going_up;
    if going_up {
        i = i + 0.1;
    } else {
        i = i - 0.01;
    }
    if i >= 1.0 {
        going_up = false
    }

    if i <= 0.0 {
        going_up = true;
    }

    (i, going_up)
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
