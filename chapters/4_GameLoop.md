\null\clearpage
The Game Loop
==============

The Input-Update-Draw Abstraction
-----------------------------------

As animations and movies are an illusion, so are games. Games and movies show still images tens of times per second, giving us the illusion of movement.

Any game and its menus can be abstracted into 3 main operations that are performed one after the other, in a loop:

1) Process the user input
2) Update the world (or menu) status
3) Display (Draw) the updated world (or again, menu) to the screen

So a pseudocode implementation of such loop would be something like the following:

~~~~~~
function game():
    game_is_running=True
    while game_is_running:
        process_user_input()
        update_world()
        draw()
~~~~~~

Input
-----

### Events vs Real Time Input

Timing your loop
----------------

### Fixed Time Steps

<!-- Accurate but depends on the speed of the PC: slower PC = slower game -->

### Variable Time Steps

<!-- Better with slower PCs, allows to disable VSync and have less input lag,
but tends to blow up at really slow framerates (players jump higher with lower framerates,
bullet-through-paper problem) also every movement has to be scaled with dt -->

### Frame Limiting

<!-- A special case of Variable Time Steps, where you have the update cycle
run as fast as possible, while the render cycle updates at 30 or 60 fps,
this solves some issues but is a bit harder to manage -->

### Frame Skipping/Dropping

<!-- When a frame takes too long to update and render, it might be useful to
just update the world "on paper" and skip the frame rendering, this will
cause a small stutter in the game, but will allow the render to "catch up"
to the update -->

### Multithreaded Loops

<!-- Talk about how you can split the game loop into an "update thread" and a
"render thread" and how some issues are solved by making the rendering happen
on frame later than the update, and how this introduces input lag -->
