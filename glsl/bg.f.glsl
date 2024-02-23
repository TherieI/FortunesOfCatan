#version 450 core

// Hex positions of the board
uniform HexPositions {
    vec2 positions[64];
};
uniform uint total_hex;
uniform uvec2 screen_size;

out vec4 color;

void main() {
    vec2 st = gl_FragCoord.xy / screen_size; // Normalize screen coordinates

    color = vec4(total_hex / 64, total_hex / 64, total_hex / 64, 1.0);

    // Iterate over the hex positions
    for (uint i = 0u; i < total_hex; ++i) {
        // Calculate the distance from the current fragment to the hex position
        float dist = length(st - positions[i]);

        // Set the radius of the circle (you can adjust this value)
        float radius = 0.1;

        // Check if the fragment is inside the circle
        if (dist < radius) {
            // If inside, set the color to red
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    }
}