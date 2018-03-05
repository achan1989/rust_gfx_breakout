#version 330 core

in vec2 position;
in vec2 texCoords;

out vec2 TexCoords;

uniform Locals {
    vec4 spriteColour;
    mat4 model;
};
uniform mat4 projection;

void main()
{
    TexCoords = texCoords;
    gl_Position = projection * model * vec4(position, 0.0, 1.0);
}
