#version 330 core
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

uniform Material uMaterial;

uniform vec3 uLightColour;
uniform vec3 uLightPos;
uniform vec3 uViewPos;

in vec3 aNormals;
in vec3 aCubePositions;

out vec4 aFragColour;

void main()
{
    vec3 normals = normalize(aNormals);
    vec3 light_dir = normalize(uLightPos - aCubePositions);
    vec3 view_dir = normalize(uViewPos - aCubePositions);
    vec3 reflect_dir = reflect(-light_dir, normals);

    vec3 ambient = uLightColour * uMaterial.ambient;
    vec3 diffuse = max(dot(normals, light_dir), 0.0) * uLightColour * uMaterial.diffuse;
    vec3 specular = pow(max(dot(reflect_dir, view_dir), 0.0), uMaterial.shininess) * uLightColour * uMaterial.specular;

    vec3 result = ambient + diffuse + specular;
	aFragColour = vec4(result, 1.0);
}