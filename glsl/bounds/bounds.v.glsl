#version 450 core

in vec2 pos;
in vec2 size;

out VS_OUT {
    vec2 g_pos;
    vec2 g_size;
} vs_out;

void main() {
    vs_out.g_pos = pos;
    vs_out.g_size = size;
    // Determine position in geometry shader
    gl_Position = vec4(0.0);
}