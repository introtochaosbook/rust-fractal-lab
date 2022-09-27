#version 330
#extension GL_ARB_shader_subroutine : require
#extension GL_ARB_gpu_shader_fp64 : require

out vec4 color;
out uvec2 depth;

uniform double xMin;
uniform double xMax;
uniform double yMin;
uniform double yMax;

uniform float height;
uniform float width;

uniform uint maxColors;

uniform uvec4 ranges;

// <inject:complex.glsl>
// <inject:colors.glsl>

vec3 get_color(uint iterations) {
    vec3 color_0 = Color(0.0);
    vec3 color_1 = Color(0.5);
    vec3 color_2 = Color(0.75);
    vec3 color_3 = Color(1.0);

    // based on https://physicspython.wordpress.com/2020/03/04/visualizing-the-mandelbrot-set-using-opengl-part-2/
    float fraction = 0.0f;
    if (iterations < ranges[1]) {
        fraction = float(iterations - ranges[0]) / float(ranges[1] - ranges[0]);
        return mix(color_0, color_1, fraction);
    } else if(iterations < ranges[2]) {
        fraction = float(iterations - ranges[1]) / float(ranges[2] - ranges[1]);
        return mix(color_1, color_2, fraction);
    } else {
        fraction = float(iterations - ranges[2]) / float(ranges[3] - ranges[2]);
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

    while (i++ < maxColors * 2u && mag < escape) {
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
