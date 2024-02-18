#version 450 core

in flat uint hex_tex;
in flat uint hex_value;
in vec2 f_tex_coords;

out vec4 color;

uniform sampler2D texture_map;

uniform uvec2 u_resolution;  // Screen resolution
uniform float u_time;       // Time for animation

void main() {
    vec2 position = gl_FragCoord.xy / u_resolution - u_resolution / 2;
    
    float dist = length(position);
    // Adjust the ripple effect based on time
    float ripple = sin(dist * 10.0 - u_time * 2.0) * 0.1;

    // hex_tex starts at DESERT=1 but the position in the texture is 0
    color = texture(texture_map, vec2((f_tex_coords.x + hex_tex - 1) / 6, f_tex_coords.y));
}