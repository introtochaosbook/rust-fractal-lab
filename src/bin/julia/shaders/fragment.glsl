#version 140
#extension GL_ARB_shader_subroutine : require

out vec4 color;

uniform float xMin;
uniform float xMax;
uniform float yMin;
uniform float yMax;

uniform float height;
uniform float width;

uniform uint maxColors;

// <inject:complex.glsl>
// <inject:colors.glsl>

subroutine vec2 f_t(vec2 z);
subroutine uniform f_t F;

subroutine(f_t)
vec2 FCosh(vec2 z) {
    return complex_cos(z);
}

subroutine(f_t)
vec2 FSinh(vec2 z) {
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

subroutine vec4 special_color_mode_t(uint i);
subroutine uniform special_color_mode_t SpecialColorMode;

subroutine(special_color_mode_t)
vec4 SpecialColorModeDefault(uint i) {
    vec3 s = Color(float(i) / float(maxColors * 2u));
    return vec4(s.xyz, 1);
}

subroutine(special_color_mode_t)
vec4 SpecialColorModeCloud(uint i) {
    switch (i / 2u) {
        // light grey
        case 4u: return vec4(211.0/255.0, 211.0/255.0, 211.0/255.0, 1);
        // dark grey
        case 5u: return vec4(100.0/255.0, 100.0/255.0, 100.0/255.0, 1);
        // white
        default: return vec4(1, 1, 1, 1);
    }
}

void main() {
    vec2 z = vec2(
        xMin + (xMax - xMin) * (gl_FragCoord.x / width),
        yMin + (yMax - yMin) * (gl_FragCoord.y / height));

    const float attract = 0.0001;

    color = vec4(1, 1, 1, 1);

    for (uint i = 0u; i < maxColors * 2u; i++) {
        // Apply function
        z = F(z);
        float mag = length(z);
        if (mag < attract) {
            // Point is an attractor
            break;
        } else if (mag >= 100) {
            // Point escaped
            color = SpecialColorMode(i);
            break;
        }
    }
}
