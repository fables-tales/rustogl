use gl;

pub fn setup_blending() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
        gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ZERO);
    }
}
