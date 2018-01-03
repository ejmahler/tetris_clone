#version 150 core

in vec4 in_position;
in vec2 in_uv0;
out vec2 v_TexCoord;

layout (std140)
uniform Globals {
	mat4 u_transform;
    vec4 u_tintColor;
};

void main() {
    v_TexCoord = in_uv0;
    gl_Position = u_transform * in_position;
}