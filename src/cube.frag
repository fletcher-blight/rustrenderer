#version 330 core
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform Material uMaterial;
uniform Light uLight;
uniform vec3 uViewPos;

in vec3 aNormals;
in vec3 aCubePositions;

out vec4 aFragColour;

void main()
{
    vec3 normals = normalize(aNormals);
    vec3 light_dir = normalize(uLight.position - aCubePositions);
    vec3 view_dir = normalize(uViewPos - aCubePositions);
    vec3 reflect_dir = reflect(-light_dir, normals);

    float diffuse_factor = max(dot(normals, light_dir), 0.0);
    float specular_factor = pow(max(dot(reflect_dir, view_dir), 0.0), uMaterial.shininess);

    vec3 ambient  = uLight.ambient * uMaterial.ambient;
    vec3 diffuse  = uLight.diffuse * (diffuse_factor * uMaterial.diffuse);
    vec3 specular = uLight.specular * (specular_factor * uMaterial.specular);

    vec3 result = ambient + diffuse + specular;
	aFragColour = vec4(result, 1.0);
}