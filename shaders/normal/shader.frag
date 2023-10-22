#version 450

layout(binding = 1) uniform sampler2D albedo_sampler;
layout(binding = 2) uniform sampler2D normal_sampler;

layout(location = 1) in vec2 frag_texcoord;
layout(location = 2) in vec3 frag_normal;

layout(location = 4) in vec3 light_pos;
layout(location = 5) in vec3 view_pos;
layout(location = 6) in vec3 frag_pos;

layout(location = 0) out vec4 out_color;

void main() 
{
	vec4 tex_color = texture(albedo_sampler, frag_texcoord);
	vec3 color = tex_color.rgb;

	vec3 ambient = 0.05 * color;

    // diffuse
	vec3 normal = normalize(frag_normal);
	vec3 normal_color = texture(normal_sampler, frag_texcoord).rbg;
	normal_color = normalize(normal_color * 2.0 - 1.0); 
	normal_color = tbn * normal_color;

	// normal = normalize(normal + normal_color);

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
    out_color = vec4(ambient + diffuse + specular, tex_color.a);
}