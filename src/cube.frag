#version 330 core
struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 direction;
    float inner_cutoff;
    float outer_cutoff;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float attenuation_linear;
    float attenuation_quadratic;
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
    vec3 light_dir = normalize(uLight.position - aCubePositions);
    vec3 view_dir = normalize(uViewPos - aCubePositions);
    vec3 reflect_dir = reflect(-light_dir, normals);

    vec3 diffuse_colours = texture(uMaterial.diffuse, aTextureCoords).rgb;
    vec3 specular_colours = texture(uMaterial.specular, aTextureCoords).rgb;

    float diffuse_factor = max(dot(normals, light_dir), 0.0);
    float specular_factor = pow(max(dot(reflect_dir, view_dir), 0.0), uMaterial.shininess);

    float theta = dot(light_dir, normalize(-uLight.direction));
    float epsilon = uLight.inner_cutoff - uLight.outer_cutoff;
    float intensity = clamp((theta - uLight.outer_cutoff) / epsilon, 0.0, 1.0);

    vec3 ambient  = uLight.ambient * diffuse_colours;
    vec3 diffuse  = intensity * diffuse_factor * uLight.diffuse * diffuse_colours;
    vec3 specular = intensity * specular_factor * uLight.specular * specular_colours;

    float distance = length(uLight.position - aCubePositions);
    float attenuation = 1.0 / (1.0 + uLight.attenuation_linear * distance + uLight.attenuation_quadratic * (distance * distance));

    vec3 result = attenuation * (ambient + diffuse + specular);
	aFragColour = vec4(result, 1.0);
}