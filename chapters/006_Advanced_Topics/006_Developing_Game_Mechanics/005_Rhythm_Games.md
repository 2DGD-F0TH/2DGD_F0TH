Rhythm Games
------------

### The world of lag

Welcome to the world of rhythm games, as with all new experiences we shall start with... the final boss: Lag.

Lag will be one of the most problematic things you will have to account for: things are not as easy as you may imagine when it comes to implementing a rhythm game. Let's see how to account for it, and eventually how to limit its effect on the player experience.

#### Input Lag

First of all: the ever-present "input lag": there is a certain time window between the moment the use presses a button and the moment the game receives such input. In the middle we find electrons running at breakneck speed through our keyboard circuitry, through the cable, to the motherboard, then the CPU, input abstraction layers in our OS, and finally the input system in our game.

And we didn't reach the game update stage yet.

Also we are not even accounting for the reaction time (about one second) from when the player sees something on screen and when they react.

Input lag is something that we cannot avoid, but there are countermeasures, as we will see below.

#### Video Lag

![Reference image for video lag](./images/developing_mechanics/video_lag.svg){width=50%}

As with the input lag, there is also a not-negligible video lag. The game has to prepare the image, send it to the video card, the card has to render it, apply effects and then send it to the screen, where the liquid crystals (or whatever technology we will have in the future) will have the re-align to create the colored pixels on screen.

#### Audio Lag

![Reference image for audio lag](./images/developing_mechanics/audio_lag.svg){width=50%}

When the audio doesn't exactly match with the video, we talk about "audio lag", this has to be accounted for if you want to have a good rhythm game. In that case, there is a need to compensate for the audio lag, by starting each sound effect (or music) earlier or later by a well-defined amount of milliseconds.

#### Lag Tests

When it comes to lag, it is really difficult to estimate how the computer will react to our game, so we need a metric that will tell us what corrections we need to apply.

Such corrections are estimated comparing video and audio to the input: this way we will keep everything synchronized to the player input, making the game feel tighter.

First kind of test is done "video vs. input", the player has to push a button when something on the screen happens (like pushing rhythmically with a dot changing color), this way we can account for the video lag, compared to the input. This means we will obtain a $(video+input)$ lag measurement.

The second test done is the "audio vs. input" one, the player has to push a button when a sound cue happens on their speakers/headphones (like pushing rhythmically with a beep), this way we can account for the audio lag, compared to the input. This way we will obtain a $(audio+input)$ lag measurement.

By simple math we can account for the "video vs. audio" lag, like follows:

$$ (video + input) - (audio + input)$$

$$ video + input - audio - input$$

$$ video + \bcancel{input} - audio - \bcancel{input}$$

$$ video - audio $$


{{placeholder}}
<!-- TODO: Talk on how to use lag tests to account for video and audio lag, and eventually use some gamification to make them more fun to do -->

### Synchronizing with the Music

{{placeholder}}

<!-- TODO: Basic Beat detection or how to somehow sync music and gameplay -->

#### Time domain vs. Frequency Domain

When we listen to music, we are essentially streaming a bunch of numbers as the time goes forward, so we can plot the amplitude of our waveform against time, as follows:

![Plotting amplitude against time](./images/developing_mechanics/time_domain.png){width=60%}

In this case, when the time is the "independent variable" that we use to base our work, it's said we're working in *time domain*.

When we are working with games, we don't really care about what will happen (music-wise) 5 minutes from now, instead we care about other things that are happening now. In that case, it may be interesting to work in *frequency domain*, which can look something like this:

![Plotting frequency domain](./images/developing_mechanics/frequency_domain.png){width=60%}

We can switch back and forth between the two domains with "transforms", the most used is the Fourier Transform, and one of the most used algorithms to do it on computer is "FFT" (Fast Fourier Transform).

#### The Fast Fourier Transform

{{placeholder}}

<!-- TODO: FFT, to turn time domain into frequency domain -->

#### Beat Detection

{{placeholder}}

<!-- TODO: How to perform beat detection in games -->
