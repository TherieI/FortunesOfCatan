#version 450 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in VS_OUT {
    vec2 g_pos;
    flat uint building_id;
    flat uint building_color_id;
} gs_in[];  // There is only one vertex in points so we will only ever be dealing with gs_in[0]

uniform mat4 u_mvp;

out vec2 f_tex_coords;
out flat uint b_id;
out flat uint b_color_id;

// A constant to reduce the radius of the texture coordinates
const float BUILDING_FIT = 1.3;

void main() {
    // Hex metadata will remain consistant throughout the whole primitive
    b_id = gs_in[0].building_id;
    b_color_id = gs_in[0].building_color_id;
    vec4 center = vec4(gs_in[0].g_pos, 0.0, 1.0);

    // Length and width of the structure
    vec2 dimensions = vec2(1.5);
    // Top left
    vec4 position = center + vec4(-dimensions.x, dimensions.y, 0, 0);
    gl_Position = u_mvp * position;
    f_tex_coords = vec2(0, 1);
    EmitVertex();
    // Top Right
    position = center + vec4(dimensions.x, dimensions.y, 0, 0);
    gl_Position = u_mvp * position;
    f_tex_coords = vec2(1, 1);
    EmitVertex();
    // Bottom left
    position = center + vec4(-dimensions.x, -dimensions.y, 0, 0);
    gl_Position = u_mvp * position;
    f_tex_coords = vec2(0, 0);
    EmitVertex();
    // Bottom right
    position = center + vec4(dimensions.x, -dimensions.y, 0, 0);
    gl_Position = u_mvp * position;
    f_tex_coords = vec2(1, 0);
    EmitVertex();

    EndPrimitive();
}