#version 450 core

layout (points) in;
layout (triangle_strip, max_vertices = 14) out;

#define PI 3.1415926535897932384626433832795

in VS_OUT {
    vec2 g_pos;
    flat uint hex_tex;
    flat uint hex_val;
} gs_in[];  // There is only one vertex in points so we will only ever be dealing with gs_in[0]

uniform mat4 u_mvp;

out vec2 f_tex_coords;
out flat uint hex_tex_id;
out flat uint hex_value;

// A constant to reduce the radius of the texture coordinates
const float HEX_FIT = 1.3;

void main() {
    // Hex metadata will remain consitant throughout the whole primitive
    hex_tex_id = gs_in[0].hex_tex;
    hex_value = gs_in[0].hex_val;
    vec4 center = vec4(gs_in[0].g_pos, 0.0, 1.0);

    // We reuse this vertex for every point of the hexagon
    vec4 center_transformed = u_mvp * center;

    // Create vertices for each 6 points of the hexagon
    float hex_radius = 2.7;
    for (int i = 0; i < 7; i++) {
        float theta = 2.0 * PI * i / 6.0 + PI / 2.0;
        // Position of hexagon point
        vec2 pos = vec2(hex_radius * cos(theta), hex_radius * sin(theta));
        // Generate vertex for hexagon point
        gl_Position = u_mvp * (center + vec4(pos, 0.0, 0.0));
        f_tex_coords = vec2(-HEX_FIT * pos.x / 6.0 + 0.5, HEX_FIT * pos.y / 6.0 + 0.5);
        EmitVertex();
        // Add a vertex for the center so triangle_strip wraps properly
        gl_Position = center_transformed;
        f_tex_coords = vec2(0.5);
        EmitVertex();
    }
    EndPrimitive();
}