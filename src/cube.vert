#version 330 core
layout (location = 0) in vec3 AttrVertices;

void main()
{
	gl_Position = vec4(AttrVertices, 1.0);
}