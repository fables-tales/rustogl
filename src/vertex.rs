use gl::types::GLfloat;
use std::mem;

#[repr(C, packed)]
pub struct Position {
    pub x: GLfloat,
    pub y: GLfloat,
}

#[repr(C, packed)]
pub struct Color {
    pub r: GLfloat,
    pub g: GLfloat,
    pub b: GLfloat,
    pub a: GLfloat,
}

#[repr(C, packed)]
pub struct Vertex {
    pub position: Position,
    pub color: Color
}

impl Vertex {
//    pub fn byte_size_of_vertex() -> usize {
//        mem::size_of::<Self>()
//    }

    pub fn float_size_of_vertex() -> usize {
        mem::size_of::<Self>() / mem::size_of::<GLfloat>()
    }

    pub fn float_size_of_position() -> usize {
        mem::size_of::<Position>() / mem::size_of::<GLfloat>()
    }

    pub fn float_size_of_color() -> usize {
        mem::size_of::<Color>() / mem::size_of::<GLfloat>()
    }

    pub fn float_offset_of_position() -> u32 {
        0
    }

    pub fn float_offset_of_color() -> u32 {
        Self::float_size_of_position() as u32
    }

    pub fn new(x: GLfloat, y: GLfloat, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) -> Self {
        Vertex {
            position: Position {
                x: x,
                y: y,
            },
            color: Color {
                r: r,
                g: g,
                b: b,
                a: a,
            }
        }
    }
}
