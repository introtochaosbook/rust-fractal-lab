#version 330
#extension GL_ARB_shader_subroutine : require

out vec4 depth;

uniform float xMin;
uniform float xMax;
uniform float yMin;
uniform float yMax;

uniform float height;
uniform float width;

uniform uint maxColors;

// <inject:complex.glsl>
// <inject:utils.glsl>

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

    gl_FragDepth = 100;

    if (mag < escape) {
        color = vec4(0, 0, 0, 1);
    } else {
        vec3 s = Color(float(i) / float(maxColors));
        color = vec4(s.xyz, 1);
    }
}
