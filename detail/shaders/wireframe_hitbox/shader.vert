#version 450

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
	vec3 color;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec3 in_normal;
layout(location = 3) in vec3 in_tangent;
layout(location = 4) in vec3 in_bitangent;

layout(location = 1) out vec3 out_color;

void main() 
{
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(in_position, 1.0);
	out_color = ubo.color;
}