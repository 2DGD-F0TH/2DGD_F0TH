Profiling your game
-------------------

### Does your application really need profiling?

In this section we will have a small check list that will let us know if our videogame really needs to be profiled. Sometimes the FPS counter is trying to tell us a different story than the one we have in our heads.

#### Does your FPS counter roam around a certain "special" value?

There are cases where the FPS counter shows a low counter, but it stays around a certain value. This means that the FPS value is artificially limited somewhere, either by VSync or something else.

Some special values you may see are:

- 25 FPS: PAL refresh rate
- 29.970 FPS: NTSC Refresh Rate
- 30 FPS: Used in some games
- 50 FPS: Used in some games
- 60 FPS: Used in most games
- 75 FPS or 80 FPS: Used in some LCD Monitors
- 90 FPS: Used mostly in VR games
- 144 FPS: Used in more modern, high-refresh rate monitors
- 240 FPS: Used in the most recent high-end games and monitors

#### Is the animation of your game stuttering but the FPS counter is fine?

If your animation stutters or its speed varies according to the load of your platform but your FPS counter is still stuck at the maximum allowed framerate, you may have forgotten to tie the animation to the delta-time in your game loop. Check the [timing your game loop](#timingloops) section for more information.

{{placeholder}}

<!-- TODO: Continue with more profiling suggestions -->

### First investigations

First of all, we need to understand what is the bottleneck of your game: check your task manager and see how your game is performing.

#### Is your game using 100% of the CPU?

Is your game using 100% of the CPU (if you're on Linux, you may see percentages over 100%, that just means your game is using more than one CPU core)?

First of all, you should check if you're using the frame limiting approaches offered by your framework or game engine: if they're not active, your game will run "as fast as possible", which means it will occupy all the CPU time it can. This can result in high FPS count (in the thousands) but high energy consumption and slowdowns in other tasks.

If you have taken all the frame limiting approaches as stated above, that may mean that the game is doing a lot of CPU work and you may need to make the game perform less work for each frame. In this case profiling tools are precious to find the spots where the CPU spends most of its time: Valgrind or GProf are great profiling tools.

![Using Valgrind's Callgrind tool and Kcachegrind we can see what is bogging down our game](./images/profiling_optimization/callgrind.png){width=60%}

#### Is your game overloading your GPU?

If instead your game is not using all of the CPU computing power, you may have a problem on the GPU: your game may be calling the drawing routines too often. The less a game has to communicate with the hardware, the higher the performance. In that case using Sprite Atlases and other "batching techniques" that allow to draw many objects with only one call will help your game perform better.

#### Is your game eating up more and more RAM as it's running?

Your game starts well enough, but after just a few minutes it starts slowing down and becomes choppy. Your may have a memory problem at hand.

If your game supports windowed mode, keep your task manager (or "top"/"htop"/"bpytop" if you're on Linux) open and look at your game's process: does the memory used by your game increase as you're playing it?

If so, you may be having a so-called *[memory leak~\[g\]~](#gl_memoryleak)*: somewhere during its running cycle, your game forgets to clean up something, which stays resident in memory until your game closes. The result, after creating and deleting a lot of entities and leaving a lot of *garbage* behind is that the total memory used increases.

This is especially common in languages like C++, where there is no automatic "garbage collecting" and having cases of so-called *[unreachable memory~\[g\]~](#gl_unreachablememory)* can really mess up your memory usage.

::: pitfall :::
Some people call unreachable memory cases "*[dangling pointers~\[g\]~](#gl_dp)*", but technically they are two different (and opposite) things.

Check the glossary for more information.
:::::::::::::::

If you suspect a memory leak, you may want to take a look at these sections:

- [Entity Cleanup and Memory Leaks](#cleanup_leaks)
- [Using memory analyzers to detect leaks](#mem_analyzers)
- [Resource Pools](#res_pools)

{{placeholder}}

<!-- TODO: Continue with more investigation suggestions -->
