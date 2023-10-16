#version 450

layout(binding = 0) uniform UniformBufferObject {
	vec3 light_pos;
	vec3 view_pos;
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec3 in_normal;
layout(location = 3) in vec3 in_tangent;

layout(location = 1) out vec2 out_uv;
layout(location = 2) out vec3 out_normal;

layout(location = 4) out vec3 out_light_pos;
layout(location = 5) out vec3 out_view_pos;
layout(location = 6) out vec3 out_frag_pos;
layout(location = 7) out mat3 tbn;

void main() 
{
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(in_position, 1.0);
	out_uv = in_uv;
	out_normal = mat3(transpose(inverse(ubo.model))) * in_normal;  
	out_light_pos = ubo.light_pos;
	out_view_pos = ubo.view_pos;
	out_frag_pos = vec3(ubo.model *  vec4(in_position, 1.0));

	vec3 N = normalize(mat3(ubo.model) * in_normal);
    vec3 T = normalize(mat3(ubo.model) * in_tangent);
    vec3 B = cross(N, T);

    tbn = mat3(T, B, N);
}