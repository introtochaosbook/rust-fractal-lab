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
    float colors_r[256];
    float colors_g[256];
    float colors_b[256];
};

dvec2 complex_square(dvec2 z) {
    return dvec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

dvec2 complex_add(dvec2 z1, dvec2 z2) {
    return dvec2(z1.x + z2.x, z1.y + z2.y);
}

dvec2 complex_sub(dvec2 z1, dvec2 z2) {
    return dvec2(z1.x - z2.x, z1.y - z2.y);
}

dvec2 complex_mult(dvec2 z1, dvec2 z2) {
    return dvec2(z1.x * z2.x - z1.y * z2.y,
                z1.x * z2.y + z1.y * z2.x);
}

dvec2 complex_div(dvec2 z1, dvec2 z2) {
    double denom = z2.x * z2.x + z2.y * z2.y;
    double real = (z1.x * z2.x + z1.y * z2.y) / denom;
    double imag = (z2.x * z1.y - z1.x * z2.y) / denom;
    return dvec2(real, imag);
}

dvec2 complex_cos(dvec2 z) {
    double real = cos(z.x) * cosh(z.y);
    double imag = -sin(z.x) * sinh(z.y);
    return dvec2(real, imag);
}

dvec2 complex_sin(dvec2 z) {
    double real = sin(z.x) * cosh(z.y);
    double imag = cos(z.x) * sinh(z.y);
    return dvec2(real, imag);
}

dvec2 complex_exp(dvec2 z) {
    double real = exp(z.x) * cos(z.y);
    double imag = exp(z.x) * sin(z.y);
    return dvec2(real, imag);
}

void main() {
    dvec2 c = dvec2(
        xMin + (xMax - xMin) * (gl_FragCoord.x / width),
        yMax - (yMax - yMin) * (gl_FragCoord.y / height));

    uint i = 0u;
    double mag = 0;
    double escape = 4.0;
    dvec2 z = dvec2(0, 0);

    while (i++ < max_colors && mag < escape) {
        z = complex_square(z) + c;
        mag = length(z);
    }

    if (mag < escape) {
        color = vec4(0, 0, 0, 1);
    } else {
        color = vec4(colors_r[i], colors_g[i], colors_b[i], 1);
    }
}
