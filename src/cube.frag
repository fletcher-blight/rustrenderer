#version 330 core
out vec4 FragColor;

uniform vec3 ObjectColour;
uniform vec3 LightColour;

void main()
{
	FragColor = vec4(ObjectColour * LightColour, 1.0);
}