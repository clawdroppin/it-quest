@@lesson dr5-fairlight Fairlight: Audio Editing, Mixing and Loudness
@@concepts numbers, lists, functions
@@lang python
@@text
Viewers forgive soft focus but not bad sound. **Fairlight** is a full digital audio workstation inside Resolve: multitrack recording and editing, a mixer with EQ and dynamics on every channel, buses, automation, sound libraries, ADR tools and loudness metering for broadcast and streaming.

## The Fairlight page

- **Timeline** with audio tracks (mono, stereo, 5.1, 7.1, Atmos), waveforms, clip gain lines and fades.
- **Mixer**: a channel strip per track: input, **EQ**, **Dynamics**, effects inserts, pan, buses, the fader.
- **Meters** and the **Loudness** meter (Control Room).
- **Sound Library**, **ADR** panel, **Foley sampler**, **Fairlight FX**: reverb, de-esser, de-hummer, noise reduction, delay, chorus, pitch, modulation, limiter, Voice Isolation (Studio, AI dialogue separation), **Dialogue Leveler**.

## Editing audio

- **Clip gain** (drag the line on a clip, or the Inspector) vs **track fader**: set clip gain first so dialogue is even, then mix with faders.
- **Fades** with the handles at clip corners; **crossfades** between clips avoid clicks; Ctrl+Click a clip edit for a crossfade.
- **Range selection** (R) and **cut/copy/paste** across tracks; **Elastic wave** (stretch audio to fit).
- **Room tone** fills gaps in dialogue; an empty silence sounds unnatural.
- **Normalize** (right-click → Normalize Audio Levels) to a sample peak or a loudness target.
- **Sync** dialogue with picture (waveform sync on the Media page) and use **J/L cuts** from the Edit page.

## Organising a mix

Typical tracks and buses:

| Tracks | Bus |
|---|---|
| Dialogue (DX) A1–A3 | Dialogue bus |
| Music (MX) | Music bus |
| Effects (FX) and ambience | Effects bus |
| Voice-over | Dialogue bus |
| All buses | **Main** (master) output |

Grouping into **stems** (dialogue, music, effects: "DME") lets you make different versions (an international version without dialogue) and is a common delivery requirement.

## Decibels

Audio levels use **decibels (dB)**: a logarithmic scale.

- Amplitude ratio to dB: `dB = 20 × log10(ratio)`. Doubling amplitude = **+6 dB**; halving = −6 dB.
- Power ratio: `10 × log10(ratio)`; doubling power = +3 dB.
- **dBFS** (decibels full scale): 0 dBFS is the digital maximum; everything is negative. Going over **clips** (distortion). Peaks around −6 to −3 dBFS for dialogue, never over **−1 dBTP** (true peak) for delivery.
- Two identical sources added together: +6 dB (correlated); two uncorrelated noises: +3 dB.

The first exercise does these conversions and mixes gains.

## EQ and dynamics

- **EQ**: high-pass dialogue at 80–100 Hz (removes rumble); a gentle cut around 200–400 Hz removes boxiness; a boost around 2–5 kHz improves clarity; a de-esser tames 5–8 kHz sibilance. Cut music where the voice lives (a "pocket" around 1–4 kHz) instead of turning it way down.
- **Compressor**: reduces the level above a **threshold** by a **ratio** (4:1 means 4 dB over the threshold comes out as 1 dB over). **Attack/release** set how fast; **make-up gain** brings the level back up. Makes dialogue even and present.
- **Limiter**: a very high ratio compressor at the end of the chain to stop peaks.
- **Gate/expander**: reduce noise between words.
- **Ducking** (sidechain): music lowers automatically when dialogue plays: Dynamics → compressor with the dialogue bus as the sidechain source.

The second exercise implements a compressor's gain computer and a ducking curve.

## Automation

Animate any mixer parameter over time: the **Automation** toolbar, modes **Write**, **Touch** (writes while you touch a control, returns to previous values), **Latch** (keeps the last value). Draw volume changes with keyframes on the clip or track.

## Loudness

Streaming services and broadcasters normalise to a **loudness** target measured in **LUFS** (loudness units relative to full scale, ITU-R BS.1770 with K-weighting and gating). Typical targets:

| Platform | Integrated loudness | True peak |
|---|---|---|
| Broadcast EBU R128 (Europe) | −23 LUFS | −1 dBTP |
| US broadcast ATSC A/85 | −24 LKFS | −2 dBTP |
| YouTube, Spotify, most streaming | about −14 LUFS | −1 dBTP |
| Apple Podcasts / Music | about −16 LUFS | −1 dBTP |
| Netflix | −27 LKFS dialogue-gated | −2 dBTP |

Use the **Loudness meter** (set to your standard in Project Settings → Fairlight), play the whole program, and adjust the main bus so the **integrated** value hits the target, with a limiter for true peaks. **Loudness range (LRA)** describes dynamics; keep dialogue intelligible across scenes.

## Immersive audio

Fairlight supports 5.1/7.1 and **Dolby Atmos** (bed + objects, binaural monitoring on headphones), with **3D panners** and a renderer: increasingly common for streaming deliveries.

@@quiz
What does doubling an audio signal's amplitude do in decibels?
- +3 dB
* +6 dB
- +10 dB
- +20 dB
? 20 × log10(2) ≈ 6.02 dB. Power doubling is +3 dB.

@@quiz
What is 0 dBFS?
* The digital maximum: anything above clips
- Silence
- The loudness target for YouTube
- The level of room tone
? Levels in digital audio are below 0 dBFS.

@@quiz
A compressor has a threshold of −20 dB and a ratio of 4:1. The input is −8 dB. What is the output (no make-up gain)?
- −8 dB
- −20 dB
* −17 dB
- −12 dB
? 12 dB over the threshold becomes 3 dB over: −20 + 3 = −17.

@@quiz
What is ducking?
- Removing ducks from field recordings
* Automatically lowering music when dialogue plays, using a sidechain compressor
- Panning audio left
- Normalising peaks
? The dialogue bus feeds the compressor's sidechain on the music bus.

@@quiz
What is a typical integrated loudness target for YouTube and Spotify?
- −23 LUFS
* About −14 LUFS
- 0 LUFS
- −40 LUFS
? EBU R128 broadcast uses −23 LUFS.

@@flashcards
Fairlight :: Resolve's audio workstation page
clip gain vs fader :: per-clip level vs track mix level
crossfade :: overlapping fades to avoid clicks
room tone :: recorded silence of a location
stems (DME) :: dialogue, music, effects mixes
dB (amplitude) :: 20 × log10(ratio); ×2 = +6 dB
dB (power) :: 10 × log10(ratio); ×2 = +3 dB
dBFS :: decibels relative to digital full scale
dBTP :: true peak level (between samples)
high-pass filter :: removes low rumble below a cutoff
compressor :: threshold, ratio, attack, release, make-up gain
limiter :: stops peaks above a ceiling
gate :: silences signal below a threshold
ducking (sidechain) :: music dips under dialogue
automation Write / Touch / Latch :: recording mixer moves
LUFS / LKFS :: loudness units (BS.1770)
EBU R128 :: −23 LUFS broadcast standard
integrated loudness :: average loudness of the whole programme
Dolby Atmos :: bed plus object-based immersive audio

@@exercise Decibels and a mix
Write `ratio_to_db(r)` (amplitude ratio → dB, rounded to 2; return `float("-inf")` for 0) and `db_to_ratio(db)` (rounded to 4). Write `apply_gain(samples, db)`: multiply each sample by the ratio, rounded to 4. Write `peak_dbfs(samples)`: the peak level in dBFS (rounded to 2). Then `mix(tracks, gains_db)`: sum the tracks sample by sample after applying each track's gain, and return `(mixed_samples, clipped)` where `clipped` is True if any absolute mixed sample exceeds 1.0 (samples rounded to 4).
@@starter
import math


def ratio_to_db(r):
    pass


def db_to_ratio(db):
    pass


def apply_gain(samples, db):
    pass


def peak_dbfs(samples):
    pass


def mix(tracks, gains_db):
    pass


dialogue = [0.5, -0.4, 0.6, -0.5]
music = [0.6, 0.6, -0.6, 0.6]
print(peak_dbfs(dialogue), mix([dialogue, music], [0, -12]))
@@tests
check(ratio_to_db(2), 6.02, "double the amplitude")
check(ratio_to_db(0.5), -6.02, "half")
check(ratio_to_db(1), 0.0, "unity")
check(ratio_to_db(0), float("-inf"), "silence")
check(db_to_ratio(-20), 0.1, "−20 dB is a tenth")
check(db_to_ratio(6), 1.9953, "+6 dB")
check(apply_gain([0.5, -0.25], -6), [0.2506, -0.1253], "apply gain")
check(peak_dbfs([0.5, -0.4, 0.6, -0.5]), -4.44, "peak")
check(peak_dbfs([1.0, -1.0]), 0.0, "full scale")
dialogue = [0.5, -0.4, 0.6, -0.5]
music = [0.6, 0.6, -0.6, 0.6]
check(mix([dialogue, music], [0, -12]), ([0.6507, -0.2493, 0.4493, -0.3493], False), "music 12 dB under")
check(mix([dialogue, music], [0, 0])[1], True, "too loud: clipped")
@@hint
`20 * math.log10(r)` and `10 ** (db / 20)`. Peak is the maximum absolute sample. In `mix`, use `zip(*scaled_tracks)` to walk the samples together.
@@solution
import math


def ratio_to_db(r):
    if r == 0:
        return float("-inf")
    return round(20 * math.log10(r), 2)


def db_to_ratio(db):
    return round(10 ** (db / 20), 4)


def apply_gain(samples, db):
    g = 10 ** (db / 20)
    return [round(s * g, 4) for s in samples]


def peak_dbfs(samples):
    return ratio_to_db(max(abs(s) for s in samples))


def mix(tracks, gains_db):
    scaled = [[s * 10 ** (g / 20) for s in t] for t, g in zip(tracks, gains_db)]
    mixed = [sum(values) for values in zip(*scaled)]
    return [round(s, 4) for s in mixed], any(abs(s) > 1.0 for s in mixed)


dialogue = [0.5, -0.4, 0.6, -0.5]
music = [0.6, 0.6, -0.6, 0.6]
print(peak_dbfs(dialogue), mix([dialogue, music], [0, -12]))

@@exercise A compressor and ducking
Write `compress(level_db, threshold, ratio, makeup=0)`: below the threshold the level passes unchanged; above it, the amount over is divided by the ratio; then add makeup gain; round to 2. Write `gain_reduction(level_db, threshold, ratio)`: how many dB the compressor removed (positive number, rounded to 2). Then `duck(dialogue_levels, music_level, threshold, depth)`: for each dialogue level (dB) in a list, the music level becomes `music_level - depth` when dialogue is above `threshold`, otherwise `music_level`. Finally `smooth_duck(levels, step)`: limit how fast the music level can change: each value can move at most `step` dB from the previous output (the first value passes as is), like attack and release.
@@starter
def compress(level_db, threshold, ratio, makeup=0):
    pass


def gain_reduction(level_db, threshold, ratio):
    pass


def duck(dialogue_levels, music_level, threshold, depth):
    pass


def smooth_duck(levels, step):
    pass


dlg = [-60, -60, -18, -16, -20, -60, -60]
raw = duck(dlg, -20, -40, 12)
print(raw, smooth_duck(raw, 4))
@@tests
check(compress(-30, -20, 4), -30, "below threshold: unchanged")
check(compress(-8, -20, 4), -17.0, "12 dB over becomes 3 dB over")
check(compress(-8, -20, 4, makeup=5), -12.0, "with make-up gain")
check(compress(0, -10, 1), 0, "ratio 1 does nothing")
check(gain_reduction(-8, -20, 4), 9.0, "9 dB of gain reduction")
check(gain_reduction(-30, -20, 4), 0.0, "no reduction below threshold")
dlg = [-60, -60, -18, -16, -20, -60, -60]
raw = duck(dlg, -20, -40, 12)
check(raw, [-20, -20, -32, -32, -32, -20, -20], "music dips under dialogue")
check(smooth_duck(raw, 4), [-20, -20, -24, -28, -32, -28, -24], "smoothed with 4 dB steps")
check(smooth_duck([0, 1, 2], 10), [0, 1, 2], "small changes pass")
@@hint
`over = level_db - threshold`; if `over <= 0` return the level (plus makeup). For smoothing, `delta = target - prev`, clamp it to `[-step, step]`.
@@solution
def compress(level_db, threshold, ratio, makeup=0):
    over = level_db - threshold
    if over <= 0:
        return level_db + makeup
    return round(threshold + over / ratio + makeup, 2)


def gain_reduction(level_db, threshold, ratio):
    return round(level_db - compress(level_db, threshold, ratio), 2) + 0.0


def duck(dialogue_levels, music_level, threshold, depth):
    return [music_level - depth if d > threshold else music_level for d in dialogue_levels]


def smooth_duck(levels, step):
    out = []
    for target in levels:
        if not out:
            out.append(target)
            continue
        prev = out[-1]
        delta = max(-step, min(step, target - prev))
        out.append(prev + delta)
    return out


dlg = [-60, -60, -18, -16, -20, -60, -60]
raw = duck(dlg, -20, -40, 12)
print(raw, smooth_duck(raw, 4))
