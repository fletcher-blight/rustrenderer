#version 330 core
layout (location = 0) in vec3 attrVertices;
layout (location = 1) in vec3 attrNormals;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

out vec3 aNormals;
out vec3 aCubePositions;

void main()
{
    aNormals = mat3(transpose(inverse(uModel))) * attrNormals;
    aCubePositions = vec3(uModel * vec4(attrVertices, 1.0));
	gl_Position = uProjection * uView * uModel * vec4(attrVertices, 1.0);
}