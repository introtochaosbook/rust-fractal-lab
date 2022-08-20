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

// Colormap subroutines from here: https://observablehq.com/@flimsyhat/webgl-color-maps

subroutine vec3 color_t(float t);
subroutine uniform color_t Color;

subroutine(color_t)
vec3 ColorInferno(float t) {
    const vec3 c0 = vec3(0.0002189403691192265, 0.001651004631001012, -0.01948089843709184);
    const vec3 c1 = vec3(0.1065134194856116, 0.5639564367884091, 3.932712388889277);
    const vec3 c2 = vec3(11.60249308247187, -3.972853965665698, -15.9423941062914);
    const vec3 c3 = vec3(-41.70399613139459, 17.43639888205313, 44.35414519872813);
    const vec3 c4 = vec3(77.162935699427, -33.40235894210092, -81.80730925738993);
    const vec3 c5 = vec3(-71.31942824499214, 32.62606426397723, 73.20951985803202);
    const vec3 c6 = vec3(25.13112622477341, -12.24266895238567, -23.07032500287172);

    return c0+t*(c1+t*(c2+t*(c3+t*(c4+t*(c5+t*c6)))));
}

subroutine(color_t)
vec3 ColorViridis(float t) {
    const vec3 c0 = vec3(0.2777273272234177, 0.005407344544966578, 0.3340998053353061);
    const vec3 c1 = vec3(0.1050930431085774, 1.404613529898575, 1.384590162594685);
    const vec3 c2 = vec3(-0.3308618287255563, 0.214847559468213, 0.09509516302823659);
    const vec3 c3 = vec3(-4.634230498983486, -5.799100973351585, -19.33244095627987);
    const vec3 c4 = vec3(6.228269936347081, 14.17993336680509, 56.69055260068105);
    const vec3 c5 = vec3(4.776384997670288, -13.74514537774601, -65.35303263337234);
    const vec3 c6 = vec3(-5.435455855934631, 4.645852612178535, 26.3124352495832);

    return c0+t*(c1+t*(c2+t*(c3+t*(c4+t*(c5+t*c6)))));
}

subroutine(color_t)
vec3 ColorPlasma(float t) {
    const vec3 c0 = vec3(0.05873234392399702, 0.02333670892565664, 0.5433401826748754);
    const vec3 c1 = vec3(2.176514634195958, 0.2383834171260182, 0.7539604599784036);
    const vec3 c2 = vec3(-2.689460476458034, -7.455851135738909, 3.110799939717086);
    const vec3 c3 = vec3(6.130348345893603, 42.3461881477227, -28.51885465332158);
    const vec3 c4 = vec3(-11.10743619062271, -82.66631109428045, 60.13984767418263);
    const vec3 c5 = vec3(10.02306557647065, 71.41361770095349, -54.07218655560067);
    const vec3 c6 = vec3(-3.658713842777788, -22.93153465461149, 18.19190778539828);

    return c0+t*(c1+t*(c2+t*(c3+t*(c4+t*(c5+t*c6)))));
}

subroutine(color_t)
vec3 ColorMagma(float t) {
    const vec3 c0 = vec3(-0.002136485053939582, -0.000749655052795221, -0.005386127855323933);
    const vec3 c1 = vec3(0.2516605407371642, 0.6775232436837668, 2.494026599312351);
    const vec3 c2 = vec3(8.353717279216625, -3.577719514958484, 0.3144679030132573);
    const vec3 c3 = vec3(-27.66873308576866, 14.26473078096533, -13.64921318813922);
    const vec3 c4 = vec3(52.17613981234068, -27.94360607168351, 12.94416944238394);
    const vec3 c5 = vec3(-50.76852536473588, 29.04658282127291, 4.23415299384598);
    const vec3 c6 = vec3(18.65570506591883, -11.48977351997711, -5.601961508734096);

    return c0+t*(c1+t*(c2+t*(c3+t*(c4+t*(c5+t*c6)))));
}

subroutine(color_t)
vec3 ColorTurbo(float x) {
    // Copyright 2019 Google LLC.
    // SPDX-License-Identifier: Apache-2.0
    const vec4 kRedVec4 = vec4(0.13572138, 4.61539260, -42.66032258, 132.13108234);
    const vec4 kGreenVec4 = vec4(0.09140261, 2.19418839, 4.84296658, -14.18503333);
    const vec4 kBlueVec4 = vec4(0.10667330, 12.64194608, -60.58204836, 110.36276771);
    const vec2 kRedVec2 = vec2(-152.94239396, 59.28637943);
    const vec2 kGreenVec2 = vec2(4.27729857, 2.82956604);
    const vec2 kBlueVec2 = vec2(-89.90310912, 27.34824973);

    x = clamp(x,0.0,1.0);
    vec4 v4 = vec4( 1.0, x, x * x, x * x * x);
    vec2 v2 = v4.zw * v4.z;
    return vec3(
    dot(v4, kRedVec4)   + dot(v2, kRedVec2),
    dot(v4, kGreenVec4) + dot(v2, kGreenVec2),
    dot(v4, kBlueVec4)  + dot(v2, kBlueVec2)
    );
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
            vec3 s = Color(float(i/2.0) / float(maxColors));
            color = vec4(s.xyz, 1);
            break;
        }
    }
}
