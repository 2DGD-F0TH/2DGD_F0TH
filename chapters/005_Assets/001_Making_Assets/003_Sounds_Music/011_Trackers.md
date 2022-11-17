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
