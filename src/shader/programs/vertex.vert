#version 100
// Our inputs (the fields from our `Vertex` struct)
attribute vec2 position;
attribute vec4 color;
// Our output (the color for our fragment shader)
varying vec4 _color;
void main() {
    gl_Position = vec4(position, -1.0, 1.0);
    _color = color;
}
