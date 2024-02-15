WIDTH = 800
HEIGHT = 600
-- We create an empty texture
local texture = Texture:new({width=WIDTH, height=HEIGHT})

-- We seed the random number generator
math.randomseed(os.time())

-- Now we iterate through each pixel of the texture
for row = 1, #texture do
    for col = 1, #row do
        -- We create a random gray color (0 is black, 255 is white)
        local rand_gray_tone = math.random(0,255)
        -- Most colors are made of Red Green and Blue, by placing them at the
        -- same value, we get a tone of gray
        local rand_color = Color:new({
            red = rand_gray_tone,
            green = rand_gray_tone,
            blue = rand_gray_tone
        })
        texture[row][col].setColor(rand_color)
    end
end
