#version 450 core

// Hex positions of the board
uniform hex_positions {
    vec2 positions[64];
};
uniform uint total_hex;
uniform uvec2 u_resolution;
uniform float u_scale;

out vec4 color;

const vec4 BROWN = vec4(1.0, 0.95, 0.85, 1.0);

// Hexagon size
const float HEX_SCALE = 0.9 * u_scale;

// Not sure how I came up with this constant but without it the points from positions[64] dont match with the hex's
const float SCALE_FACTOR = 0.5;

const float ASPECT_RATIO = float(u_resolution.x) / float(u_resolution.y);

// If 'r' falls into the equation, r ≤ sec(1/3 * arcsin(sin(3θ))), then the point is within the hexgon
bool is_land(vec2 polar_pos) {
    float radius = polar_pos.x;
    float angle = polar_pos.y;
    // Check if the point is within the hexagon equation
    return radius <= HEX_SCALE / cos((1.0 / 3.0) * asin(sin(3.0 * angle)));
}

// (x, y) -> (r, θ)
vec2 polar_coordinates_of(vec2 pos) {
    return vec2(length(pos), atan(pos.y,  pos.x));
}

void main() {
    vec2 st = gl_FragCoord.xy / u_resolution.xy; // Normalize screen coordinates

    // Initialize color to the default color
    color = vec4(st, 0.3, 1.0);

    // Iterate over the hex positions
    for (uint i = 0u; i < total_hex; i++) {
        // A normalized point that is (x, y) distance from (0, 0)
        vec2 delta_origin = st - positions[i] * SCALE_FACTOR;


    
        // Check if the fragment is inside hexagon
        if (is_land(polar_coordinates_of(vec2(delta_origin.x * ASPECT_RATIO, delta_origin.y)))) {
            // On land
            color = BROWN;
        }
    }
}