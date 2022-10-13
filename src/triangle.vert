#version 330 core
layout (location = 0) in vec3 AttrVertices;
layout (location = 1) in vec2 AttrTextureCoords;

out vec2 TextureCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
	gl_Position = projection * view * model * vec4(AttrVertices, 1.0);
	TextureCoords = AttrTextureCoords;
}