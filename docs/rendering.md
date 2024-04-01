# Rendering Mechanism using OpenGL
## VBO (Vertex Buffer Object)
VBO is a buffer that stores the vertex data inside the GPU memory.

```
pub struct Vbo {
    id: GLuint
}

unsafe {
    let mut vbo = Self { id: 0 };
    gl::GenBuffers(1, &mut vbo.id);
}
```

- `GL_STREAM_DRAW`: the data is set only once and used by the GPU at most a few times.
- `GL_STATIC_DRAW`: the data is set only once and used many times.
- `GL_DYNAMIC_DRAW`: the data is changed a lot and used many times.

<!-- # Rendering Pipeline
Rendering Pipeline is a series of steps that are used to render a 3D scene.

A reference for shaders written in glsl is available at:
> https://codelabo.com/posts/20200228150223

For more detail about OpenGL
> https://registry.khronos.org/OpenGL/specs/gl/glspec33.core.pdf

1. Vertex Shader
2. Tessellation Shader
3. Geometry Shader
4. Fragment Shader -->
