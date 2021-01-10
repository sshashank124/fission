#version 450

in  vec4 gl_FragCoord;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) readonly buffer Blit {
    vec4[] colors;
};
layout(set = 0, binding = 1) uniform FrameWidth {
    uint frame_width;
};

float srgb(float channel) {
    if (channel <= 0.0031308) { return 12.92 * channel; }
    else { return 1.055 * pow(channel, 0.416667) - 0.055; }
}

void main() {
    vec4 color = colors[int(gl_FragCoord.y) * frame_width
                      + int(gl_FragCoord.x)];
    vec3 rgb = color.rgb / color.w;
    out_color = vec4(srgb(rgb.r), srgb(rgb.g), srgb(rgb.b), 1);
}
