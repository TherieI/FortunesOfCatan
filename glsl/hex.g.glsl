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

void main() {
    // Hex metadata will remain consitant throughout the whole primitive
    hex_tex_id = gs_in[0].hex_tex;
    hex_value = gs_in[0].hex_val;
    // Position derived in vertex shader
    vec4 center = vec4(gs_in[0].g_pos, 0.0, 1.0);

    // Create vertices for each 6 points of the hexagon
    float hex_radius = 2.7;
    for (int i = 0; i < 7; i++) {
        float theta = 2.0 * PI * i / 6.0 + PI / 2.0;
        vec2 pos = vec2(hex_radius * cos(theta), hex_radius * sin(theta));
        gl_Position = u_mvp * (center + vec4(pos, 0.0, 0.0));
        f_tex_coords = vec2(pos.x / 6.0 + 0.5, pos.y / 6.0 + 0.5);
        EmitVertex();
        // Create a vertex for the center of the hexagon to wrap up the last triangle
        gl_Position = u_mvp * center;
        f_tex_coords = vec2(0.5);
        EmitVertex();
    }
    EndPrimitive();
}