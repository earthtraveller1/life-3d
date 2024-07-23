#version 430 core

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec3 a_normal;
layout (location = 2) in vec2 a_uv;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

layout (std140, binding = 0) buffer instance_offsets {
    vec3 offset[];
};

out vec3 position;

void main() {
    const vec3 world_position = a_position + offset[gl_InstanceID];
    gl_Position = projection * view * model * vec4(world_position, 1.0);
    position = a_position;
}
