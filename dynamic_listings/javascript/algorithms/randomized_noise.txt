let WIDTH = 800;
let HEIGHT = 600;
// We create an empty texture (assuming it can be seen as a 2D array)
let texture = new Texture(WIDTH, HEIGHT);

// Now we iterate through each pixel of the texture
for (const row in texture){
    for (const pixel in row){
        // We create a random gray color (0 is black, 255 is white)
        let rand_gray_tone = Math.random() * 255;
        // Most colors are made of Red Green and Blue, by placing them at the
        // same value, we get a tone of gray
        let rand_color = new Color(rand_gray_tone, rand_gray_tone, rand_gray_tone);
        pixel.setColor(rand_color);
    }
}
