#version 450 core

in flat uint hex_tex_id;
in flat uint hex_value;
in vec2 f_tex_coords;

out vec4 color;

uniform sampler2D texture_map;

uniform uvec2 u_resolution;  // Screen resolution
uniform float u_time;       // Time for animation

void main() {
    // hex_tex starts at DESERT=1 but the position in the texture is 0
    color = texture(texture_map, vec2((f_tex_coords.x + hex_tex_id - 1) / 6, f_tex_coords.y));
}