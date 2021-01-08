#version 450

const vec2 uv[4] = vec2[4](vec2(0, 0), vec2(1, 0), vec2(0, 1), vec2(1, 1));

void main() {
    gl_Position = vec4(2 * uv[gl_VertexIndex] - 1, 0, 1);
}
