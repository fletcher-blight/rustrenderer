#version 330 core
out vec4 FragColor;

in vec2 TextureCoords;

uniform sampler2D Texture1;
uniform sampler2D Texture2;

void main()
{
	FragColor = mix(texture(Texture1, TextureCoords), texture(Texture2, vec2(-TextureCoords.x, TextureCoords.y)), 0.3);
}