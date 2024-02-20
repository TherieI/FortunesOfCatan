#version 450 core

layout (points) in;
layout (trangles, max_vertices = 7) out;

void main() {
    // vec4 point = gl_in[0].gl_Position;

    // // Define the vertices of a square around the input point
    // vec4 v0 = point + vec4(-0.5, -0.5, 0.0, 0.0);
    // vec4 v1 = point + vec4( 0.5, -0.5, 0.0, 0.0);
    // vec4 v2 = point + vec4(-0.5,  0.5, 0.0, 0.0);
    // vec4 v3 = point + vec4( 0.5,  0.5, 0.0, 0.0);

    // // Emit the first triangle
    // gl_Position = v0;
    // EmitVertex();
    // gl_Position = v1;
    // EmitVertex();
    // gl_Position = v2;
    // EmitVertex();
    // EndPrimitive();

    // // Emit the second triangle
    // gl_Position = v1;
    // EmitVertex();
    // gl_Position = v2;
    // EmitVertex();
    // gl_Position = v3;
    // EmitVertex();
    // EndPrimitive();
}