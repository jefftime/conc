#version 450

vec2[] verts = {vec2(1.0, 0.0), vec2(0.0, 1.0), vec2(-1.0, 0.0)};

void main() {
    gl_Position = vec4(verts[gl_VertexIndex], 0.0, 1.0);
}
