#version 330 core
layout (location = 0) in vec3 AttrVertices;
layout (location = 1) in vec2 AttrTextureCoords;

out vec2 TextureCoords;

void main()
{
	gl_Position = vec4(AttrVertices, 1.0);
	TextureCoords = AttrTextureCoords;
}