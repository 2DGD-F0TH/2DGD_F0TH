### Some audio processing tips

Sometimes you may find yourself being a bit confused about what to do with your audio samples and music, so here's a small list of tips to make your life that tiny bit easier when it comes to processing audio.

#### Prefer cutting over boosting

Sometimes we may find our audio samples lacking that "punch" they would need, the first idea we may have would be to use a "bass boost" filter to make the low frequencies more prominent. Most of the time, this is not a good idea, since boost filters can create artifacts.

It's better to cut the higher frequencies instead, and eventually boost the entire volume of the sample during mixing. This way the nature of the sample doesn't get tainted by boosting, and we obtain the result we wanted.

#### Spatial sound is not only for 3D Games

If you have a 2D game, that doesn't mean you can't immerse the player further using spatial sound. This works well with top-down games (where you see the character from the top of their head) or games that make use of ray casting to create pseudo-3d environments.

Imagine that you have a source of sound in front of you.

![A source of sound right in front of you sounds the same in both ears](./images/resources/front_headphones.svg){width=30%}

Both your ears are more or less the same distance from the sound source, hence they will hear the sound at the same volume.

But if you are at a different location, like the following example

![A source of sound out of center sounds different in each ear](./images/resources/side_headphones.svg){width=30%}

Each ear would hear the sound with a different volume, with the ear "farthest from the source" hearing the sound at a lower volume.

This is the beginnings of spatial sound in a nutshell.

This can be expanded and made as complex as you want:

- Technically speaking, each ear would hear the sound at a slightly different delay (although considering an average distance between ears of 20cm - less than 10 inches, the delay between each ear would be about 0.0006 seconds);
- The sound gets lowered in volume and muffled if it goes through walls;
- If you're peeking from a corner with only one eye (like you're spying), one ear would be behind a wall, thus hearing less than the other;
- Sounds reflect on surfaces, thus a sound may feel like it comes from another direction than the one where the source is, thus a second "echoed source" should be mixed in.

It can be as simple or as complex as you want, but the more complex something becomes, the more diminishing your returns will probably be.

{{extend}}

<!-- TODO: Give some tips to process audio files -->
