#version 460

layout(set = 1, binding = 0) uniform sampler2D albedo_map;
layout(set = 1, binding = 1) uniform sampler2D normal_map;

layout(location = 1) in vec2 frag_texcoord;
layout(location = 2) in vec3 frag_normal;

layout(location = 4) in vec3 light_pos;
layout(location = 5) in vec3 view_pos;
layout(location = 6) in vec3 frag_pos;
layout(location = 7) in mat3 tbn;

layout(location = 0) out vec4 out_color;

void main() 
{
	vec4 tex_color = texture(albedo_map, frag_texcoord);
	vec3 color = tex_color.rgb;

	vec3 ambient = 0.05 * color;

	vec3 sampled_normal = texture(normal_map, frag_texcoord).rgb;
	sampled_normal = normalize(sampled_normal * 2.0 - 1.0);
	sampled_normal = normalize(tbn * sampled_normal);
	vec3 normal = sampled_normal;

	// vec3 normal = -sampled_normal;
	// float old_x = normal.x;
	// normal.x = normal.z;
	// normal.z = old_x;

	// normal = frag_normal;

    // diffuse
    vec3 light_dir = normalize(light_pos - frag_pos);
    
    float diff = max(dot(light_dir, normal), 0.0);
    vec3 diffuse = diff * color;

    // specular
    vec3 view_dir = normalize(view_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = 0.0;

	vec3 halfway_dir = normalize(light_dir + view_dir);  
	spec = pow(max(dot(normal, halfway_dir), 0.0), 32.0);

    vec3 specular = vec3(0.3) * spec; // assuming bright white light color
	specular *= (length(ambient) / 255.0);
    out_color = vec4(ambient + diffuse + specular, tex_color.a);
    // out_color = vec4(normal, 1.0);
}