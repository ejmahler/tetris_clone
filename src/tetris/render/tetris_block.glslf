#version 150 core

in vec2 v_TexCoord;
out vec4 Target0;

layout (std140)
uniform Globals {
	mat4 u_transform;
    vec4 u_tintColor;
};

void main() {
    Target0 = u_tintColor;
}