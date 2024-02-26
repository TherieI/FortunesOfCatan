#version 450 core

// Hex positions of the board
uniform hex_positions {
    vec2 positions[64];
};
uniform uint total_hex;
uniform uvec2 u_resolution;
uniform float u_scale;

out vec4 color;

const float SCALE_FACTOR = 0.5;

void main() {
    vec2 st = gl_FragCoord.xy / u_resolution.xy; // Normalize screen coordinates

    // Initialize color to the default color
    color = vec4(st, 0.3, 1.0);

    vec2 ab = vec2(0.2);

    // Iterate over the hex positions
    for (uint i = 0u; i < total_hex; i++) {
        // Calculate the distance from the current fragment to the hex position

        vec2 from_origin = st - positions[i] * SCALE_FACTOR;

    
        // Check if the fragment is inside the circle
        if ((from_origin.x * from_origin.x) / (ab.x * ab.x) + (from_origin.y * from_origin.y) / (ab.y * ab.y) < 1 * u_scale) {
            // If inside and closer than previous, update the color to red
            color = vec4(positions[i], 0.0, 1.0);
        }
    }
}