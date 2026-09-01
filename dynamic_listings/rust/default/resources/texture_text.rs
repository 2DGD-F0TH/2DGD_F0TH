const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;
const SPRITESHEET: Surface = ENGINE.load_spritesheet("resources/font.png");

fn cell_from_letter(letter: char) -> u32 {
    /*
     * Here we will use a bit of ASCII magic, each letter
     * has a "number attached to it", related to its position
     * in the ASCII table. A starts at position 65 and each letter
     * follows, until Z at position 90.
     * This means we can "convert" each letter to its ASCII index,
     * subtract 65 and have our "array index"
     */
    let ascii_index = letter as u32;
    let array_index = ascii_index - 65;
    array_index
}

fn make_text(to_write: &str) -> Surface {
    // First, we create the surface to write the text onto
    let surface_width = TILE_WIDTH * to_write.len() as u32;
    let mut final_text = Surface::new(surface_width, TILE_HEIGHT);
    // This will keep track of the left side of the first free "cell"
    let mut current_pixel = 0;
    // Now we iterate each letter of the word we want to render
    for letter in to_write.chars() {
        // First, we need to find which cell corresponds to the letter
        let letter_cell = cell_from_letter(letter);
        // After that, we need to get the subsurface, which contains only the letter
        // It will start at letter_cell * TILE_WIDTH
        let graphical_letter = SPRITESHEET.subsurface(
            letter_cell * TILE_WIDTH, // Left
            0,                        // Top
            TILE_WIDTH,               // Width
            TILE_HEIGHT,              // Height
        );
        // Now that we have the subsurface, we can draw it on our final surface, to render the text
        final_text.draw(graphical_letter, (current_pixel, 0));
        // Now we prepare for the next loop, by increasing the current pixel by 32 (thus moving
        // our "cursor" right)
        current_pixel += TILE_WIDTH;
    }
    // After the loop exits, our text is ready to be used
    final_text
}
