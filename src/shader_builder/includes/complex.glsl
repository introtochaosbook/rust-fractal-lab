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
    float denom = z2.x * z2.x + z2.y * z2.y;
    float real = (z1.x * z2.x + z1.y * z2.y) / denom;
    float imag = (z2.x * z1.y - z1.x * z2.y) / denom;
    return vec2(real, imag);
}

vec2 complex_cos(vec2 z) {
    float real = cos(z.x) * cosh(z.y);
    float imag = -sin(z.x) * sinh(z.y);
    return vec2(real, imag);
}

vec2 complex_sin(vec2 z) {
    float real = sin(z.x) * cosh(z.y);
    float imag = cos(z.x) * sinh(z.y);
    return vec2(real, imag);
}

vec2 complex_exp(vec2 z) {
    float real = exp(z.x) * cos(z.y);
    float imag = exp(z.x) * sin(z.y);
    return vec2(real, imag);
}
