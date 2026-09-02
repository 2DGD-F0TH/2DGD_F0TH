# #![allow(dead_code)]
# struct Mesh;
# struct Texture;
# struct Vector2D;
struct Common {
    mesh: Mesh,
    texture: Texture,
}

struct FlyWeight {
    common_pointer: Common,
    position: Vector2D,
    scale_factor: f32,
}
