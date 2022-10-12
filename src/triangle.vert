#version 330 core
layout (location = 0) in vec3 InVertices;
layout (location = 1) in vec2 InTextureCoords;

out vec2 OutTextureCoords;

void main()
{
	gl_Position = vec4(InVertices, 1.0);
	OutTextureCoords = vec2(InTextureCoords.x, InTextureCoords.y);
}