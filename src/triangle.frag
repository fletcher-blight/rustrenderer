#version 330 core
out vec4 FragColour;

uniform vec4 singleColour;

void main()
{
    FragColour = singleColour;
}