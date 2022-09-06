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

// <inject:complex.glsl>
// <inject:utils.glsl>
// <inject:colors.glsl>

vec4 get_color(uint iterations) {
    vec4 color_0 = vec4(0.0f, 0.0f, 0.0f, 1.0f);
    vec4 color_1 = vec4(0.0f, 0.2f, 0.5f, 1.0f);
    vec4 color_2 = vec4(1.0f, 0.8f, 0.0f, 1.0f);
    vec4 color_3 = vec4(1.0f, 0.0f, 0.4f, 1.0f);

    return color_3;

    float fraction = 0.0f;
    if (iterations < ranges[1])
    {
        fraction = (iterations - ranges[0]) / (ranges[1] - ranges[0]);
        return mix(color_0, color_1, fraction);
    }
    else if(iterations < ranges[2])
    {
        fraction = (iterations - ranges[1]) / (ranges[2] - ranges[1]);
        return mix(color_1, color_2, fraction);
    }
    else
    {
        fraction = (iterations - ranges[2]) / (ranges[3] - ranges[2]);
        return mix(color_2, color_3, fraction);
    }
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
        color = get_color(i);
    } else {
        depth = uvec2(i, 0);
        color = vec4(0, 0, 0, 1);
    }
}
