use gl;

use std::ffi::CString;
use std::str;
use std::ptr;
use std::mem;

pub type VertexShader = gl::types::GLuint;
pub type FragmentShader = gl::types::GLuint;

pub struct ShaderProgram {
    vs: VertexShader,
    fs: FragmentShader,
    gl_pointer: gl::types::GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_pointer);
            gl::DeleteShader(self.fs);
            gl::DeleteShader(self.vs);
        }
    }
}

impl ShaderProgram {
    pub fn with<F>(&self, output_attribute_name: String, callable: F) -> Result<(), String>
    where F: FnOnce() -> Result<(), String> {
        unsafe {
            // Use shader program
            gl::UseProgram(self.gl_pointer);
            gl::BindFragDataLocation(
                self.gl_pointer,
                0,
                CString::new(output_attribute_name.bytes().collect::<Vec<u8>>())
                .unwrap()
                .as_ptr(),
            );

            let res = callable();
            gl::UseProgram(0);
            res
        }
    }


    pub fn bind_attribute(&self, name: String, count: usize, stride: usize, offset: usize) {
        unsafe {
            let attr = gl::GetAttribLocation(self.gl_pointer, CString::new(name.into_bytes()).unwrap().as_ptr());
            let err = gl::GetError();
            println!("err: {}", err);

            gl::EnableVertexAttribArray(attr as gl::types::GLuint);
            let err = gl::GetError();
            println!("err: {}", err);
            gl::VertexAttribPointer(
                attr as gl::types::GLuint,
                count as i32,
                gl::FLOAT,
                gl::FALSE as gl::types::GLboolean,
                ((stride)*mem::size_of::<gl::types::GLfloat>()) as i32,
                (offset*(mem::size_of::<gl::types::GLfloat>())) as *const gl::types::GLvoid
                );
            let err = gl::GetError();
            println!("err: {}", err);
        }
    }
}

pub fn compile_vertex_shader(src: &str) -> Result<VertexShader, String> {
    compile_shader(src, gl::VERTEX_SHADER).map(|r| r as VertexShader)
}

pub fn compile_fragment_shader(src: &str) -> Result<FragmentShader, String> {
    compile_shader(src, gl::FRAGMENT_SHADER).map(|r| r as FragmentShader)
}

pub fn link_shader_program(vs: VertexShader, fs: FragmentShader) -> Result<ShaderProgram, String> {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as gl::types::GLint) {
            let mut len: gl::types::GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut gl::types::GLchar,
            );
            let si = try!(str::from_utf8(&buf).map_err(
                |_| "ProgramInfoLog not valid utf8",
            ));
            return Err(format!("{}", si));
        }

        let complete = ShaderProgram {
            vs: vs,
            fs: fs,
            gl_pointer: program,
        };

        Ok(complete)
    }
}


fn compile_shader(src: &str, ty: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as gl::types::GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut gl::types::GLchar,
            );

            let si = try!(str::from_utf8(&buf).map_err(
                |_| "ShaderInfoLog not valid utf8",
            ));
            return Err(format!("{}", si));
        }

        Ok(shader)
    }
}
