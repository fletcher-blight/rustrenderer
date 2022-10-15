#version 330 core
out vec4 FragColor;

in vec3 Normals;
in vec3 Vertices;
in vec3 Positions;

uniform vec3 ObjectColour;
uniform vec3 LightColour;
uniform mat4 LightModel;
uniform vec3 ViewPos;
uniform float Intensity;

void main()
{
    float ambientStrength = 0.2;
    vec3 ambient = ambientStrength * LightColour;

    vec3 lightCoords = vec3(LightModel * vec4(Vertices, 1.0));

    vec3 norm = normalize(Normals);
    vec3 lightDir = normalize(lightCoords - Positions);
    float diffusionStrength = max(dot(norm, lightCoords), 0.0);
    vec3 diffuse = diffusionStrength * LightColour;

    float specularStrength = 0.5;
    vec3 viewDir = normalize(ViewPos - Positions);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * LightColour;

    vec3 res = Intensity * (ambient + diffuse + specular) * ObjectColour;
	FragColor = vec4(res, 1.0);
}