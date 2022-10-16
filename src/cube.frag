#version 330 core
struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform Material uMaterial;
uniform Light uLight;
uniform vec3 uViewPos;

in vec3 aNormals;
in vec3 aCubePositions;
in vec2 aTextureCoords;

out vec4 aFragColour;

void main()
{
    vec3 normals = normalize(aNormals);
    vec3 light_dir = normalize(-uLight.direction);
    vec3 view_dir = normalize(uViewPos - aCubePositions);
    vec3 reflect_dir = reflect(-light_dir, normals);

    vec3 diffuse_colours = texture(uMaterial.diffuse, aTextureCoords).rgb;
    vec3 specular_colours = texture(uMaterial.specular, aTextureCoords).rgb;

    float diffuse_factor = max(dot(normals, light_dir), 0.0);
    float specular_factor = pow(max(dot(reflect_dir, view_dir), 0.0), uMaterial.shininess);

    vec3 ambient  = uLight.ambient * diffuse_colours;
    vec3 diffuse  = uLight.diffuse * (diffuse_factor * diffuse_colours);
    vec3 specular = uLight.specular * (specular_factor * specular_colours);

    vec3 result = ambient + diffuse + specular;
	aFragColour = vec4(result, 1.0);
}