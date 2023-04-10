### Simulating older consoles' audio

Sometimes we want to give our players a sensation of nostalgia, or we just want to limit ourselves to get the most out of our creativity (creativity comes from limitations), so we may decide to emulate the audio of a famous console (or at least remember of it in some way).

It's not sufficient to "make it sound" the same way (same pitch, same general sound) but we also need to adhere at the limitations of the consoles. Usually such limitations are in the number of channels (which means the number of notes that can played at once by all instruments), but sometimes it's more structural.

:::: note ::::
These is just a simplified version of the limitations of each console. Also if you want you can freely break any of the "rules" and make something original that has the "taste" of something "classic".
::::::::::::::

Let's take a look at some of the most famous.

#### Commodore Vic20

The Commodore Vic20 is one of the first famous home computers; its audio comes from the VIC chip, the same chip that takes care of the video output.

The VIC chip has 3 channels dedicated only to square/pulse waves, with a range of 3 octaves where each octave is a single octave apart from the others (So its octave structure would be something like 1st - 3rd - 5th).

The VIC chip also features a noise channel, for a total of 4 voices. Remember that these voices are shared between music and sound effects.

#### Commodore 64

Probably the most famous home computer of the 8-bit era and had an amazing sound chip for the time: the timeless SID chip, which was used for audio output and controlling paddles/joysticks.

The SID chip (in its two main iterations: the MOS 6581 and the 8580) has 3 channels, each one can be programmed to use one of four wave forms:

- Square/pulse wave
- Triangle
- Sawtooth
- Noise

also the SID chip features ADSR controls for each channel, giving even more control and possibilities.

:::: trivia ::::
The MOS 6581 (the first SID Chip model) had a flaw in its volume register that was very special: if you could change the value of the \$D418 register fast enough, you could play audio samples with a 4-bit resolution, effectively giving the C64 a 4th channel for playing PCM samples!

This issue was fixed in the later MOS 8580 revision, but it can be "added back" by adding a resistor on the board.
::::::::::::::::

But one thing that makes the SID chip very special is the ability to reprogram (and thus change) the instruments on the fly. This real-time programming capability makes it possible to give the "illusion of more instruments"

![A freeze frame of a C64 song, you can see the instruments changing](./images/resources/c64_instrument_change.png){width=60%}

:::: note ::::
You can hear the difference between a somewhat "simple song" like "Monty on the run" and something more complex, like the "R-Type" title theme (check the oscilloscopes). To see how "volume samples" worked, check the oscilloscopes of "Hot Rod" and "Netherworld" (both title themes).
::::::::::::::

#### Commodore Amiga

This is another famous home computer, although some could argue that it was being produced during the fall of Commodore, it is still the cradle of sample-based music and music trackers.

The Commodore Amiga's sound chip, name Paula, had a 4-channel PCM sample-based sound system, where each sample has an 8-bit resolution. Nothing stops people from just mixing more samples together and give the illusion of more channels.

Another limitation of the Paula chip is that 2 channels are strictly dedicated to the "left" stereo channel, while the other 2 are for the "right" stereo channel.

#### Sega Master System / GameGear

The Sega Master System is a quite famous 8-bit console, which had moderate success, and has a lot in common (hardware-wise) with the portable Sega GameGear and those similarities extend to the sound chip too.

The sound chip used is an equivalent of the Texas Instruments SN76489 which features 3 channels dedicated to square/pulse waves + a noise channel.

:::: trivia ::::
To be precise, the ancestor of the Master System, the SEGA SG-1000, used a real TI-SN76489, while the Master System uses a "clone" integrated into its VDP (Video Display Processor).
::::::::::::::::

#### Sega Genesis/MegaDrive

Probably Sega's most famous console: the Genesis/MegaDrive is a bit of a weird beast when it comes to sound. You'll see why.

Mainly the console uses a Yamaha YM2612 chip for sound, which offers 6 programmable FM channels + 1 DAC (digital to analog converter) that can play small samples. The sound chip is technically stereo, but the feature is underused due to the fact that, in the original console, stereo sound could be heard only through the headphone jack.

In addition, mostly for Master System compatibility, the console features a TI-SN76489 chip equivalent (integrated into its VDP), adding 3 square wave channels and a noise channel.

:::: trivia ::::
To underline how important the sound was in this console, just think that sound had its own dedicated fully-fledged CPU! It was a Zylog Z80, the same used as main CPU in the Sinclair ZX Spectrum.
::::::::::::::::

#### NES

Probably the most famous console in the world, the NES had a limited but interesting toolkit for its sound.

The base console had 5 channels, distributed as follows:

- 2 channels dedicated to square waves;
- 1 channel only for triangle waves;
- 1 noise channel;
- 1 channel dedicated to playing small digital sound samples (used normally for drums).

But what's most interesting is that such capabilities could be extended by cartridge hardware, the most famous is probably the Konami VRC6, which added 2 more square wave channels, as well as a sawtooth one, used in Castlevania III.

:::: trivia ::::
Just to underline how extensible this system was, the Konami VRC7 contained a sound chip that provided a 6-channel FM synthesizer. Sadly its extended audio capabilities were used in a single, japan-exclusive game.
::::::::::::::::

#### SNES

The SNES is a huge step forward in time for audio on Nintendo systems, featuring 8 channels that make use of 8-bit samples.

The S-SMP Chip also features a variety of filters and effects, so you have pretty much full freedom except for the number of channels.

#### AdLib / SoundBlaster

The AdLib and SoundBlaster cards are based on the Yamaha YM3812 chip, which features 9 channels that use a digital oscillator. Given the high number of channels and the freedom given by them, it's pretty easy to get a result that sounds like old DOS game as soon as you get the tone down.
