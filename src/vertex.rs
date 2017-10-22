use gl::types::GLfloat;

#[repr(C, packed)]
pub struct Vertex {
    pub x: GLfloat,
    pub y: GLfloat,
    pub r: GLfloat,
    pub g: GLfloat,
    pub b: GLfloat,
    pub a: GLfloat,
}

impl Vertex {
    pub fn new(x: GLfloat, y: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) -> Self {
        Vertex {
            x: x,
            y: y,
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}
