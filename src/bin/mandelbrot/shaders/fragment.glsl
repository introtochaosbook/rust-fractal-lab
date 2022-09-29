#version 330
#extension GL_ARB_shader_subroutine : require

out vec4 color;
out uvec2 depth;

uniform float xMin;
uniform float xMax;
uniform float yMin;
uniform float yMax;

uniform float height;
uniform float width;

uniform uint maxColors;

uniform uvec4 ranges;
uniform uvec4 ranges_2;

// <inject:complex.glsl>
// <inject:colors.glsl>

uint get_ranges_value(uint index) {
    if (index < 4u) {
        return ranges[index];
    }

    return ranges_2[index - 4u];
}

vec3 get_color(uint iterations) {
    vec3 colors[8] = vec3[]( Color(0.0), Color(1.0 / 7.0), Color(2.0 / 7.0), Color(3.0 / 7.0), Color(4.0 / 7.0), Color(5.0 / 7.0), Color(6.0 / 7.0), Color(1.0) );

    float fraction = 0.0f;
    for (uint i = 1u; i < 8u; i++) {
        if (iterations < get_ranges_value(i)) {
            fraction = float(iterations - get_ranges_value(i - 1u)) / float(get_ranges_value(i) - get_ranges_value(i - 1u));
            return mix(colors[i - 1u], colors[i], fraction);
        }
    }

    fraction = float(iterations - get_ranges_value(6u)) / float(get_ranges_value(7u) - get_ranges_value(6u));
    return mix(colors[6], colors[7], fraction);
}

void main() {
    vec2 c = vec2(
        xMin + (xMax - xMin) * (gl_FragCoord.x / width),
        yMin + (yMax - yMin) * (gl_FragCoord.y / height));

    uint i = 0u;
    float mag = 0;
    const float escape = 4.0;
    vec2 z = vec2(0, 0);

    while (i++ < maxColors && mag < escape) {
        z = complex_square(z) + c;
        mag = length(z);
    }

    if (mag < escape) {
        depth = uvec2(0, 1);
        color = vec4(1, 1, 1, 1);
    } else {
        depth = uvec2(i, 0);
        color = vec4(get_color(i), 1);
    }
}
