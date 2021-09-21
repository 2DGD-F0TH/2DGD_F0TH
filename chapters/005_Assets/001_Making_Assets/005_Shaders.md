Shaders
--------

### What are shaders

Shaders are technically small programs that (usually) run inside your Graphics Card (GPU). In gaming they are usually used for post-processing and special effect, allowing to free the CPU from a lot of workload, using the specialized GPU capabilities.

Shaders can be classified in different groups:

- **2D Shaders:** These shaders act on textures and modify the attributes of pixels.
    - **Pixel (Fragment) Shaders:** Used to compute color and other attributes relating to a single output pixel. They can be used for blur, edge detection or enhancement and cel/cartoon shading.
- **3D Shaders:** These shaders act on 3D models or other geometry.
    - **Vertex Shaders:** These shaders are run once per vertex given to the GPU, converting the 3D position in virtual space to the 2D coordinates of your screen. These shaders cannot create any new geometry.
    - **Geometry Shaders:** These shaders can create new primitives, like points or triangles.
    - **Tessellation Shaders:** These shaders allow to divide simple meshes into finer ones at runtime, this allows the meshes closest to the camera to have finer details, while the further ones will be less detailed.
    - **Primitive Shaders:** Akin to the computing shaders, but have access to data to process geometry.
- **Computing Shaders:** These shaders are not limited to graphics, but are related to the world of GPGPU (General Purpose computing on GPU), these can be used to further stages in animation or lighting.

### Shader Programming Languages

There are numerous programming languages, depending on the platform and libraries you are using to program your game.

If you are using OpenGL, you should use the official OpenGL Shading Language, called **GLSL**.

```{.frag caption="Simple GLSL Fragment shader"}
#ifdef GL_ES
precision lowp float;
#endif

uniform float u_time;

void main() {
    gl_FragColor = vec4(1.0,0.0,0.0,1.0);
}
```

If you are using Direct3D, you should instead use the "High Level Shader Language", also called **HLSL**.

If instead you want to use Vulkan, you will need to use the **SPIR-V** (Standard Portable Intermediate Representation) format, but the good news is that (at the time of writing) you can convert your GLSL code into SPIR-V and use it with Vulkan.

Modern engines, like Unity and Unreal Engine also include GUI node-based editors that help you create new shaders by using directed graphs, without using any code.

![Godot's "Visual Shader" Editor](./images/resources/godot_visual_shader.png){width=60%}

### The GLSL Programming Language

{{placeholder}}

<!-- TODO: Teach some basic GLSL -->

#### The data types

{{placeholder}}
<!-- TODO: Talk about GLSL data types -->

### Some GLSL Shaders examples

{{placeholder}}

<!-- TODO: Add some simple 2D fragment shaders examples -->
