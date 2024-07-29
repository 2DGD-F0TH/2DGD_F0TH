#include <utility>
#include <string>

const int TILE_WIDTH = 32;
const int TILE_HEIGHT = 32;
const Surface SPRITESHEET = engine.load_spritesheet("resources/font.png");

int get_cell_from_letter(char letter){
    /*
     * Here we will use a bit of ASCII magic, each letter
     * has a "number attached to it", related to its position
     * in the ASCII table. A starts at position 65 and each letter
     * follows, until Z at position 90.
     * This means we can "convert" each letter to its ASCII index,
     * subtract 65 and have our "array index"
     */
    int ascii_index = (int) letter;
    int array_index = ascii_index - 65;
    return array_index;
}

Surface make_text(std::string to_write){
    // First, we create the surface to write the text onto
    int surface_width = TILE_WIDTH * to_write.length();
    Surface final_text = Surface(surface_width, TILE_HEIGHT);
    // This will keep track of the left side of the first free "cell"
    int current_pixel = 0;
    // Now we iterate each letter of the word we want to render
    for (char letter : to_write){
        // First, we need to find which cell corresponds to the letter
        int letter_cell = get_cell_from_letter(letter);
        // After that, we need to get the subsurface, which contains only the letter
        // It will start at letter_cell * TILE_WIDTH
        Surface graphical_letter = SPRITESHEET.get_subsurface(
            letter_cell * TILE_WIDTH,  // Left
            0,  // Top
            TILE_WIDTH,  // Width
            TILE_HEIGHT  // Height
        );
        // Now that we have the subsurface, we can draw it on our final surface, to render the text
        final_text.draw(graphical_letter, std::pair<int, int>(current_pixel, 0));
        // Now we prepare for the next loop, by increasing the current pixel by 32 (thus moving
        // our "cursor" right)
        current_pixel += TILE_WIDTH;
    }
    // After the loop exits, our text is ready to be used
    return final_text;
}
