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

// COLORS
const vec4 TAN = vec4(1.0, 0.95, 0.85, 1.0);
const vec4 FOAM_BLUE = vec4(0.721,0.840,0.971,1.000);
const vec4 DARK_BLUE = vec4(0.107,0.127,0.440,1.000);
const vec4 LIGHT_BLUE = vec4(0.059, 0.898, 0.91, 1.0);
const vec4 WHITE_BLUE = vec4(0.949,0.980,0.990,1.000);
// Hexagon size
const float HEX_SCALE = 0.9 * u_scale;
// Not sure how I came up with this constant but without it the points from positions[64] dont match with the hex's
const float SCALE_FACTOR = 0.5;
const float ASPECT_RATIO = float(u_resolution.x) / float(u_resolution.y);
// Pixel Dimensions
const float pixel_scale = 42.0;
const int pixel_x = int(pixel_scale * u_scale);
const int pixel_y = int(pixel_scale * u_scale);


// ========================================================================
//       Author @patriciogv - 2015 | http://patriciogonzalezvivo.com
// ========================================================================
vec3 mod289(vec3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
vec2 mod289(vec2 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
vec3 permute(vec3 x) { return mod289(((x*34.0)+1.0)*x); }

float snoise(vec2 v) {
    const vec4 C = vec4(0.211324865405187,  // (3.0-sqrt(3.0))/6.0
                        0.366025403784439,  // 0.5*(sqrt(3.0)-1.0)
                        -0.577350269189626,  // -1.0 + 2.0 * C.x
                        0.024390243902439); // 1.0 / 41.0
    vec2 i  = floor(v + dot(v, C.yy) );
    vec2 x0 = v -   i + dot(i, C.xx);
    vec2 i1;
    i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);
    vec4 x12 = x0.xyxy + C.xxzz;
    x12.xy -= i1;
    i = mod289(i); // Avoid truncation effects in permutation
    vec3 p = permute( permute( i.y + vec3(0.0, i1.y, 1.0 ))
        + i.x + vec3(0.0, i1.x, 1.0 ));

    vec3 m = max(0.5 - vec3(dot(x0,x0), dot(x12.xy,x12.xy), dot(x12.zw,x12.zw)), 0.0);
    m = m*m ;
    m = m*m ;
    vec3 x = 2.0 * fract(p * C.www) - 1.0;
    vec3 h = abs(x) - 0.5;
    vec3 ox = floor(x + 0.5);
    vec3 a0 = x - ox;
    m *= 1.79284291400159 - 0.85373472095314 * ( a0*a0 + h*h );
    vec3 g;
    g.x  = a0.x  * x0.x  + h.x  * x0.y;
    g.yz = a0.yz * x12.xz + h.yz * x12.yw;
    return 130.0 * dot(m, g);
}

// ========================================================================
//                         Author @GabrielRosas
// ========================================================================
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
        vec3 col = vec3(0.0);
        float ripples = 2 / (u_scale);
        vec2 pos = vec2(st*3.) * ripples - positions[0] * ripples;

        float DF = 0.0;

        // Add a random position
        float a = 0.0;
        vec2 vel = vec2(u_time*.1);
        DF += snoise(pos+vel)*.25+.25;

        // Add a random position
        a = snoise(pos*vec2(cos(u_time*0.15),sin(u_time*0.1))*0.1)*3.1415;
        vel = vec2(cos(a),sin(a));
        DF += snoise(pos+vel)*.25+.25;

        col = vec3( smoothstep(.732,.15,fract(DF)) );

        color = vec4(1.0-col,1.0) * mix(LIGHT_BLUE, FOAM_BLUE, 0.5 + sin(u_time / 20) / 3) + vec4(col,1.0) * mix(LIGHT_BLUE, WHITE_BLUE, 0.5 + sin(u_time / 10 + 1.234) / 5);
    }
}