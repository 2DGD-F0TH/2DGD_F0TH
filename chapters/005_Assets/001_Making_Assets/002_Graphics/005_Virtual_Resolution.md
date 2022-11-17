### Virtual Resolution {#VirtualRes}

There are times where having the crispest graphics around is not a good idea, for instance in Pixel-Art games.

Times where it's a better idea to keep your game low-resolution, be it for a matter of performance (your first games won't be extremely optimized and will be slow even at low map sizes and resolutions) or to keep your pixel-art crisp and "pixelly".

This clashes with the continuous push towards higher resolutions, which can mess up your game in a variety of ways, like the following ones.

If the game is forced in windowed mode, you'll have a problem like the following:

![Windowed Game Example - A 640x480 game in a 1920x1080 Window](./images/resources/virtual_res_fail_1.svg){width=60%}

The game window is way too small and hard to see, and can get even smaller if the *[HUD~\[g\]~](#gl_hud)* takes even more space out of the window. This can be mitigated by calculating the position of each element in comparison to the window size, this although can result in items too small (or too big if downscaling), like the following HUD example.

![Fullscreen Game Example - Recalculating items positions according to the window size](./images/resources/virtual_res_fail_2.svg){width=60%}

This is where **virtual resolution** comes into play, the frame is rendered in a virtual surface which operates at a "virtual resolution" and the whole frame is drawn and upscaled (or downscaled) to the screen, without having to recalculate anything in real time (thus making the game faster and lighter).

![Fullscreen Game Example - Virtual Resolution](./images/resources/virtual_res.svg){width=60%}

The items get scaled accordingly and there is no real need to do heavy calculations. Virtual Resolution allows for different kinds of upscaling, with or without filtering or interpolation, for instance:

- **No filtering** - Useful for keeping the "pixeliness" of your graphics, for instance in pixel-art-based games. It's fast.;
- **Linear, Bilinear, Trilinear Filtering** - Gives a more soft look, slower;
- **Anisotropic Filtering** - Used in modern 3D games, highest quality but also among the slowest, it is usually used when rendering sloped surfaces (from the player's point of view).

For more details, check the [filtering](#texture_filtering) section.
