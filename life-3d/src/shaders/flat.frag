#version 430 core

out vec4 out_color;

in vec3 position;

uniform float cell_size;

void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}
