#version 450

layout(binding = 0) uniform UniformBufferObject {
	vec2 foo;
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec2 inTexCoord;
layout(location = 2) in vec3 inNormal;

layout(location = 1) out vec2 fragTexCoord;
layout(location = 2) out vec3 Normal;
layout(location = 3) out vec3 WorldNormal; // Added output for WorldNormal

void main() 
{
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(inPosition, 1.0);
	fragTexCoord = inTexCoord;
	Normal = inNormal;
	WorldNormal = mat3(ubo.model) * inNormal; // Transforming normal into world space
}