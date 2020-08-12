\null\clearpage

The Game Loop
==============

\epigraph{All loops are infinite ones for faulty RAM modules.}{\textit{Anonymous}}

The Input-Update-Draw Abstraction
-----------------------------------

As animations and movies are an illusion, so are games. Games and movies show still images tens of times per second, giving us the illusion of movement.

Any game and its menus can be abstracted into 3 main operations that are performed one after the other, in a loop:

1) Process the user input
2) Update the world (or menu) status
3) Display (Draw) the updated world (or again, menu) to the screen

![UML Diagram of the input-update-draw abstraction](./images/gameloop/gameloop.pdf){width=50%}

So a pseudocode implementation of such loop would be something like the following:

\code{gameloop/gameloop}{Game Loop example}

This abstraction will become really useful when dealing with many rows of code and keeping it neatly organized.

Input
-----

### Events vs Real Time Input

Some frameworks may be able to further abstract how they process input by giving an *API~[g]~* that allows to make use of **events**.

Most of the time, events will be put in a queue that will be processed separately. This way it's easier to program how to react to each event and keep our code neatly organized. The downside is that the performance of an event-driven input processing is directly tied to how many events are triggered: the more events are triggered, the longer the wait may be before we get to our processed input.

This usually depends on the implementation of the event queue: an event queue is less wasteful in terms of resources and allows for less coupled code, but the queue could be cluttered with events we're not interested in (for instance mouse movement events in a game that uses only keyboard for controls) so we need to take the time to configure our event handler to ignore certain events when not necessary.

A well-configured event-based input system **is the most efficient way of doing things**, allowing code to be executed only when necessary.

On the opposite side, we have so-called "real-time input", where at a certain point of our update routine, we check for the instantaneous status of the input peripherals and process it immediately. This allows for a faster, more reactive code and to apply some different logic (for instance pressing left and right on the keyboard can be coded to make the character stop). Besides being more immediate, this system shares a lot of traits with "polling" which can be performance-heavy, as well as inducing some undesired code coupling.

Again, a well-implemented and well-configured event-based system should feel no different from real-time input, with the advantage of having better performance and having less code coupling.

Timing your loop {#timingloops}
----------------

When it comes to anything that remotely relates to physics (that includes videogames), we need to set the relation to time in our loop. There are many ways to set our delta time (or time steps), we'll see some of the most common.

### What is a time step

A time step (or delta time) is a number that will define "how much time passed" between two "snapshots" of our world (remember, the world is updating and showing in discrete intervals, giving the illusion of movement). This number will allow us to make our loop more flexible and react better to the changes of load and machines.

### Fixed Time Steps

The first and simplest way is to use a fixed time step, our delta time is fixed to a certain number, which makes the simulation easier to calculate but also makes some heavy assumptions:

- Vertical Synchronization is active in the game
- The PC is powerful enough to make our game work well, 100% of the time

An example of fixed time step loop can be the following (assuming 60 frames per second or $dt=\frac{1}{60}$):

\code{gameloop/fixed_timesteps}{Game loop with fixed timesteps}

Everything is great, until our computer starts slowing down (high load or just not enough horsepower), in that case the game will slow down.

This means that every time the computer slows down, even for a microsecond, the game will slow down too, which can be annoying.

### Variable Time Steps {#variable_timesteps}

A way to limit the issues given by a fixed time step approach is to make use of variable time steps, which are simple in theory, but can prove hard to manage.

The secret is measuring how much time passed between the last frame and the current frame, and use that value to update our world.

An example in pseudocode could be the following:

\code{gameloop/variable_timesteps}{Game loop with variable time steps}

This allows to smooth the possible lag spikes, even allowing us to disable Vertical Sync and have a bit less input lag, but this approach has some drawbacks too.

Since the delta time now depends on the speed of the game, the game can "catch up" in case of slowdowns; that can result in a slightly different feeling, depending on the framerate, but if there is a really bad slowdown `dt` can become really big and break our simulation, and collision detection will probably be the first victim.

Also this method can be a bit harder to manage, since every movement will have to be scaled with `dt`.

### Semi-fixed Time Steps

This is a special case, where we set an upper limit for our time steps and let the update loop execute as fast as possible. This way we can still simulate the world in a somewhat reliable way, avoiding the dangers of higher spikes.

A semi-fixed time step approach is the following (assuming 60 fps or $dt=\frac{1}{60}$):

\code{gameloop/semifixed_timesteps}{Game loop with Semi-Fixed time steps}

This way, if the loop is running too slow, the game will slow down and the simulation won't blow up. The main disadvantage of this approach is that we're taking more update steps for each draw step, which is fine if drawing takes more than updating the world. If instead the update phase of the loop takes more than drawing it, we will spiral into a terrible situation.

We can call it a "spiral of death", where the simulation will take Y seconds (real time) to simulate X seconds (of game time), with $Y>X$, being behind in your simulation makes the simulation take more steps, which will make the simulation fall behind even more, thus making the simulation lag behind more and more.

### Frame Limiting

Frame limiting is a technique where we aim for a certain duration of our game loop. If an iteration of the game loop is faster than intended, such iteration will wait until we get to our target loop duration.

Let's again consider a loop running at 60fps (or $dt=\frac{1}{60}$):

\code{gameloop/frame_limiting}{Game loop with Frame Limiting}

Even if the frame is limited, it's necessary that all updates are tied to our delta time to work correctly. With this loop the game will run **at most** at 60 frames per second, if there is a slowdown the game will slow down under 60 fps, if the game runs faster it won't go over 60fps.

### Frame Skipping/Dropping

A common solution used when a frame takes longer to update and render than the target time is using the so-called "frame dropping". The game won't render the next frame, in an effort to "catch up" to the desired frame rate.

This will obviously cause a perceptible visual stutter.

### Multithreaded Loops

Higher budget (AAA) games don't usually use a variation of the "classic" game loop, but instead make use of the capabilities of newer hardware. Using multiple threads (lines of execution) executing at the same time, making everything quicker and the framerate higher.

Multithreaded loops are created in a way that separates the input-update part of the game loop from the drawing part of it. This way the update thread can take care of updating our simulation, while the drawing/rendering loop can take care of drawing the result to screen.

The catch is that we can't just wait for the input-update thread to finish before rendering, that wouldn't make it quicker than just using a one-threaded game loop: instead we make the rendering thread "lag behind" the input-update thread by *1 frame* - this way while the input-update thread takes care of the frame number $n$, the drawing thread will be rendering the prepared frame number $n-1$.

| Thread       |        |        |        |        |        |        |
| :----------: | :----: | :----: | :----: | :----: | :----: | :----: |
| Updating     | 1      | 2      | 3      | 4      | 5      | 6      |
| Rendering    |        | 1      | 2      | 3      | 4      | 5      |


This 1-frame difference between updating and rendering introduces lag that can be quantified between *16.67ms* (at 60fps) and *33.3ms* (at 30fps), which needs to be added with the 2-5 ms of the LCD refresh rate, and other factors that can contribute to lag. In some games where extreme precision is needed, this could be considered unacceptable, so a single-threaded loop could be considered more fitting.

Issues and possible solutions
-----------------------------

In this section we have a little talk about some common issues related to the game loop and its timing, and some possible solutions

### Frame/Screen Tearing

Screen tearing is a phenomenon that happens when the "generate output" stage of the game loop happens in the middle of the screen drawing a frame.

This makes it so that a part of the drawn frame shows the result of an output stage, while another part shows a more updated version of the frame, given by a more recent game loop iteration.

![An example of screen tearing](./images/gameloop/screentearing.png)

A very used fix for this phenomenon is **double buffering**, where two color buffers are used. While the first is shown on screen, the game loop updates and draws on the second color buffer.

When comes the time to draw the color buffer on screen, an operation called "flipping" is performed, where the second color buffer is shown on screen, so that the game loop can draw on the first color buffer.

To smooth the game, a technique called "triple buffering" can be used, which adds a third color buffer is used to make the animation smoother at the cost of a higher input lag.

Drawing to screen
------------------

When drawing to screen, the greatest majority of games make use of what is called the "painter's algorithm", which looks something like the following:

> 1. Clear the screen
>
> 2. Draw The Farthest Background
>
> 3. Draw The Second Farthest Background
>
> 4. Draw The Tile Map
>
> 5. Draw The enemies and obstacles
>
> 6. Draw The Player
>
> 7. Display everything on screen

If we divide each "layer" we can see how the painter's algorithm works:

![A small example of the "painter's algorithm"](./images/gameloop/painter_algorithm.png){width=40%}

As a painter, we draw the background items before the foreground ones, layering each one on top of the other. Sometimes games make use of priority queues to decide which items to draw first, other times game developers (usually under the time constraints of a game jam) just hard-code the draw order.

### Clearing the screen

Special note to clearing the screen: this is an operation that sometimes may look useless but, like changing the canvas for a painter, clearing the screen (or actually the "buffer" we're drawing on) avoids a good deal of graphical glitches.

![How not clearing the screen can create glitches](./images/gameloop/glitchy_noclear.png){width=50%}

In the previous image, we can see how a black screen with only a FPS counter can end up drawing all kinds of glitches when the screen buffer is not cleared: we can clearly see the FPS counter, but the rest of the screen should be empty, instead the GPU is trying to represent residual data from its memory, causing the glitches.
