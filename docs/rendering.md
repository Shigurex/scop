# Rendering Mechanism using OpenGL
## Rendering Pipeline
Initial vertex data will be processed as follows in OpenGL.
In OpenGL, it is necessary to define the vertex shader and fragment shader. (geometry shader is optional)

1. Vertex Shader
2. Geometry Shader
3. Shape Assembly
4. Rasterization
5. Fragment Shader
6. Tests and Blending

## Version Compatibility

Version Compatibility of OpenGL and GLSL is as follows.

OpenGL        |GLSL        |version          |suffix |ITC|UBlock
--------------|------------|-----------------|-------|---|------
OpenGL 3.0    |GLSL 1.3	   |#version 130     |u,f    |◎  |-
OpenGL 3.1    |GLSL 1.4	   |#version 140     |u,f    |◎  |U
OpenGL 3.2    |GLSL 1.5    |#version 150     |u,f    |◎  |U,I,O
OpenGL ES 3.0 |GLSL ES 3.0 |#version 300 es  |u,f    |-  |U
OpenGL 3.3    |GLSL 3.3	   |#version 330     |u,f    |◎  |U,I,O
OpenGL 4.0    |GLSL 4.0	   |#version 400     |u,f,lf |◎  |U,I,O
OpenGL 4.1    |GLSL 4.1	   |#version 410     |u,f,lf |◎  |U,I,O
OpenGL 4.2    |GLSL 4.2	   |#version 420     |u,f,lf |◎  |U,I,O
OpenGL 4.3    |GLSL 4.3	   |#version 430     |u,f,lf |◎  |U,I,O,B
OpenGL 4.4    |GLSL 4.4	   |#version 440     |u,f,lf |◎  |U,I,O,B

## Shader

To create, run the following command.
```
pub unsafe fn CreateShader(type_: GLenum) -> GLuint
```

In order to attach the shader to the program, use the following command.
Shader is the return value of `CreateShader`, count is the number of elements in string array, string is the source code of the shader, and length is the length of the array strings.
```
pub unsafe fn ShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint
)
```

Then, to compile the shader, use the following command.
```
pub unsafe fn CompileShader(shader: GLuint)
```

## VBO (Vertex Buffer Object)
VBO is a buffer that stores the vertex data inside the GPU memory.

To generate a VBO with a buffer ID, use the following command. The buffer has a unique ID corresponding to the buffer so that it can be referenced by the buffer ID.

```
pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint)
```

To bind newly created buffer, run the following commands. By the way, the type to bind for vertex buffer is `gl::ARRAY_BUFFER`.

```
pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint)
```

To set the data of the buffer, use the following command.

```
pub unsafe fn BufferData(
    target: GLenum,
    size: GLsizeiptr,
    data: *const c_void,
    usage: GLenum
)
```

The fourth parameter is used to manage the way ti process data. Choose one of them from the below to set the way to process data.

- `gl::STREAM_DRAW`: the data is set only once and used by the GPU at most a few times.
- `gl::STATIC_DRAW`: the data is set only once and used many times.
- `gl::DYNAMIC_DRAW`: the data is changed a lot and used many times.

## VAO (Vertex Array Object)
VAO is a buffer that stores the vertex array data inside the GPU memory.



## Reference
For more detail about OpenGL
> https://registry.khronos.org/OpenGL/specs/gl/glspec33.core.pdf

<!--
For more detail about OpenGL
> https://registry.khronos.org/OpenGL/specs/gl/glspec33.core.pdf
-->
