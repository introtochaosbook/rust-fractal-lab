subroutine vec2 f_t(vec2 z);
subroutine uniform f_t F;

subroutine(f_t)
vec2 FCos(vec2 z) {
    return complex_cos(z);
}

subroutine(f_t)
vec2 FSin(vec2 z) {
    return complex_sin(z);
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

subroutine(f_t)
vec2 FDragon(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(0.360284, 0.100376));
    return z;
}

subroutine(f_t)
vec2 FAmoeba(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(0.3, -0.4));
    return z;
}

subroutine(f_t)
vec2 FFlower1(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(0.384, 0.0));
    return z;
}

subroutine(f_t)
vec2 FFlower2(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(0.2541, 0.0));
    return z;
}

subroutine(f_t)
vec2 FCloud(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(-0.194, 0.6557));
    return z;
}

subroutine(f_t)
vec2 FSnowflakes(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(0.11031, 0.67037));
    return z;
}

subroutine(f_t)
vec2 FDendrite(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(0.0, 1.0));
    return z;
}

subroutine(f_t)
vec2 FEkg(vec2 z) {
    z = complex_mult(z, z);
    z = complex_add(z, vec2(-1.5, 0.0));
    return z;
}