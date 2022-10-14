#version 330 core
out vec4 FragColor;

in vec3 Normals;
in vec3 Positions;

uniform vec3 ObjectColour;
uniform vec3 LightColour;
uniform vec3 LightPos;

void main()
{
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * LightColour;

    vec3 norm = normalize(Normals);
    vec3 lightDir = normalize(LightPos - Positions);
    float diffusionStrength = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diffusionStrength * LightColour;

    vec3 res = (ambient + diffuse) * ObjectColour;
	FragColor = vec4(res, 1.0);
}