#version 450

layout(location=0) in  vec2 uv;
layout(location=0) out vec4 color;

layout(set = 0, binding = 0) uniform texture2D bitmap;
layout(set = 0, binding = 1) uniform sampler bitmap_sampler;

void main() {
    color = texture(sampler2D(bitmap, bitmap_sampler), uv);
}
