#version 450 core

in flat uint hex_tex_id;
in flat uint hex_value;
in vec2 f_tex_coords;

out vec4 color;

uniform sampler2D texture_map_hex;
uniform sampler2D texture_map_chances;

uniform uvec2 u_resolution;  // Screen resolution
uniform float u_time;       // Time for animation

vec4 layer(vec4 foreground, vec4 background) {
    return foreground * foreground.a + background * (1.0 - foreground.a);
}

void main() {
    // hex_tex starts at DESERT=1 but the position in the texture is 0
    vec4 hex_texture = texture(texture_map_hex, vec2((f_tex_coords.x + hex_tex_id - 1) / 6, f_tex_coords.y));
    if (hex_tex_id > 1) {
        // Account for 7's not being a roll
        uint offset = 0;
        if (hex_value > 7) {
            offset += 1;
        }
        vec4 chances_texture = texture(texture_map_chances, vec2((f_tex_coords.x + hex_value - 2 - offset) / 10, f_tex_coords.y));
        color = layer(chances_texture, hex_texture);
    } else {
        color = hex_texture;
    }
}