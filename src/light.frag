#version 330 core
uniform vec3 uLightColour;

out vec4 aFragColour;

void main()
{
	aFragColour = vec4(uLightColour, 1.0);
}