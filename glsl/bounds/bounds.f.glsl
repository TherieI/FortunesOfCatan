#version 450 core

out vec4 color;

uniform uvec2 u_resolution;  // Screen resolution
uniform float u_time;       // Time for animation

void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0); // Red
}