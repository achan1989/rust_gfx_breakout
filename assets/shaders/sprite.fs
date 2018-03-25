#version 330 core

in vec2 TexCoords;
out vec4 target;

uniform sampler2D image;
uniform Locals {
    vec4 spriteColour;
    mat4 model;
};

void main()
{
    vec4 texColour = spriteColour * texture(image, TexCoords);
    if (texColour.a < 0.1)
        discard;
    target = texColour;
}
