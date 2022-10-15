#version 330 core
layout (location = 0) in vec3 AttrVertices;
layout (location = 1) in vec3 AttrNormals;

out vec3 Normals;
out vec3 Vertices;
out vec3 Positions;

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

void main()
{
    Normals = mat3(transpose(inverse(Model))) * AttrNormals;
    Vertices = AttrVertices;
    Positions = vec3(Model * vec4(AttrVertices, 1.0));
	gl_Position = Projection * View * Model * vec4(AttrVertices, 1.0);
}