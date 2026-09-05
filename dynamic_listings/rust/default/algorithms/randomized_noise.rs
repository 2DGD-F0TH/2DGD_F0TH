# //! ```cargo
# //! [dependencies]
# //! rand = "0.10"
# //! ```
#
# #![allow(dead_code, unused)]
#
# struct Texture;
# impl Texture {
#     fn new(_: u32, _: u32) -> Self { todo!() }
#     fn set_color(&mut self, _: u32, _: u32, _: &Color) {}
# }
#
# struct Color;
# impl Color {
#     fn new(_: u8, _: u8, _: u8) -> Self { todo!() }
# }
#
# fn main() {
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// We create an empty texture
let mut texture = Texture::new(WIDTH, HEIGHT);

// Now we iterate through each pixel of the texture
for row in 0..HEIGHT {
    for pixel in 0..WIDTH {
        // We create a random gray color (0 is black, 255 is white)
        let rand_gray_tone = rand::random_range(0..=255);
        // Most colors are made of Red Green and Blue, by placing them at the
        // same value, we get a tone of gray
        let rand_color = Color::new(rand_gray_tone, rand_gray_tone, rand_gray_tone);
        texture.set_color(row, row, &rand_color);
    }
}
# }
