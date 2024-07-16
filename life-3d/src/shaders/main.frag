#version 430 core

out vec4 out_color;

in vec3 position;

void main() {
    out_color = vec4(position, 1.0);
}
