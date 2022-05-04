#version 130
#extension GL_ARB_gpu_shader_fp64 : require

out vec4 color;

uniform double xMin;
uniform double xMax;
uniform double yMin;
uniform double yMax;

uniform double height;
uniform double width;

dvec2 cSqr(dvec2 z) {
	return dvec2(z.x * z.x - z.y * z.y, 2 * z.x * z.y);
}
bool m(dvec2 c) {
	dvec2 z = dvec2(0, 0);
	for(int i = 0; i < 30; i++)
		z = cSqr(z) + c;
	return length(z) < 4;
}

void main() {
	dvec2 c = dvec2(
		xMin + (xMax - xMin) * (gl_FragCoord.x / width),
		yMax - (yMax - yMin) * (gl_FragCoord.y / height));
	if(m(c))
		color = vec4(0, 0, 0, 1);
	else
		color = vec4(1, 1, 1, 1);
}
