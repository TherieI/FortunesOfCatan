#version 450 core

in vec2 pos;
in vec2 tex_coords;

uniform mat4 perspective;

void main() {
    gl_Position = perspective * vec4(pos, 0.0, 1.0);
}