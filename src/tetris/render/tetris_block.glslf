#version 150 core

in vec2 v_TexCoord;
out vec4 Target0;

layout (std140)
uniform Transients {
	mat4 u_transform;
    vec4 u_tintColor;
};

uniform sampler2D t_albedoMap;

void main() {
    Target0 = u_tintColor * texture(t_albedoMap, v_TexCoord);
}