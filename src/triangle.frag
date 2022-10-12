#version 330 core
out vec4 FragColor;
in vec2 InTextureCoords;

uniform sampler2D InTexture;

void main()
{
	FragColor = texture(InTexture, InTextureCoords);
}