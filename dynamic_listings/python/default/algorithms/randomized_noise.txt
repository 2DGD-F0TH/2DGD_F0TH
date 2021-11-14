from random import randint
WIDTH = 800
HEIGHT = 600
# We create an empty texture
texture = Texture(WIDTH, HEIGHT)

# Now we iterate through each pixel of the texture
for row in texture:
    for pixel in row:
        # We create a random gray color (0 is black, 255 is white)
        rand_gray_tone = randint(0,255)
        # Most colors are made of Red Green and Blue, by placing them at the
        # same value, we get a tone of gray
        rand_color = Color(rand_gray_tone, rand_gray_tone, rand_gray_tone)
        pixel.setColor(rand_color)
