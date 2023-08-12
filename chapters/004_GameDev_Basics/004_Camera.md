{{pagebreak}}

Cameras
=========

:::::: {.epigraph author="Horace"}
Nothing's beautiful from every point of view
::::::

The great majority of games don't limit the size of their own maps to the screen size only, but instead they have maps way bigger than the screen.

To be able to manage and display such maps to the screen, we need to talk about cameras (sometimes called "viewports"): they will allow us to show only a portion of our world to the user, making our game much more expansive and involving.

Screen Space vs. Game Space
----------------------------

Before starting with the most used type of cameras, we need to distinguish between what could be called "screen space" and what is instead "game space" (sometimes called "map space").

![Reference Image for Screen Space and Game Space[^jawbreaker]](./images/camera/screenspace.png){width=70%}

We talk about "game space" when the coordinates of a point we are talking about are referred to the top-left corner *of the entire game (or level) map*.

Instead we talk about "screen space" when the coordinate of such point are referred to the top-left corner *of the screen*.

Looking at our reference image, we can see how different the coordinates of the magenta dot are in screen space and in map space.

It is possible to convert screen space to map space and vice-versa by accounting for the viewport offset (represented by the red dot in the reference image), like follows:

$$screen\ coordinates = map\ coordinates - viewport\ coordinates$$

$$map\ coordinates = screen\ coordinates + viewport\ coordinates$$

In a more friendly way, we can see our viewport as a "window" that moves around the map. Alternatively, we can see it as a viewport that is still all the time but has the map scrolling under it.

[^jawbreaker]: Jawbreaker tileset, listed as public domain at [https://adamatomic.itch.io/jawbreaker](https://adamatomic.itch.io/jawbreaker)

Cameras and projections
-----------------------

:::: wizardry ::::
This subsection gives a general idea on how cameras work in a 3D engine, but it is definitely useful to better understand how cameras work in general.
::::::::::::::::::

Cameras are just an approximation of of how we see things as humans. This approximation is due to a number of tradeoffs made to make things seem realistic, but avoid the issues that reality brings with itself.

Let's look at how a person sees, in a somewhat schematic way:

![How a person sees things](./images/camera/real_vision.svg){width=40%}

A person can see anything directly in front of their eyes, to infinity (or at least until something blocks their vision, like a mountain, a building or fog).

We're definitely having a couple of problems: the first one is that we cannot represent infinity on a computer. If we try to represent everything from the camera's point of view to infinity, we won't be able to play the game at all.

The second issue is very close objects: in real life an object that is right up to your face will cover your entire vision. This may not be something that you want.

This is why computers render only things between two given planes, like the following:

![How videogame cameras see things](./images/camera/camera_vision.svg){width=40%}

A videogame camera renders only what is situated between a "close plane" and a "far plane". Moreover objects are projected onto the screen, which may deform them if odd ["Field of View" (FOV)~\[g\]~](#gl_fov) values are used.

Most used camera transitions and types
--------------------------------------

### Static Camera

This is the simplest camera we can implement: each level has the size of the screen (or of the virtual resolution we decided, see [Virtual Resolution](#VirtualRes)), and every time we go out of the map, the screen fades to black and back to the new "room".

{{placeholder}}

<!-- TODO: Simplest: each room is screen-sized, exit a room and the room gets switched, usually with a fade-to-black -->

### Grid Camera

This is an improvement on the static camera formula, each level (or room) has the size of the screen (or virtual resolution we chose), every time we go out of the map, the screen scrolls into the new section. This camera is used by the first Legend Of Zelda game for the Nintendo Entertainment System.

{{placeholder}}

<!-- TODO: Variation on the static camera, where you scroll on exit (zelda 1 style) -->

### Position-Tracking Camera

This camera is a bit more involved: the viewport tracks the position of the player and moves accordingly, so to keep the character centered on the screen. There are two types of position tracking cameras that are used in video games: horizontal-tracking and full-tracking cameras.

This type of camera can has some serious drawbacks when sudden and very quick changes of direction are involved: since the camera tracks the player all the time, the camera can feel twitchy and over-reactive; this could cause uneasiness or even nausea.

#### Horizontal-Tracking Camera

Horizontal-tracking cameras keep the player in the center of the screen horizontally, while jumps don't influence the camera position. This is ideal for games that span horizontally, since we won't have the camera moving when jumping and temporarily hiding enemies we may fall on.

![Example of an horizontally-tracking camera](./images/camera/horitrack.png){width=50%}

This is the camera used in the classic Super Mario Bros. for the Nintendo Entertainment System.

{{placeholder}}

<!-- TODO: Horizontal tracking camera (maybe code?) -->

#### Full-Tracking Camera {#fully_tracking_camera}

Sometimes our levels don't span only horizontally, so we need to track the player in both axes, keeping it in the center of the screen at all times. This is good for platformers that don't require extremely precise maneuvering, since precise maneuvering could result in way too much movement from the camera.

![Example of a full-tracking camera](./images/camera/fulltrack.png){width=50%}

{{placeholder}}

<!-- TODO: Fully tracking camera (can be jarring, maybe code?) -->


### Camera Trap

The "Camera Trap" system was invented to eliminate, or at least mitigate, the issues given by the position tracking camera. The playable character is encased in a "trap" that, when "escaped" makes the camera catch up in an effort to put the player back in such "trap".

The trap is represented by an invisible rectangle which can be visualized on screen in case you need to debug your camera.

![Example of camera trap-based system](./images/camera/cameratrap.png){width=50%}

This allows the camera to be less twitchy, giving a more natural sensation. Furthermore you can size the camera trap according to the type of game you are coding: slow-paced games can have a larger camera trap, allowing for the camera to rest more on the same screen, while faster paced games can have a smaller camera trap for faster reaction times.

{{placeholder}}

<!-- TODO: Talk about camera traps, rectangular spaces where the character is "trapped inside", as soon as the player "escapes the trap" the camera scrolls to "trap them again", this allows for less jarring camera movement. Traps can be small for fast games (sonic) or larger for slower ones -->

### Look-Ahead Camera

This is a more complex camera that is implemented when the playable character moves towards a certain direction very quickly. The Look-Ahead camera is used to show more space in front of the player, giving more time to react to upcoming obstacles or enemies.

![Example of look-ahead camera](./images/camera/lookahead.png){width=50%}

This camera needs a good implementation when it comes to changing direction: having a sudden change of direction in the player character should have a slow panning response from the camera towards the new direction, or the game will feel nauseating.

So this camera is not ideal for games that require precision platforming, since the continuous "corrections" required to hit a tight platform would move the camera around too much, giving the player nausea.

{{placeholder}}

<!-- TODO: Camera that gives more space in front of the character, allowing for better reaction times, must be coded well on transitions (between opposite sides) or it can be extremely confusing or nauseating -->

### Hybrid Approaches

There are hybrid approaches to cameras too, mixing and matching different types of camera can give your game an additional touch of uniqueness. For instance in "Legend of Zelda: A link to the past", the camera is a mix between a "camera trap" and a "grid camera", where each zone is part of a grid, and inside each "grid cell" we have a tracking system based on the "camera trap".

This allows the game to have a more dynamic feel, but also saves memory, since the SNES had to load only one "zone" at a time, instead of the whole map.

Another idea would be using an "out-of-center" camera trap that changes position according to how the player "escapes the camera trap", thus solving some of the biggest issues of the look-ahead camera.

Feel free to experiment and invent!

Clamping your camera position
-----------------------------

Whichever type of camera you decide to make use of (besides the static and grid cameras), there may be a side effect that could not be desirable: the camera tracking could follow the player so obediently that it ends up showing off-map areas.

![How the camera may end up showing off-map areas](./images/camera/offmap.png){width=70%}

Off-map areas may be shown as black in many cases, but in other cases (when the screen is not properly "cleared") the off-map area can show glitchy versions of the current map.

In this case, it will be necessary to "clamp" the camera position, this way it will still follow the player, but won't show off-map areas.

This usually just involves a check on the viewport boundaries against the map boundaries, followed by a reset of the coordinates to the map boundaries.
