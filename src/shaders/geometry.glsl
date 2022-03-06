#version 400
#define MAX_VERTICES 256

layout(points) in;
layout(points, max_vertices=MAX_VERTICES) out;

uniform float shift_r;
uniform float shift_x;
uniform float zoom;
uniform float height;

float update_x(float x, float r)
{
    return r * x * (1.0f - x);
}

void bifurcate(float r, float zoom, float x_shift)
{
    // initial guess
    float x = 0.5f;

    // throw away first N values in the series
    for (int i = 0; i < 2 * MAX_VERTICES; ++i)
    {
        x = update_x(x, r);
    }

    // emit point only if it lands inside viewspace
    int num_emited = 0;

    while (num_emited < MAX_VERTICES)
    {
        x = update_x(x, r);

        float pos = (x - x_shift) / zoom;

        if (pos >= -1.0f && pos <= 1.0f)
        {
            num_emited++;
            gl_Position = gl_in[0].gl_Position - vec4(0.0f, pos, 0.0f, 0.0f);
            EmitVertex();
            EndPrimitive();
        }
    }
}

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