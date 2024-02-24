#version 450 core

// Hex positions of the board
uniform hex_positions {
    vec2 positions[64];
};
uniform uint total_hex;
uniform uvec2 u_resolution;

out vec4 color;

void main() {
    vec2 st = gl_FragCoord.xy / u_resolution.xy; // Normalize screen coordinates

    // Initialize color to the default color
    color = vec4(st, 0.3, 1.0);

    // Iterate over the hex positions
    for (uint i = 0u; i < total_hex; i++) {
        // Calculate the distance from the current fragment to the hex position
        float dist = length(st - positions[i]);

        // Set the radius of the circle (you can adjust this value)
        float radius = 0.1;

        // Check if the fragment is inside the circle
        if (dist < radius) {
            // If inside and closer than previous, update the color to red
            color = vec4(positions[i], 0.0, 1.0);
        }
    }
}