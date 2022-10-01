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

subroutine(special_color_mode_t)
vec4 SpecialColorModeSnowflakes(uint i) {
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
