extern crate gl;
extern crate sdl2;
extern crate gl_generator;

use gl::types::*;

use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod shader;
mod blend;
mod program;

// Shader sources
static VS_SRC: &'static str = r##"#version 100
        // Our inputs (the fields from our `Vertex` struct)
        attribute vec2 position;
        attribute vec4 color;
        // Our output (the color for our fragment shader)
        varying vec4 _color;
        void main() {
            gl_Position = vec4(position, -1.0, 1.0);
            _color = color;
        }
    "##;

static FS_SRC: &'static str = r##"#version 100
        // Our input (the color copied from our vertex shader)
        varying highp vec4 _color;
        void main() {
            gl_FragColor = _color;
        }
    "##;

const FLOATS_FOR_POSITION: usize = 2;
const FLOATS_FOR_COLOR: usize = 4;
const ENTIRE_VERTEX_STRIDE: usize = FLOATS_FOR_POSITION + FLOATS_FOR_COLOR;

fn main() {
    // Vertex data
    let mut vertex_data: [GLfloat; ENTIRE_VERTEX_STRIDE * 3] = [
        0.0,
        0.5,
        1.0,
        1.0,
        1.0,
        1.0,
        0.5,
        -0.5,
        1.0,
        1.0,
        1.0,
        0.0,
        -0.5,
        -0.5,
        1.0,
        1.0,
        1.0,
        1.0,
    ];

    let mut p = program::Program::new("hello triangle".into(), 800, 600).unwrap();

    let vs = shader::compile_vertex_shader(VS_SRC).unwrap();
    let fs = shader::compile_fragment_shader(FS_SRC).unwrap();
    let program = shader::link_shader_program(vs, fs).unwrap();

    shader::use_shader_program(program, "out_color".into());

    blend::setup_blending();

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            FLOATS_FOR_POSITION as i32,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            ENTIRE_VERTEX_STRIDE as i32 * mem::size_of::<GLfloat>() as i32,
            ptr::null(),
        );

        let color_attr = gl::GetAttribLocation(program, CString::new("color").unwrap().as_ptr());
        gl::EnableVertexAttribArray(color_attr as GLuint);
        gl::VertexAttribPointer(
            color_attr as GLuint,
            FLOATS_FOR_COLOR as i32,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            ENTIRE_VERTEX_STRIDE as i32 * mem::size_of::<GLfloat>() as i32,
            (FLOATS_FOR_POSITION as i32 * (mem::size_of::<GLfloat>() as i32)) as *const GLvoid,
        );
    }

    let mut done = false;
    let mut i = 0.0;
    let mut going_up = true;
    while !done {
        for event in p.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    done = true;
                }
                _ => {}
            }
        }

        vertex_data[0] = i;
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

        unsafe {

            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

        }

        send_and_draw_buffer(&vertex_data, gl::TRIANGLES, ENTIRE_VERTEX_STRIDE);

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
