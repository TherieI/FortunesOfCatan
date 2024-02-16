#version 450 core

in flat uint hex_tex;
in flat uint hex_value;
in vec2 f_tex_coords;

out vec4 color;

uniform sampler2D texture_map;

void main() {
    if (hex_tex == 2) {
        color = texture(texture_map, f_tex_coords);
    } else {
        color = vec4(0.96, 0.87, 0.70, 1.0);
    }
    
}