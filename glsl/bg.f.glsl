#version 450 core
// https://www.shadertoy.com/view/NlyfWm
#define PI 3.1415926535897932384626433832795
#define E  2.7182818284590452353602874713526

out vec4 color;

// Hex positions of the board
uniform hex_positions {
    vec2 positions[64];
};
uniform uint total_hex;
uniform uvec2 u_resolution;
uniform float u_scale;
uniform float u_time;

// Pixel Dimensions
const float pixel_scale = 12.0;
const int pixel_x = int(5 * pixel_scale * u_scale);
const int pixel_y = int(4 * pixel_scale * u_scale);

// COLORS
const vec4 TAN = vec4(1.0, 0.95, 0.85, 1.0);
const vec4 LIGHT_BLUE = vec4(0.059, 0.898, 0.91, 1.0);

// Hexagon size
const float HEX_SCALE = 0.9 * u_scale;
// Not sure how I came up with this constant but without it the points from positions[64] dont match with the hex's
const float SCALE_FACTOR = 0.5;
const float ASPECT_RATIO = float(u_resolution.x) / float(u_resolution.y);

// Random 1D value from (-1, 1)
float rand1d(float value) {
    float k = 0.545;
    float a = -0.584;
    float b = -0.52;
    float c = 0.855;
    float d = -0.655;
	float random = k * (a*sin(E * value) + c*cos(PI * value) + c*sin(d * value));
    return random;
}

// (x, y) -> (r, θ)
vec2 polar_coordinates_of(vec2 pos) {
    return vec2(length(pos), atan(pos.y,  pos.x));
}

// If r ≤ sec(1/3 * arcsin(sin(3θ))), then the point is within the hexgon
bool in_hexagon(vec2 polar_pos, float radius_scale = 1) {
    float radius = polar_pos.x;
    float angle = polar_pos.y;
    // Check if the point is within the hexagon equation
    return radius / radius_scale <= HEX_SCALE / cos((1.0 / 3.0) * asin(sin(3.0 * angle)));
}

void main() {
    // Pixellate
    vec2 frag_coord = gl_FragCoord.xy;
    frag_coord.x -= (float(int(gl_FragCoord.x * 1.0) % pixel_x) / 1.0);
    frag_coord.y -= (float(int(gl_FragCoord.y * 1.0) % pixel_y) / 1.0);
    vec2 st = frag_coord.xy / u_resolution.xy; // Normalize screen coordinates

    // Initialize color to the default color
    color = vec4(0.15, 0.3, 0.7, 1.0);

    // Center is the average of all the hexagon points
    vec2 center = vec2(0);

    bool land = false;
    // Iterate over the hex positions
    for (uint i = 0u; i < total_hex; i++) {
        // A normalized point that is (x, y) distance from (0, 0)
        vec2 delta_origin = st - positions[i] * SCALE_FACTOR;

        center += positions[i] * SCALE_FACTOR;

        // Check if the fragment is inside hexagon
        if (in_hexagon(polar_coordinates_of(vec2(delta_origin.x * ASPECT_RATIO, delta_origin.y)))) {
            // On land
            color = TAN;
            land = true;
        }


    }
    center /= total_hex; 
    // Draw ocean
    if (!land) {
        float pct = distance(st, center);
        vec2 pos = vec2(st - center);
        vec2 polar = polar_coordinates_of(vec2(pos.x * ASPECT_RATIO, pos.y));
        vec3 waves = LIGHT_BLUE.rgb * (vec3(1) - vec3(pct * sin(u_time / 20) / 2.0) / 2.0);
        color *= vec4(waves, 1.0);
    }

    // pixelate(color, center, st, 100);
}