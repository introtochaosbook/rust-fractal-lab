#version 400
#define MAX_VERTICES 256

layout(points) in;
layout(points, max_vertices=MAX_VERTICES) out;

uniform float pan_hor;
uniform float pan_vert;
uniform float zoom;
uniform float height;

uniform vec2 c_range;
uniform bool is_price;

float map(float x, float in_min, float in_max, float out_min, float out_max) {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

void main()
{
    // x sweeps from -1 to 1
    float x = gl_in[0].gl_Position[0] / zoom + pan_hor;

    // Use x to sweep c across its range
    float c = map(x, -1.0, 1.0, c_range[0], c_range[1]);

    float f_x = is_price ? 0.9 : 0.0;
    for (int i = 0; i < MAX_VERTICES; i++) {
        if (is_price) {
            f_x = c * f_x - c * pow(f_x, 2);
        } else {
            f_x = pow(f_x, 2) + c;
        }

        if (i > 50) {
            float f_x_remapped = (f_x - pan_vert) * zoom;
            gl_Position = gl_in[0].gl_Position + vec4(0.0f, f_x_remapped, 0.0f, 0.0f);
            EmitVertex();
            EndPrimitive();
        }
    }
}