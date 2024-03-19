#version 450 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
    vec2 g_pos;
    vec2 g_size;
} gs_in[];  // There is only one vertex in points so we will only ever be dealing with gs_in[0]

uniform mat4 u_mvp;

void main() {
    // Hex metadata will remain consistant throughout the whole primitive
    vec4 bottom_left = vec4(gs_in[0].g_pos, 0.0, 1.0);
    vec2 size = gs_in[0].g_size;

    // Top left
    vec4 position = bottom_left + vec4(0, size.y, 0, 0);
    gl_Position = u_mvp * position;
    EmitVertex();
    // Top Right
    position = bottom_left + vec4(size.x, size.y, 0, 0);
    gl_Position = u_mvp * position;
    EmitVertex();
    // Bottom left
    position = bottom_left + vec4(0, 0, 0, 0);
    gl_Position = u_mvp * position;
    EmitVertex();
    // Bottom right
    position = bottom_left + vec4(size.x, 0, 0, 0);
    gl_Position = u_mvp * position;
    EmitVertex();

    EndPrimitive();
}