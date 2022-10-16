#version 140

out vec4 color;

uniform sampler2D state;
uniform uvec2 scale;

in vec2 v_tex_coords;

int get(int x, int y) {
    return int(texture2D(state, v_tex_coords));
}

void main() {
    if (get(0, 0) == 1) {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
