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

This abstraction will become really useful when dealing with many rows of code and keeping it neatly organized.

Input
-----

### Events vs Real Time Input

Some frameworks may be able to further abstract how they process input by giving an *API~[g]~* that allows to make use of **events**.

Most of the time, events will be put in a queue that will be processed separately. This way it's easier to program how to react to each event and keep our code neatly organized. The downside is that the performance of an event-driven input processing is directly tied to how many events are triggered: the more events are triggered, the longer the wait may be before we get to our processed input. Also in case of conflicting inputs (for instance pressing left and right on the keyboard), the one that gets processed later in the queue might take over.

On the opposite side, we have so-called "real-time input", where at a certain point of our update routine, we check for the instantaneous status of the input peripherals and process it immediately. This allows for a faster, more reactive code and to apply some different logic (for instance pressing left and right on the keyboard can be coded to make the character stop).

Timing your loop
----------------

When it comes to anything that remotely relates to physics (that includes videogames), we need to set the relation to time in our loop. There are many ways to set our delta time (or time steps), we'll see some of the most common.

### What is a time step

A time step (or delta time) is a number that will define "how much time passed" between two "snapshots" of our world (remember, the world is updating and showing in discrete intervals, giving the illusion of movement). This number will allow us to make our loop more flexible and react better to the changes of load and machines.

### Fixed Time Steps

The first and simplest way is to use a fixed time step, our delta time is fixed to a certain number, which makes the simulation easier to calculate but also makes some heavy assumptions:

- Vertical Synchronization is active in the game
- The PC is powerful enough to make our game work well, 100% of the time

An example of fixed time step loop can be the following (assuming 60 frames per second or $dt=\frac{1}{60}$):

~~~~~
dt = 1.0/60.0
game_is_running = True

while game_is_running:
    process_input()
    update_world(dt)
    render()
~~~~~

Everything is great, until our computer starts slowing down (high load or just not enough horsepower), in that case the game will slow down.

This means that every time the computer slows down, even for a microsecond, the game will slow down too, which can be annoying.

### Variable Time Steps

A way to limit the issues given by a fixed time step approach is to make use of variable time steps, which are simple in theory, but can prove hard to manage.

The secret is measuring how much time passed between the last frame and the current frame, and use that value to update our world.

An example in pseudocode could be the following:

~~~~~
game_is_running = True

while game_is_running:
    dt = measure_time_from_last_frame()
    process_input()
    update_world(dt)
    render()
~~~~~

This allows to smooth the possible lag spikes, even allowing us to disable Vertical Sync and have a bit less input lag, but this approach has some drawbacks too.

Since the delta time now depends on the speed of the game, the game can "catch up" in case of slowdowns; that can result in a slightly different feeling, depending on the framerate, but if there is a really bad slowdown `dt` can become really big and break our simulation, and collision detection will probably be the first victim.

Also this method can be a bit harder to manage, since every movement will have to be scaled with `dt`.

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
