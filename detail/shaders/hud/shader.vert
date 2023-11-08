#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 proj;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_color;

layout(location = 1) out vec3 out_color;

void main() 
{
    gl_Position = ubo.proj * vec4(in_position, 1.0);
	out_color = in_color;
}