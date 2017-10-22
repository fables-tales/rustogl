#version 100
// Our input (the color copied from our vertex shader)
varying highp vec4 _color;
void main() {
    gl_FragColor = _color;
}
