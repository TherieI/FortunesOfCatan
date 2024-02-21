#version 450 core

in vec2 pos;
in uint hex_meta;

out VS_OUT {
    vec2 g_pos;
    flat uint hex_tex;
    flat uint hex_val;
} vs_out;

uint hex_texture_id(in uint meta) {
    // First 8 bits of meta
    return meta & 255;
}

uint hex_value(in uint meta) {
    // Second 8 bits of meta
    return (meta >> 8) & 255;
}

void main() {
    vs_out.hex_tex = hex_texture_id(hex_meta);
    vs_out.hex_val = hex_value(hex_meta);
    vs_out.g_pos = pos;
    // Determine position in geometry shader
    gl_Position = vec4(0.0);
}