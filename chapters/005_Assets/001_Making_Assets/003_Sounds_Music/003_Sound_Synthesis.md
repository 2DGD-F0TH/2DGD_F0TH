### Sound Synthesis

Now we're entering technical territory. We're going to talk about sound synthesis: the art of creating sounds, also called "sound synthesis".

#### AM Synthesis

The first, and technically simplest way to generate sound is via AM (amplitude modulation) synthesis.

With this technique you take the wave form created by an *[oscillator~\[g\]~](#gl_oscillator)* and modulate its amplitude (volume) according to a second wave form.

![Example of AM Synthesis](./images/resources/AM_Synthesis.png){width=50%}

In this example we see a 440Hz sine wave (in the middle) having its amplitude (quite heavily) modulated by a 110Hz sine wave (on top): the resulting wave form on the bottom has a "tremolo effect" to it.

#### FM Synthesis

With this technique you take the wave form created by an *[oscillator~\[g\]~](#gl_oscillator)* (called "carrier frequency") but instead of modulating its amplitude, you modulate its frequency (pitch) according to another wave (called "modulator frequency").

![Example of FM Synthesis](./images/resources/FM_Synthesis.png){width=50%}

In this example (for sake of visibility) we have a 110Hz sine wave (in the middle) having its frequency (again, heavily) modulated by a 22Hz sine wave (on top): we can see the result in the bottom of the figure.

This frequency modulation happens so fast that we end up with something that sounds completely different from the original wave form.

#### FM Synthesis vs Sample-based music

FM synthesis was invented to circumvent one of the biggest issues that plagued the 8 and 16-bit era: lack of space.

Games were saved on small cartridges (which ranged, on average, from 4kb on the Vic20 to 2MB on the Genesis/MegaDrive), such cartridges had to contain graphics, music and the entire code of the game.

Back then there were no compressed music files, not that it would have helped much, so instead of memorizing the song itself, the instructions to play the song would be saved.

Let's make a simple example: we need to make a very simplistic soundtrack, composed of a pure sine wave that represents the "A above middle C" (or $A_4$) for an indefinite amount of time. We could either save all the samples (and thus waste precious cartridge space), or we could just save the following "code":

```{caption="How FM music may be saved on an old console"}
stop all channels;
select the sinewave waveform on channel 1;
set the frequency of channel 1 at 440Hz;
start channel 1;
```

All music made via FM synthesis is nothing more than a bunch of instructions for the FM chip on how to work.

The next step forward was on the Commodore Amiga, where the first sample-based music started: we save small pieces of PCM audio (called samples), rework them a bit using ADSR envelope and pitch-shifting and call them "instruments". Such instruments are then used to compose the track.

The music heard from these systems is fruit of a "hybrid approach": small pieces of (sometimes recorded) audio actually exist in the track, and they're re-used, pitch shifted and reworked all around the track. This makes for very small files (around 10 to 100kb) with a lot of flexibility.

Modern music is essentially made up of a huge, monolithic sample, usually in the form of an MP3 file or something equivalent, recorded from real-life instruments or synthetized, but instead of "saving the instructions" like in the old days, we just save the entire track as a PCM sample.
