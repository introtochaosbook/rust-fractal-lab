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
// <inject:julia_funcs.glsl>

subroutine vec4 colorize_t(uint i);
subroutine uniform colorize_t Colorize;

subroutine(colorize_t)
vec4 ColorizeDefault(uint i) {
    vec3 s = ColorMap(float(i) / float(maxColors));
    return vec4(s.xyz, 1);
}

subroutine(colorize_t)
vec4 ColorizeCloud(uint i) {
    switch (i / 2u) {
        // light grey
        case 4u: return vec4(211.0/255.0, 211.0/255.0, 211.0/255.0, 1);
        // dark grey
        case 5u: return vec4(100.0/255.0, 100.0/255.0, 100.0/255.0, 1);
        // white
        default: return vec4(1, 1, 1, 1);
    }
}

subroutine(colorize_t)
vec4 ColorizeSnowflakes(uint i) {
    if (i >= 8u) {
        return vec4(0, 0, 0, 1);
    } else if (i >= 12u) {
        return vec4(1, 1, 1, 1);
    }

    return vec4(0, 0, 0, 1);
}

void main() {
    vec2 z = vec2(
        xMin + (xMax - xMin) * (gl_FragCoord.x / width),
        yMin + (yMax - yMin) * (gl_FragCoord.y / height));

    const float attract = 0.0001;

    color = vec4(1, 1, 1, 1);

    for (uint i = 0u; i < maxColors; i++) {
        // Apply function
        z = F(z);
        float mag = length(z);
        if (mag < attract) {
            // Point is an attractor
            break;
        } else if (mag >= 100) {
            // Point escaped
            color = Colorize(i);
            break;
        }
    }
}
