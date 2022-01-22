Sounds And Music
----------------

{{placeholder}}

<!-- TODO: Talk about the importance of good sound quality, introduce people to chip tunes and tools to create music and sounds , talk about the importance of having areas where you have to apply a certain effect or it will happen on the whole level -->


### Some audio basics

Before creating sounds and music, we need to clarify some terminology, as well as learn some basics before diving into FM synthesis like wave forms. After that we can learn about trackers and Software DAWs.

In this section we will learn about sample rate, bit depth, lossy/lossless formats and clipping, among other things.

#### Sample Rate

Differently from Analog Audio, which is continuous (as in has an infinite amount of detail), Digital Audio is a stream of numbers (ones and zeros) that is "discrete" in nature. That means that we blast these numbers thousands of times a second to be able to build a decent sounding sound.

The number of times we record such numbers from our digital microphone (as well as the number of times we blast such numbers back from our speakers) is called **sample rate** and it is measured in *Hz*.

The more the samples per second, the more detail we can squeeze into our audio files, but at the same time the bigger the file will become too.

![Graphical Representation of Sample Rate (44.1KHz)](./images/resources/Sample_Rate_44100.png){width=60%}

In normal CD-Audio, we have a sample rate of 44100 Hz, which means that we recorded a sample 44100 times in a single second.

When making our game's audio, we should always stay around such value, since going lower would make the audio sound worse, since we lower the amount of information the audio itself has.

![Graphical Representation of Sample Rate (8KHz)](./images/resources/Sample_Rate_8000.png){width=60%}

Also we should avoid using weird sample rates, 44.1KHz (or 44100 Hz if you prefer) is a "magic value" that guarantees the most compatibility.

#### Bit Depth

Along with sample rate, there is another value in audio that expresses its quality: the bit depth.

The bit depth defines how many "volume values" we can have in a single sample, which shapes the quality of the sound in a different way than sample rate.

If our audio has a 1-bit bit depth, each sample will have only 2 values:

- **0:** Mute
- **1:** Blast at full volume

Which strongly reduces the quality of the audio.

::: trivia :::
In Pokèmon Yellow for GameBoy, Pikachu's voice was encoded with in 1-bit depth.
::::::::::::::

If we had a 2-bit depth, we could make each sample have more volume values:

- 00: Mute
- 01: 33% volume
- 10: 66% volume
- 11: 100% volume

Usually audio has a 16 Bit depth, but more modern systems make use of 24 Bits or even 32 Bits.

#### Lossless Formats

As with graphics, there are audio formats that allow you to store uncompressed information, as well as compressed (but without losses) sounds. The most common lossless audio formats include:

- WAV (uncompressed);
- AIFF (usually uncompressed, but AIFF-C supports both lossless and lossy compression);
- FLAC (lossless compression).

#### Lossy Formats

As with graphics, there are also "lossy formats" that allow us to store information in even less space by getting rid of information that is considered outside our hearing spectrum, for instance. Some of the most known are:

- Mpeg Layer 3 (MP3);
- OGG Vorbis;
- Windows Media Audio (WMA);
- Advanced Audio Codec (AAC).

#### Clipping

Clipping is a phenomenon when you're trying to output (or record) a wave that exceeds the capacity of your equipment. As a result, the wave is "clipped" (cut) and there is a very audible distortion.

You surely heard clipping in audio before, usually when people scream on a low-quality microphone and the audio gets distorted.

The best way to repair clipping is to re-record the audio completely, although some tools can help in case you absolutely cannot re-record the audio.

Also you should be wary of clipping, because there may be cases where it damages your audio equipment.

### Sound Synthesis

Now we're entering technical territory. We're going to talk about sound synthesis: the art of creating sounds, also called "sound synthesis".

### AM Synthesis

The first, and technically simplest way to generate sound is via AM (amplitude modulation) synthesis.

With this technique you take the wave form created by an *oscillator*~[g]~ and modulate its amplitude (volume) according to a second wave form.

![Example of AM Synthesis](./images/resources/AM_Synthesis.png){width=50%}

In this example we see a 440Hz sine wave (in the middle) having its amplitude (quite heavily) modulated by a 110Hz sine wave (on top): the resulting wave form on the bottom has a "tremolo effect" to it.

### FM Synthesis

With this technique you take the wave form created by an *oscillator*~[g]~ (called "carrier frequency") but instead of modulating its amplitude, you modulate its frequency (pitch) according to another wave (called "modulator frequency").

![Example of FM Synthesis](./images/resources/FM_Synthesis.png){width=50%}

In this example (for sake of visibility) we have a 110Hz sine wave (in the middle) having its frequency (again, heavily) modulated by a 22Hz sine wave (on top): we can see the result in the bottom of the figure.

This frequency modulation happens so fast that we end up with something that sounds completely different from the original wave form.

### Basic Wave Forms

It is important to know how the main wave forms look and sound in order to understand how to create your own instruments, as well as having a further insight on how older 8-bit games sounded.

It is suggested to look for each sine wave on the internet and hear how it sounds, here we will briefly talk about the main waveforms, their shape and uses.

#### Sine Wave

A sine wave has an amplitude that follows a trigonometric sine wave, it sounds really "pure" and is usually used at 440Hz to tune instruments in A.

![How a sine wave looks](./images/resources/waveforms/sine.png){width=60%}

In games it is usually used to give out the impression of a flute-like instrument.

#### Square Wave

A square wave looks... square-like. It is one of the most used waves in 8-bit music, sounds a bit rougher than a sine wave and is used for beeps and blips.

![How a square wave looks](./images/resources/waveforms/square.png){width=60%}

In game music it is normally used as lead instrument, and with various modulations, it can sound like a xylophone or a piano, or at least a very artificial rendition of those.

The NES had 2 voices (or channels) dedicated to square waves.

#### Triangle Wave

A triangle wave is another very used wave in 8-bit music, given it's very "muted" characteristics it can be used to give songs a "bass track" of some sort.

![How a triangle wave looks](./images/resources/waveforms/triangle.png){width=60%}

The NES had one channel completely dedicated to triangle waveforms.

#### Sawtooth Wave

Sawtooth waves were a staple of the Commodore64 era, with its "gritty", "bzzt" sound which can sound a lot like a trombone on long notes.

![How a sawtooth wave looks](./images/resources/waveforms/sawtooth.png){width=60%}

#### Noise

Noise is not a real, static waveform, but more like what comes out when the amplitude has a random value on each sample.

![How a noise wave looks](./images/resources/waveforms/noise.png){width=60%}

Noise can be used (with the right modulation and processing) to simulate percussions.

The NES had one channel completely dedicated to noise waveforms.

### ADSR Envelope

Now that we've seen how waveforms look like, we need to understand how such sounds change over time. The sound envelope is used to describe exactly that.

The most common way to control the signal envelope is through four parameters: Attack, Decay, Sustain and Release. Thus the name "ADSR envelope".

![Representation of an ADSR Envelope](./images/resources/adsr_1.svg){width=50%}

#### Attack

The attack is the measure of time the sound takes from zero to its initial peak, when the key is pressed.

![Attack on ADSR Envelope](./images/resources/adsr_2.svg){width=50%}

The longer the attack, the slower the sound will "rise" when a key is pressed.

#### Decay

After the attack, comes the decay, which is the measure of time it takes the sound to drop to "sustain level" after the initial peak.

![Decay on ADSR Envelope](./images/resources/adsr_3.svg){width=50%}

The longer the decay, the slower the sound will drop to sustain level.

#### Sustain

After the decay is completed, we are now sustaining the signal. Sustain is *not* a measure of time, but **it is a measure of volume**.

![Sustain on ADSR Envelope](./images/resources/adsr_4.svg){width=50%}

The higher the sustain level, the louder the signal will be when at "sustain level". This signal will last until we release the key.

#### Release

After we release the key, the sound will have to fade out somehow. Release is the measure of time it takes the sound to go from sustain level back to zero.

![Release on ADSR Envelope](./images/resources/adsr_5.svg){width=50%}

The higher the release time, the longer the sound will take to "fade out".

### Digital Sound Processing (DSP)

Let's think about a simple situation: we want to play a "walk" sound effect: every time our character's foot hits the ground, we play a "step" sound effect.

If we play the same sound over and over, it will become boring really quickly, breaking the immersion. An idea could be saving different sounds for a footstep and then every time the player takes a step, a random footstep sound will be played.

This solves the problem, at a cost: we need to use more memory to keep track of such sounds, multiply that for tens of sound effects and the game may run out of memory on low-end systems (or we can "run out" of patience in creating tens of variants of sound effects).

An alternative solution could be using DSP: editing the sound sample in real time to add more variety and depth while saving memory, the trade-off would be CPU time, but it's an acceptable deal.

#### Reverb

When you take a stroll on a sidewalk, you have a certain "openness" on the footstep sounds you hear, but that surely changes if you're walking with hard shoes on a hard floor inside of a small cave. You can hear a lot of reverb and echo at every single step.

Reverb is the first of the sound effects that we encounter in our journey: it allows to give more depth to our sound effects, making it sound like we're inside of a small cave or a very large room.

#### Pitch Shift

A way to give more variety to a sound effect without much work is using pitch shift to make our sound a bit higher or lower, randomly, so that each step is slightly different from the other: this way our ears will get less tired of hearing said sound effect.

Pitch shifting must be used with caution, since abusing it will distort the sound effect and break the immersion in our game.

Another example of pitch shift is used in racing games, where the car roar is pitch-shifted up or down according to the acceleration given to the car.

#### Filtering

Another sound effect we can use is filtering, which are divided in 3 main sections, according to the frequency they "allow to pass through":

- **Low Pass Filter:** This filter allows low frequencies to pass through unfiltered, while the frequencies higher than a defined threshold will be cut. This allows for effects where the bass is unaltered but higher frequencies are cut away;
- **High Pass Filter:** Opposite of the previous filter, this filter allows high frequencies to pass through unaltered while the frequencies lower than a defined threshold will be cut;
- **Band Pass Filter:** A combination of the two previous filters, this filter let's through all the frequencies between two defined threshold values. This allows for more interesting effects like a music sounding through an old radio.

An interesting example is when an explosion happens near the player, in that case the "stun" effect is given by using a low pass filter on an explosion sound (which makes it sound really low and muffled), eventually a slowdown is applied and a secondary sound effect of a very high pitch sound is added (something similar to what you hear when your ears are ringing).

#### Doppler Effect

To give more depth to your sound effects, you can use pitch shift to create the "Doppler Effect" that we hear in real life when a police car passes by: when the car approaches us the pitch is higher, when the car is in front of us we hear the siren as it should be, and when the car passes us we hear a lower pitched version of the siren sound effect.

The Doppler effect can be really useful when applied to car racing games again, when we overtake one of our opponents (or one of our opponents overtakes us) using the doppler effect can help the player feel more "immersed" in the experience.

The doppler effect would actually apply to light too, but we would need to have something travelling at a really high speed or said object to be really far away (like a planet).

### FM Synthesis vs Sample-based music

{{placeholder}}

<!-- TODO: Explain the difference between FM Synthesis and modern sample-based music -->

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

The SID chip (in its two main iterations: the MOS 6581 and the 8580) has 3 channels, each one can be reprogrammed in real time to use one of four wave forms:

- Square/pulse wave
- Triangle
- Sawtooth
- Noise

This real-time programming capability makes it easier to give the "illusion of more instruments", also the SID chip features ADSR controls for each channel, giving more possibilities.

:::: trivia ::::
The MOS 6581 (the first SID Chip model) had a flaw in its volume register that was very special: if you could change the value of the \$D418 register fast enough, you could play audio samples with a 4-bit resolution, effectively giving the C64 a 4th channel for playing PCM samples!

This issue was fixed in the later MOS 8580 revision, but it can be "added back" by adding a resistor on the board.
::::::::::::::::

#### Commodore Amiga

This is another famous home computer, although some could argue that it was being produced during the fall of Commodore, it is still the cradle of sample-based music and music trackers.

The Commodore Amiga's sound chip, name Paula, had a 4-channel PCM sample-based sound system, where each sample has an 8-bit resolution. Nothing stops people from just mixing more samples together and give the illusion of more channels.

Another limitation of the Paula chip is that 2 channels are strictly dedicated to the "left" stereo channel, while the other 2 are for the "right" stereo channel.

#### Sega Master System / GameGear

The Sega Master System is a quite famous 8-bit console, which had moderate success, and has a lot in common (hardware-wise) with the portable Sega GameGear and those similarities extend to the sound chip too.

The sound chip used is an equivalent of the Texas Instruments SN76489 which features 3 channels dedicated to square/pulse waves + a noise channel.

:::: trivia ::::
The ancestor of the Master System, the SEGA SG-1000 used a real TI-SN76489, while the Master System uses a "clone" integrated into its VDP (Video Display Processor).
::::::::::::::::

#### Sega Genesis/MegaDrive

Probably Sega's most famous console: the Genesis/MegaDrive is a bit of a weird beast when it comes to sound. You'll see why.

Mainly the console uses a Yamaha YM2612 chip for sound, which offers 6 programmable FM channels + 1 DAC (digital to analog converter) that can play small samples. The sound chip is technically stereo, but the feature is underused due to the fact that in the original console stereo sound could be heard only through the headphone jack.

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
Just to underline how extensible this system was, the Konami VRC7 contained a sound chip that provided a 6-channel FM sythetizer. Sadly its extended audio capabilities were used in a single, japan-exclusive game.
::::::::::::::::

#### SNES

The SNES is a huge step forward in time for audio on Nintendo systems, featuring 8 channels that make use of 8-bit samples.

The S-SMP Chip also features a variety of filters and effects, so you have pretty much full freedom except for the number of channels.

#### AdLib / SoundBlaster

The AdLib and SoundBlaster cards are based on the Yamaha YM3812 chip, which features 9 channels that use a digital oscillator. Given the high number of channels and the freedom given by them, it's pretty easy to get a result that sounds like old DOS game as soon as you get the tone down.

### "Swappable" sound effects

Back to our walking example, an idea to increase the variety of sound effects at our disposal would be keeping a list of "swappable sounds": sounds that are still part of the class we're considering, but are radically different.

For instance we could have different walking sounds for different floors, so that walking on grass and walking on a stone pavement will be different. In this case it would be useful to make the sounds configurable and give the sound manager the chance to inspect what type of floor we're walking on.

An example of "swappable sound effects" configuration is given in the following file, which is written in YAML:

```{.yaml caption="Example of swappable sound effects"}
footsteps:
  grass:
    - grasswalk1.wav
    - grasswalk2.wav
  stone:
    - stonewalk1.wav
    - stonewalk2.wav
  metal:
    - metalstep1.wav
    - metalstep2.wav
```

Making a configuration file instead of hard-coding the elements allows for easy extensibility and modding, which everyone loves (See [Designing entities as data](#entitiesasdata)).

### Some audio processing tips

{{placeholder}}

<!-- TODO: Give some tips to process audio files -->

#### Prefer cutting over boosting

Sometimes we may find our audio samples lacking that "punch" they would need, the first idea we may have would be to use a "bass boost" filter to make the low frequencies more prominent. Most of the time, this is not a good idea, since boost filters can create artifacts.

It's better to cut the higher frequencies instead, and eventually boost the entire volume of the sample during mixing. This way the nature of the sample doesn't get tainted by boosting, and we obtain the result we wanted.

### DAW Basics

#### What is a DAW Software?

Digital Audio Workstation Software (DAW Software) are pieces of software that have extensive recording, playback and editing features, allowing you to create your own songs, given some instrument samples (or pre-recorded tracks).

They also feature mixing facilities, waveform display and track controls, some even feature (the software equivalent of) effect racks, such as equalizers, to further the possibilities of creating your work in the best way possible.

#### The Piano Roll

Have you ever seen one of those pianos that seem to have an integrated music box? That is a so-called "piano roll" and is used to store music and play it back on the piano.

Most Software DAW have a similar abstraction, called "piano roll" too.

![Example of a piano roll in LMMS](./images/resources/piano_roll_1.png){width=60%}

In the previous image, we can see four notes, each defined by its vertical position (respectively a C5, C6, A5, F#5) and length (respectively a single whole/semibreve and three half/minim notes). In the previous example each vertical green bar represents $\frac{1}{4}$ and each light green bar represents a beat. This should help you imagining how to compose something.

Different systems have different piano roll variations. For instance:

![Example of a piano roll in FamiStudio](./images/resources/piano_roll_2.png){width=60%}

This Software DAW has a different shape for the notes, meaning a certain kind of effect is applied to them. All the basic principles still apply, though.

The piano roll abstraction allows you to edit your music easily, by grabbing notes and moving them, or making them longer by dragging their edges.

### Music Tracker Basics

#### What is a Music Tracker Software?

Born with the 4-voice sampling system in the Commodore Amiga, Music Trackers are essentially a type of music sequencer. The great majority of music trackers have the majority of their screen occupied by the tracker version of a music sheet.

![A screen from MilkyTracker](./images/resources/tracker_overview.png){width=60%}

Differently from how the great majority Software DAW work, notes are positioned in channels at certain points of a timeline that spans vertically.

Usually each channel contains 64 rows (16 beats), but it can be changed to the composer's preference.

![Simple overview of a tracker](./images/resources/tracker_guide_1.png){width=60%}

Each row of a channel contains instructions for the tracker to execute, in the form of notes, instruments and commands (or effects).

![How each tracker channel is divided](./images/resources/tracker_guide_2.png){width=60%}

Notes are written in the usual "Letter Notation" that you see in many music environments, while instruments are enumerated, then there are commands: commands can instruct the tracker to apply a temporary effect on the note, like portamento or vibrato.

In the previous example, there is a "vibrato" command going on, starting with the `48C`: `4` is the "vibrato command", `8` is the vibrato speed and `C` (hex for "12") is the vibrato depth.

The vibrato continues with a `V0 E41` command pair, where `V0` changes the vibrato depth to `0` (the speed is the same defined in the previous command), while `E41` is a "vibrato control command" (`E4`) which changes the waveform of the vibrato to `1` which is "ramp down".

The next command does more or less the same thing, besides changing the vibrato waveform to "square".

The `E40` command resets the vibrato waveform to the default "sine wave".

{{placeholder}}

<!-- TODO: Finish talking about music trackers -->

#### Samples

Samples are the basis of a music tracker: they are essentially wave forms which can be sped up or slowed down to create different notes. Without any sample, you wouldn't have any instrument, which in turn would mean you'd have no sound at all.

Usually samples come in the form of small digital sound files, most trackers allow the sample to be looped (wholly or in part) to simulate a "sustain" effect.

{{placeholder}}

<!-- TODO: Finish talking about sound samples -->

#### Instruments

An instrument is a set of a sound sample, with some effects applied by default (if you want). Essentially an instrument is a "container" for a sample and some parameters to allow the change of pitch and effects.

{{placeholder}}

<!-- TODO: Finish talking about tracker instrument -->

#### Channels

A "channel" (also called a "voice"), is a space where one sample is played back at a time. One channel is not "fixed" to a certain instrument and modern music trackers can mix an unlimited number of channels. Many times music makers limit themselves to a certain number of channels to achieve a "retro feeling" or to challenge themselves.

{{placeholder}}

<!-- TODO: Finish talking about channels in trackers -->

#### Patterns

A pattern is essentially a piece of a song: a group of tracks with their own instruments, settings and notes written in them. The "pattern" abstraction allows you to easily repeat pieces of a song by just referring to the pattern.

{{placeholder}}

<!-- TODO: Finish talking about patterns in trackers -->

### Basic Rhythms

When composing music, we may not know where to start: this is the objective of this section: give you some easy rhythms to start with. Here I will use LMMS's beat+bassline editor to represent the notes, as it's the easiest to understand.

Remember to check your tempo too, since it may make the difference between something akin to the "house" genre and something more "techno".

#### Four on the floor

This is the most basic rhythm there is: let's consider a situation where we have 16 beats per bar (so each note is 1/16th):

![A single bar in our basic rhythm](./images/resources/sixteen_beats.png){width=50%}

The four on the floor rhythm uses a kick drum in the 1st, 5th, 9th and 13th beat, giving a constant rhythm.

![A basic four on the floor rhythm](./images/resources/four_on_the_floor.png){width=50%}

This is practically the basic of all dance-based music.

#### Four on the floor with off-beat hi-hats

A basic rhythm like the "four on the floor", by itself, can prove to be quite boring. To spice it up, we can add some closed hi-hats in the off-beats, like the following:

![Four on the floor with off-beat hi-hats](./images/resources/fotf_offbeat.png){width=50%}

If you listen closely, a lot of music has this basic pattern mixed into it, in one way or another.

#### A simple rock beat

By using hi-hats, a kick drum and a snare drum, you can create a very simple rock beat. Keep 4 beats in a bar, each beat put a hi-hat, on 1 and 2 put a kick drum and on 3 a snare drum.

![A simple rock beat](./images/resources/simple_rock_beat.png){width=50%}

{{placeholder}}

<!-- TODO: Teach more basic rhythms to get people going -->


#### Adaptive Music

Adaptive Music (sometimes called "dynamic music" or "interactive music") is a background music track that reacts to the events of the game. Such reactions can involve volume, rhythm, tune, adding new instruments (adding drums, for instance).

{{placeholder}}
