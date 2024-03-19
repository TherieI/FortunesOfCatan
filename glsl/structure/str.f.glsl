#version 450 core

in flat uint b_id;
in flat uint b_color_id;
in vec2 f_tex_coords;

out vec4 color;

uniform sampler2D texture_map_structures;

uniform uvec2 u_resolution;  // Screen resolution
uniform float u_time;       // Time for animation

void main() {
    if (b_id == 2) {
        // Settlement
        color = texture(texture_map_structures, f_tex_coords);
    }
}