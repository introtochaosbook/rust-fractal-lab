#version 140
#extension GL_ARB_gpu_shader_fp64 : require

out vec4 color;

uniform double xMin;
uniform double xMax;
uniform double yMin;
uniform double yMax;

uniform double height;
uniform double width;

uniform uint max_colors;

layout(std140) uniform Block {
    uint colors_r[1024];
    uint colors_g[1024];
    uint colors_b[1024];
};

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

void main() {
    vec2 c = vec2(
        xMin + (xMax - xMin) * (gl_FragCoord.x / width),
        yMax - (yMax - yMin) * (gl_FragCoord.y / height));

    uint i = 0u;
    double mag = 0;
    double escape = 4.0;
    vec2 z = vec2(0, 0);

    while (i++ < max_colors && mag < escape) {
        z = complex_square(z) + c;
        mag = length(z);
    }

    if (mag < escape) {
        color = vec4(0, 0, 0, 1);
    } else {
        color = vec4(float(colors_r[i]) / 255.0, float(colors_g[i]) / 255.0, float(colors_b[i]) / 255.0, 1);
    }
}
