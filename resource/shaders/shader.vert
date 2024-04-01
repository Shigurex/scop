#version 330 core

void main() {
    gl_Position = gl_ModelViewProjectionMatrix * gl_Vertex;
}

