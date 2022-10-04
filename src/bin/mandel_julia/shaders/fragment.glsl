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

uniform uint iterations;

uniform uvec4 ranges;
uniform uvec4 ranges_2;

uniform bool is_mandelbrot;

// <inject:complex.glsl>
// <inject:colors.glsl>
// <inject:julia_funcs.glsl>

uint get_ranges_value(uint index) {
    if (index < 4u) {
        return ranges[index];
    }

    return ranges_2[index - 4u];
}

vec3 get_color(uint iterations) {
    vec3 colors[8] = vec3[]( Color(0.0), Color(1.0 / 7.0), Color(2.0 / 7.0), Color(3.0 / 7.0), Color(4.0 / 7.0), Color(5.0 / 7.0), Color(6.0 / 7.0), Color(0.9) );

    // based on https://physicspython.wordpress.com/2020/03/04/visualizing-the-mandelbrot-set-using-opengl-part-2/
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

subroutine vec4 special_color_mode_t(uint i);
subroutine uniform special_color_mode_t SpecialColorMode;

subroutine(special_color_mode_t)
vec4 SpecialColorModeDefault(uint i) {
    return vec4(get_color(i), 1);
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
    vec2 c = vec2(
        xMin + (xMax - xMin) * (gl_FragCoord.x / width),
        yMin + (yMax - yMin) * (gl_FragCoord.y / height));

    uint i = 0u;
    if (is_mandelbrot) {
        float mag = 0;
        const float escape = 4.0;
        vec2 z = vec2(0, 0);

        while (i++ < iterations && mag < escape) {
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
    } else {
        vec2 z = c;

        const float attract = 0.0001;

        color = vec4(1, 1, 1, 1);
        depth = uvec2(0, 1);

        while (i++ < iterations) {
            // Apply function
            z = F(z);
            float mag = length(z);
            if (mag < attract) {
                // Point is an attractor
                break;
            } else if (mag >= 100) {
                // Point escaped
                depth = uvec2(i, 0);
                color = SpecialColorMode(i);
                break;
            }
        }
    }
}
