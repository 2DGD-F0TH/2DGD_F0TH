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
