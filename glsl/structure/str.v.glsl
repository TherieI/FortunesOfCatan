#version 450 core

in vec2 pos;
in uint meta;

out VS_OUT {
    vec2 g_pos;
    flat uint building_id;
    flat uint building_color_id;
} vs_out;

uint building_id(in uint meta) {
    // First 4 bits of meta
    return meta & 15;
}

uint building_color_id(in uint meta) {
    // Next 8 bits of meta
    return (meta >> 4) & 255;
}

void main() {
    vs_out.building_id = building_id(meta);
    vs_out.building_color_id = building_color_id(meta);
    vs_out.g_pos = pos;
    // Determine position in geometry shader
    gl_Position = vec4(0.0);
}