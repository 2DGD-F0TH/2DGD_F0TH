TILE_WIDTH = 32
TILE_HEIGHT = 32
SPRITESHEET = engine.load_spritesheet("resources/font.png")

local function get_cell_from_letter(letter)
    --[[
    -- Here we will use a bit of ASCII magic, each letter
    -- has a "number attached to it", related to its position
    -- in the ASCII table. A starts at position 65 and each letter
    -- follows, until Z at position 90.
    -- This means we can "convert" each letter to its ASCII index,
    -- subtract 65 and have our "array index"
    --
    --]]
    local ascii_index = string.byte(letter)
    local array_index = ascii_index - 65
    return array_index
end

local function make_text(to_write)
    -- First, we create the surface to write the text onto
    local surface_width = TILE_WIDTH * to_write.length()
    local final_text = Surface:new({surface_width, TILE_HEIGHT})
    -- This will keep track of the left side of the first free "cell"
    local current_pixel = 0
    -- Now we iterate each letter of the word we want to render
    for i = 1, #to_write do
        local letter = to_write:sub(i,i)
        -- First, we need to find which cell corresponds to the letter
        local letter_cell = get_cell_from_letter(letter)
        -- After that, we need to get the subsurface, which contains only the letter
        -- It will start at letter_cell * TILE_WIDTH
        local graphical_letter = SPRITESHEET:get_subsurface({
            left = letter_cell * TILE_WIDTH,
            top = 0,
            width = TILE_WIDTH,
            height = TILE_HEIGHT
        })
        -- Now that we have the subsurface, we can draw it on our final surface, to render the text
        final_text.draw(graphical_letter, {current_pixel, 0})
        -- Now we prepare for the next loop, by increasing the current pixel by 32 (thus moving
        -- our "cursor" right)
        current_pixel = current_pixel + TILE_WIDTH
    end
    -- After the loop exits, our text is ready to be used
    return final_text
end
