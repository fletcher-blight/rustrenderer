#version 330 core
layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Colour;

uniform float horizontalOffset = 0.0;

out vec3 vertexColour;

void main()
{
    gl_Position = vec4(Position.x + horizontalOffset, Position.yz, 1.0);
    vertexColour = Colour;
}