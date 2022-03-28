#version 400
#define MAX_VERTICES 256

layout(points) in;
layout(points, max_vertices=MAX_VERTICES) out;

uniform float shift_r;
uniform float shift_x;
uniform float zoom;
uniform float height;

float map(float x, float in_min, float in_max, float out_min, float out_max) {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

void main()
{
    // x sweeps from -1 to 1
    float x = gl_in[0].gl_Position[0];

    // Sweep c from -2 to 0.25
    float c = map(x, -1, 1, -2, 0.25);

    float f_x = 0;
    for (int i = 0; i < MAX_VERTICES; i++) {
        f_x = pow(f_x, 2) + c;

        if (i > 50) {
            gl_Position = gl_in[0].gl_Position + vec4(0.0f, f_x, 0.0f, 0.0f);
            EmitVertex();
            EndPrimitive();
        }
    }
}