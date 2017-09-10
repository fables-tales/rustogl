extern crate gl;
extern crate sdl2;
extern crate gl_generator;

use gl_generator::{Profile};

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video;


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

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader,
                                 len,
                                 ptr::null_mut(),
                                 buf.as_mut_ptr() as *mut GLchar);
            panic!("{}",
                   str::from_utf8(&buf)
                       .ok()
                       .expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program,
                                  len,
                                  ptr::null_mut(),
                                  buf.as_mut_ptr() as *mut GLchar);
            panic!("{}",
                   str::from_utf8(&buf)
                       .ok()
                       .expect("ProgramInfoLog not valid utf8"));
        }
        program
    }
}

fn main() {
    // Vertex data
    let mut vertex_data: [GLfloat; (2+4)*3] = [
        0.0, 0.5, 1.0, 1.0, 1.0, 1.0,
        0.5, -0.5, 1.0, 1.0, 1.0, 0.0,
        -0.5, -0.5, 1.0, 1.0, 1.0, 1.0,
    ];

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    // force vsync
    video.gl_set_swap_interval(1);

    let gl_attr = video.gl_attr();

    // Use OpenGL 4.1 core. Note that glitter is (currently) only designed
    // for OpenGL ES 2.0, but OpenGL 4.1 added the GL_ARB_ES2_compatibility
    // extension, which adds OpenGL ES 2 compatibility
    gl_attr.set_context_profile(video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    gl_attr.set_context_flags().debug().set();

    // Create our window (and make it usable with OpenGL)
    let window = video.window("Hello Triangle!", 800, 600)
                      .opengl()
                      .build()
                      .expect("Failed to create SDL window");


    let context = window.gl_create_context().unwrap();

    // Load the system's OpenGL library
    video.gl_load_library_default().expect("Failed to load OpenGL library");

    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::Enable(gl::BLEND);
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());

        let floats_for_vertex = 2;
        let floats_for_color = 4;

        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint,
                                floats_for_vertex,
                                gl::FLOAT,
                                gl::FALSE as GLboolean,
                                (floats_for_vertex+floats_for_color)*mem::size_of::<GLfloat>() as i32,
                                ptr::null());

        let color_attr = gl::GetAttribLocation(program, CString::new("color").unwrap().as_ptr());
        gl::EnableVertexAttribArray(color_attr as GLuint);
        gl::VertexAttribPointer(color_attr as GLuint,
                                floats_for_color,
                                gl::FLOAT,
                                gl::FALSE as GLboolean,
                                (floats_for_vertex+floats_for_color)*mem::size_of::<GLfloat>() as i32,
                                (floats_for_vertex*(mem::size_of::<GLfloat>() as i32)) as *const GLvoid);
        gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
        gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ZERO);
    }

    let mut done = false;
    let mut event_pump = sdl.event_pump().expect("Failed to get SDL events");
    let mut i = 0.0;
    let mut going_up = true;
    while !done {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        done = true;
                    },
                _ => { }
            }
        }
        unsafe {
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

            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BufferData(gl::ARRAY_BUFFER,
                       (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&vertex_data[0]),
                       gl::DYNAMIC_DRAW);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
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
