#include <cstdlib>
#include <ctime>

// We seed the randomizer (using the system time)
std::srand(std::time(nullptr));

const int WIDTH = 800;
const int HEIGHT = 600;
// We create an empty texture
Texture texture(WIDTH, HEIGHT);

// Now we iterate through each pixel of the texture
for (int row = 0; row < HEIGHT; ++ row) {
    for (int pixel = 0; pixel < WIDTH; ++pixel) {
        // We create a random gray color (0 is black, 255 is white)
        int rand_gray_tone = std::rand() % 256;
        // Most colors are made of Red Green and Blue, by placing them at the
        // same value, we get a tone of gray
        Color rand_color = Color(rand_gray_tone, rand_gray_tone, rand_gray_tone);
        pixel.setColor(rand_color);
    }
}
