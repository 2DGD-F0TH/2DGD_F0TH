TILE_WIDTH: int = 32
TILE_HEIGHT: int = 32
SPRITESHEET: Surface = engine.load_spritesheet("resources/font.png")


def get_cell_from_letter(letter: str) -> int:
    """
    Here we will use a bit of ASCII magic, each letter
    has a "number attached to it", related to its position
    in the ASCII table. A starts at position 65 and each letter
    follows, until Z at position 90.
    This means we can "convert" each letter to its ASCII index,
    subtract 65 and have our "array index"
    """
    ascii_index: int = ord(letter)
    array_index: int = ascii_index - 65
    return array_index


def make_text(to_write: str) -> Surface:
    # First, we create the surface to write the text onto
    surface_width: int = TILE_WIDTH * len(to_write)
    final_text: Surface = Surface(surface_width, TILE_HEIGHT)
    # This will keep track of the left side of the first free "cell"
    current_pixel: int = 0
    # Now we iterate each letter of the word we want to render
    for letter in to_write:
        # First, we need to find which cell corresponds to the letter
        letter_cell: int = get_cell_from_letter(letter)
        # After that, we need to get the subsurface, which contains only the letter
        # It will start at letter_cell * TILE_WIDTH
        graphical_letter: Surface = SPRITESHEET.get_subsurface(
            left=letter_cell * TILE_WIDTH,
            top=0,
            width=TILE_WIDTH,
            height=TILE_HEIGHT
        )
        # Now that we have the subsurface, we can draw it on our final surface, to render the text
        final_text.draw(graphical_letter, (current_pixel, 0))
        # Now we prepare for the next loop, by increasing the current pixel by 32 (thus moving
        # our "cursor" right)
        current_pixel += TILE_WIDTH
    # After the loop exits, our text is ready to be used
    return final_text
