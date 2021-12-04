#version 450

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

layout (binding = 0) uniform Uniforms {
    vec4 color;
    mat4 mvp;
} u;

layout (location = 0) out vec4 out_color;

void main() {
    gl_Position = vec4(position, 1.0) * u.mvp;
    // out_color = vec4(color, 1.0);
    out_color = u.color;
}
