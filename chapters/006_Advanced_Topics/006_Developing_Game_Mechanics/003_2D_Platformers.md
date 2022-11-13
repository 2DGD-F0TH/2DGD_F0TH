2D Platformers
---------------

### Simulating Gravity

Gravity in a 2D platformer is quite easy to simulate: you just need to apply a constant acceleration towards the direction your gravity is pulling (it doesn't have to be towards the bottom of the screen!) and move your objects accordingly to such acceleration.

Your acceleration should not be precise (like the physics constant $9.81 m/s^2$), you don't want to make a physics engine: you want to make a somewhat convincing (or even better: entertaining) approximation of reality.

This is usually done before the player movement is used to update the character's status (but after the player input has been captured). Remember to add this acceleration before the collision detection is processed.

A useful precaution to avoid the [bullet through paper](#bulletthroughpaper) problem when you are working with long falls: put a limit at the fall velocity (kind of like air friction limits an object's fall velocity) of your objects. By applying a hard limit to the velocity, your gravity will be realistic but won't break your simulation.

```{src='developing_mechanics/gravity' caption='Code for applying gravity to an object'}
```

### Avoiding "Floaty Jumps"

The previous trick shows a physics-accurate jumping: if we plot the height against time, we would get something that represents the curve of jump like the following:

![Plotting a physics-accurate jump](./images/developing_mechanics/physics_accurate_jump_plot.svg){width=50%}

Although this can give the sensation that the character we're controlling is "floaty", which is not fun. In this case it's a better idea to enhance gravity when falling, to give the character some more "weight", which would be represented, more or less, by the following curve:

![Plotting a jump with enhanced gravity](./images/developing_mechanics/enhanced_gravity_jump_plot.svg){width=50%}

This can be obtained with few simple lines of code, not very different from the gravity example of earlier:

```{src='developing_mechanics/enhanced_gravity_jump' caption='Code for jump with enhanced gravity while falling'}
```

In this example we are assuming that the framework used uses the screen coordinate system, and jumping brings the player from bottom towards the top of the screen. If you want different behaviour (like gravity inversion in puzzle games), something a tiny bit more involved may be in order.

### Making jumps "float differently"

As an addendum to the previous section, you can change how gravity is applied at the peak of the jump to give the player more time to correct their movement.

This can be done by reducing gravity when the jump is peaking, thus obtaining a plot similar to the following:

![Plotting a jump with multiple gravity changes](./images/developing_mechanics/multiple_gravity_jump_plot.svg){width=50%}

This is an example of a jump with multiple "gravity changes":

```{src='developing_mechanics/multiple_gravity_jump' caption='Code for jump with more gravity while falling and less when peaking'}
```

### Ladders

{{placeholder}}

<!-- TODO: How to allow the player to use ladders -->

### Walking on slanted ground

{{placeholder}}

<!-- TODO: How to walk on slanted ground -->

### Stairs

{{placeholder}}

<!-- TODO: How to walk on stairs -->

### Ledge Grabbing

{{placeholder}}

<!-- TODO: How to grab onto ledges -->

### Jump Buffering {#jump_buffering}

A nice trick used mostly in 2D platformers to allow for smoother gameplay is "jump buffering", also known as "input buffering".

Normally when a character is mid-air, the jump button does nothing, in code:

```{src='developing_mechanics/jump_buffering_nobuffer' caption='Code for jumping without buffering'}
```

Jump Buffering consists in allowing the player to "buffer" a jump slightly before the character lands, making the controls a bit less stiff and the gameplay more fluid.

![Example of how jump buffering would work](./images/developing_mechanics/jump_buffering.svg){width=60%}

Jump buffering usually is put into practice using a timer, in a fashion similar to the following:

```{src='developing_mechanics/jump_buffering_buffer' caption='Jump buffering example'}
```

### Coyote Time {#coyote_time}

Coyote time (also known as "edge tolerance") is a technique used to allow a player to jump a few frames after they fall off a platform, allowing for a more fluid gameplay.

![Example of how coyote time would work](./images/developing_mechanics/coyote_time.svg){width=60%}

The trick is starting a countdown as soon as the player leaves a platform without jumping, then if the player presses the jump button while that time is still going, they will perform the jump action, like they still were on a platform.

```{src='developing_mechanics/coyote_time' caption='Coyote time code example'}
```

### Timed Jumps

A way to extend the mobility and challenge of a 2D platformer game is allowing players to jump higher the more the jump button is pressed: this allows the character to perform low and high jumps without much effort, making timing the jump button press a variable that adds to the challenge of a game.

![Example of how timed jumps would work](./images/developing_mechanics/timed_jumps.svg){width=60%}

To work well, timed jumps need to be implemented by tracking the jump button's `onPress` and `onRelease` events. When the jump button has just been pressed, the character's `Y` velocity will be set, as soon as the button is released, such velocity will be capped, shortening the jump height.

```{src='developing_mechanics/timed_jumps' caption='Example code of how timed jumps work'}
```

### Wall Jumps

{{placeholder}}

<!-- TODO: Some games have mechanics involving wall jumps, at those times it may be good to add "sticky walls" to give players more time to make their wall-jump -->

### Screen Wrap

{{placeholder}}

<!-- TODO: Used in puzzle platformers: when you exit a stage right, you re-enter left (or top-bottom), this also means that one of the 2 directions must be locked (or the camera will jolt back and forth). -->
