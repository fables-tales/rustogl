use gl;

pub fn clear_screen(r: gl::types::GLfloat, g: gl::types::GLfloat, b: gl::types::GLfloat, a: gl::types::GLfloat) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
