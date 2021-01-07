#version 450

layout(location=0) in  vec2 uv_in;
layout(location=0) out vec2 uv_out;

void main() {
    uv_out = vec2(uv_in[0], 1 - uv_in[1]);
    gl_Position = vec4(2 * uv_in - 1, 0, 1);
}
