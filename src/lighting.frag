#version 330 core
struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct DirectionalLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;

    float attenuation_constant;
    float attenuation_linear;
    float attenuation_quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct SpotLight {
    vec3 position;
    vec3 direction;

    float inner_cutoff;
    float outer_cutoff;

    float attenuation_constant;
    float attenuation_linear;
    float attenuation_quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

#define NUM_POINT_LIGHTS 4

uniform Material uMaterial;
uniform DirectionalLight uDirectionalLight;
uniform PointLight uPointLights[NUM_POINT_LIGHTS];
uniform SpotLight uSpotLight;
uniform vec3 uViewPos;
uniform int uFlashlight;

in vec3 aNormal;
in vec3 aFragPos;
in vec2 aTextureCoords;

out vec4 aFragColours;

vec3 compute_directional_lighting(DirectionalLight light, vec3 normal, vec3 view_dir);
vec3 compute_point_lighting(PointLight light, vec3 normal, vec3 frag_pos, vec3 view_dir);
vec3 compute_spot_lighting(SpotLight light, vec3 normal, vec3 frag_pos, vec3 view_dir);

void main()
{
    vec3 normal = normalize(aNormal);
    vec3 view_dir = normalize(uViewPos - aFragPos);

    vec3 result = compute_directional_lighting(uDirectionalLight, normal, view_dir);
    for (int i = 0; i != NUM_POINT_LIGHTS; ++i) {
        result += compute_point_lighting(uPointLights[i], normal, aFragPos, view_dir);
    }

    if (uFlashlight > 0) {
        result += compute_spot_lighting(uSpotLight, normal, aFragPos, view_dir);
    }

    aFragColours = vec4(result, 1.0);
}

vec3 compute_directional_lighting(DirectionalLight light, vec3 normal, vec3 view_dir)
{
    vec3 light_dir = normalize(-light.direction);
    vec3 reflect_dir = reflect(-light_dir, normal);

    float diffuse_factor = max(dot(normal, light_dir), 0.0);
    float specular_factor = pow(max(dot(view_dir, reflect_dir), 0.0), uMaterial.shininess);

    vec3 diffuse_colours = texture(uMaterial.diffuse, aTextureCoords).rgb;
    vec3 specular_colours = texture(uMaterial.specular, aTextureCoords).rgb;

    vec3 ambient = light.ambient * diffuse_colours;
    vec3 diffuse = light.diffuse * diffuse_factor * diffuse_colours;
    vec3 specular = light.specular * specular_factor * specular_colours;

    return (ambient + diffuse + specular);
}

vec3 compute_point_lighting(PointLight light, vec3 normal, vec3 frag_pos, vec3 view_dir)
{
    vec3 light_dir = normalize(light.position - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, normal);

    float diffuse_factor = max(dot(normal, light_dir), 0.0);
    float specular_factor = pow(max(dot(view_dir, reflect_dir), 0.0), uMaterial.shininess);
    float distance = length(light.position - frag_pos);
    float attenuation = 1.0 / (light.attenuation_constant + light.attenuation_linear * distance + light.attenuation_quadratic * distance * distance);

    vec3 diffuse_colours = texture(uMaterial.diffuse, aTextureCoords).rgb;
    vec3 specular_colours = texture(uMaterial.specular, aTextureCoords).rgb;

    vec3 ambient = light.ambient * diffuse_colours;
    vec3 diffuse = light.diffuse * diffuse_factor * diffuse_colours;
    vec3 specular = light.specular * specular_factor * specular_colours;

    return attenuation * (ambient + diffuse + specular);
}

vec3 compute_spot_lighting(SpotLight light, vec3 normal, vec3 frag_pos, vec3 view_dir)
{
    vec3 light_dir = normalize(light.position - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, normal);

    float diffuse_factor = max(dot(normal, light_dir), 0.0);
    float specular_factor = pow(max(dot(view_dir, reflect_dir), 0.0), uMaterial.shininess);

    float theta = dot(light_dir, normalize(-light.direction));
    float epsilon = light.inner_cutoff - light.outer_cutoff;
    float intensity = clamp((theta - light.outer_cutoff) / epsilon, 0.0, 1.0);

    float distance = length(light.position - frag_pos);
    float attenuation = 1.0 / (light.attenuation_constant + light.attenuation_linear * distance + light.attenuation_quadratic * distance * distance);

    vec3 diffuse_colours = texture(uMaterial.diffuse, aTextureCoords).rgb;
    vec3 specular_colours = texture(uMaterial.specular, aTextureCoords).rgb;

    vec3 ambient = light.ambient * diffuse_colours;
    vec3 diffuse = intensity * light.diffuse * diffuse_factor * diffuse_colours;
    vec3 specular = intensity * light.specular * specular_factor * specular_colours;

    return attenuation * (ambient + diffuse + specular);
}