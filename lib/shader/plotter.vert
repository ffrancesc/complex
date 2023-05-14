#version 300 es
 
in vec2 position;
out vec2 st;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    st = position;
}