#version 450 core

in vec2 pos;
in vec2 tex_coords;
in uint hex_meta;

out flat uint hex_tex;
out flat uint hex_value;
out vec2 f_tex_coords;

uniform mat4 u_mvp;

uint get_hex_tex(in uint meta) {
    // First 8 bits of meta
    return meta & 255;
}

uint get_hex_value(in uint meta) {
    // Second 8 bits of meta
    return (meta >> 8) & 255;
}

void main() {
    hex_tex = get_hex_tex(hex_meta);
    hex_value = get_hex_value(hex_meta);
    f_tex_coords = tex_coords;

    gl_Position = u_mvp * vec4(pos, 0.0, 1.0);
}