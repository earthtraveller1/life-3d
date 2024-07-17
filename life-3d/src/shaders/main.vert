#version 430 core

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec3 a_normal;
layout (location = 2) in vec2 a_uv;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out vec3 position;

void main() {
    gl_Position = projection * view * model * vec4(a_position, 1.0);
    position = a_position;
}
