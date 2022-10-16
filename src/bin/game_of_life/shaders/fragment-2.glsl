#version 140

out vec4 color;

uniform sampler2D state;
uniform uvec2 scale;

in vec2 v_tex_coords;

void main() {
    color = texture(state, gl_FragCoord.xy / scale);
}
