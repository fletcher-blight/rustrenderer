#version 330 core
layout (location = 0) in vec3 inVertices;
layout (location = 1) in vec3 inNormals;
layout (location = 2) in vec2 inTextureCoords;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

out vec3 aNormals;
out vec3 aCubePositions;
out vec2 aTextureCoords;

void main()
{
    aNormals = mat3(transpose(inverse(uModel))) * inNormals;
    aCubePositions = vec3(uModel * vec4(inVertices, 1.0));
    aTextureCoords = inTextureCoords;
	gl_Position = uProjection * uView * uModel * vec4(inVertices, 1.0);
}