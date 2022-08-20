#version 140
#extension GL_ARB_gpu_shader_fp64 : require
#extension GL_ARB_shader_subroutine : require

out vec4 color;

uniform double xMin;
uniform double xMax;
uniform double yMin;
uniform double yMax;

uniform double height;
uniform double width;

uniform uint maxColors;

vec3 color_map(float t) {
    const vec3 c0 = vec3(0.0002189403691192265, 0.001651004631001012, -0.01948089843709184);
    const vec3 c1 = vec3(0.1065134194856116, 0.5639564367884091, 3.932712388889277);
    const vec3 c2 = vec3(11.60249308247187, -3.972853965665698, -15.9423941062914);
    const vec3 c3 = vec3(-41.70399613139459, 17.43639888205313, 44.35414519872813);
    const vec3 c4 = vec3(77.162935699427, -33.40235894210092, -81.80730925738993);
    const vec3 c5 = vec3(-71.31942824499214, 32.62606426397723, 73.20951985803202);
    const vec3 c6 = vec3(25.13112622477341, -12.24266895238567, -23.07032500287172);

    return c0+t*(c1+t*(c2+t*(c3+t*(c4+t*(c5+t*c6)))));
}

vec2 complex_square(vec2 z) {
    return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

vec2 complex_add(vec2 z1, vec2 z2) {
    return vec2(z1.x + z2.x, z1.y + z2.y);
}

vec2 complex_sub(vec2 z1, vec2 z2) {
    return vec2(z1.x - z2.x, z1.y - z2.y);
}

vec2 complex_mult(vec2 z1, vec2 z2) {
    return vec2(z1.x * z2.x - z1.y * z2.y,
                z1.x * z2.y + z1.y * z2.x);
}

vec2 complex_div(vec2 z1, vec2 z2) {
    double denom = z2.x * z2.x + z2.y * z2.y;
    double real = (z1.x * z2.x + z1.y * z2.y) / denom;
    double imag = (z2.x * z1.y - z1.x * z2.y) / denom;
    return vec2(real, imag);
}

vec2 complex_cos(vec2 z) {
    double real = cos(z.x) * cosh(z.y);
    double imag = -sin(z.x) * sinh(z.y);
    return vec2(real, imag);
}

vec2 complex_sin(vec2 z) {
    double real = sin(z.x) * cosh(z.y);
    double imag = cos(z.x) * sinh(z.y);
    return vec2(real, imag);
}

vec2 complex_exp(vec2 z) {
    double real = exp(z.x) * cos(z.y);
    double imag = exp(z.x) * sin(z.y);
    return vec2(real, imag);
}

subroutine vec2 f_t(vec2 z);

subroutine uniform f_t F;

subroutine(f_t)
vec2 FCosh(vec2 z) {
    return complex_cos(z);
}

subroutine(f_t)
vec2 FRabbit(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(-0.122,0.745));
    return z;
}

subroutine(f_t)
vec2 FSiegel(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(-0.390540,-0.58679));
    return z;
}

void main() {
    vec2 z = vec2(
        xMin + (xMax - xMin) * (gl_FragCoord.x / width),
        yMin + (yMax - yMin) * (gl_FragCoord.y / height));

    const double attract = 0.0001;

    color = vec4(1, 1, 1, 1);

    for (uint i = 0u; i < maxColors * 2u; i++) {
        // Apply function
        z = F(z);
        double mag = length(z);
        if (mag < attract) {
            // Point is an attractor
            break;
        } else if (mag >= 100) {
            vec3 s = color_map(float(i/2.0) / float(maxColors));
            color = vec4(s.xyz, 1);
            break;
        }
    }
}
