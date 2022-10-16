#version 330 core
layout (location = 0) in vec3 inVertices;
layout (location = 1) in vec3 inNormals;
layout (location = 2) in vec2 inTextureCoords;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

out vec3 aNormal;
out vec3 aFragPos;
out vec2 aTextureCoords;

void main()
{
    aNormal = mat3(transpose(inverse(uModel))) * inNormals;
    aFragPos = vec3(uModel * vec4(inVertices, 1.0));
    aTextureCoords = inTextureCoords;
	gl_Position = uProjection * uView * uModel * vec4(inVertices, 1.0);
}