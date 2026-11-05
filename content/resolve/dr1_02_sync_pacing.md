@@lesson dr1-sync-pacing Syncing, Multicam and Pacing an Edit
@@lang python
@@text
Two jobs come up on almost every project: lining up picture with separately recorded sound (or several cameras with each other), and making the cut feel right. Resolve automates the first through audio waveform sync; the second is craft, but measurable craft. This lesson explains how sync works under the hood, how multicam editing flows, and how editors think about rhythm, shot length and cutting to music.

## Syncing sound and cameras

- **Timecode sync**: if all devices were jam-synced to the same timecode (Tentacle Sync, Deity TC-1), select clips and *Auto Sync Audio → Based on Timecode*. Exact and fast.
- **Waveform sync**: *Auto Sync Audio → Based on Waveform* compares each camera's scratch audio with the recorder's audio and finds the offset with the best match: in effect a **cross-correlation**, sliding one signal over the other and scoring the overlap.
- A **slate/clap** gives a sharp spike that's easy to align by eye.
- Sound travels about **343 m/s**: a camera 10 m away hears a clap about 29 ms (almost a frame at 30 fps) late. Sync to the close microphone.
- Long takes can **drift** if recorders' clocks differ slightly; check sync at the end of the take too.

## Multicam

1. Select the angles in the Media Pool → *Create New Multicam Clip Using Selected Clips*, syncing by timecode, sound, in points or markers.
2. Edit the multicam clip into a timeline; open the **Multicam viewer** (on the Edit page, set the viewer to Multicam).
3. Play and click angles (or press number keys) to cut live; Alt-click switches video only, keeping the chosen audio.
4. Refine cuts with the Roll tool, or *Flatten Multicam Clip* to get ordinary clips.

## Pacing and rhythm

- **Average shot length (ASL)**: total duration / number of shots. Fast action trailers run around 1–2 s; dialogue drama 4–8 s; slow cinema much longer. Measure your cut and compare with the reference you're imitating.
- Vary shot lengths: a constant length feels mechanical. Hold on reactions; trim the entrances and exits of movement ("come in late, leave early").
- **J-cuts** (audio leads the picture) and **L-cuts** (audio continues over the next shot) make dialogue flow.
- **Cutting to music**: at a tempo of B beats per minute, one beat = 60 / B seconds. At 24 fps and 120 BPM a beat is exactly 12 frames. Cut on the beat (or deliberately just before it), and change sections at bar boundaries (usually every 4 beats). Place markers on beats (M while playing) and snap to them.

@@quiz
How does waveform-based sync find the offset between two recordings?
* It slides one signal over the other and picks the offset with the best match
- It reads the timecode
- It compares file names
- It uses the clip colour
? Cross-correlation.

@@quiz
At 120 BPM and 24 fps, how many frames is one beat?
- 6
* 12
- 24
- 30
? 0.5 s × 24.

@@quiz
A 3-minute cut has 90 shots. What's the average shot length?
- 0.5 s
* 2 s
- 3 s
- 90 s
? 180 / 90.

@@quiz
What's an L-cut?
- Picture leads the audio
* The audio of a shot continues over the next shot
- A cut on the beat
- A multicam angle change
? J-cut is the reverse.

@@quiz
Why sync to the close microphone rather than a distant camera's audio?
- It's louder
* Sound takes time to travel; distant mics hear events later
- Cameras record in mono
- Timecode is only on recorders
? About 3 ms per metre.

@@flashcards
timecode sync :: jam-synced devices
waveform sync :: cross-correlate audio
speed of sound :: about 343 m/s
drift :: clocks diverge over long takes
multicam clip :: synced angles in one clip
Alt-click angle :: switch video only
flatten multicam :: back to ordinary clips
ASL :: total duration / shots
J-cut :: audio leads picture
L-cut :: audio trails into next shot
beat length :: 60 / BPM seconds
bar :: usually 4 beats
come in late, leave early :: trim scene edges

@@drill
How many seconds long is one beat at 90 BPM? (two decimals)
= 0.67
= .67
= 0.667
? 60 / 90.

@@drill
What is the cut called where the next shot's audio starts before its picture?
= J-cut
= j cut
= jcut
= J cut
? The audio leads like the letter J.

@@exercise Sync two recordings by cross-correlation
`recorder` is the clean audio envelope and each camera has a scratch track of the same event starting at an unknown offset (with noise). For each camera, try every lag in a range, score the overlap with a normalised dot product, and report the best lag in samples, in milliseconds and in frames at 25 fps. Also estimate how far each camera was from the clap if all of its offset came from sound travel time.
@@starter
import math
import random

RATE = 1000  # envelope samples per second
random.seed(4)
event = [0] * 50 + [1.0, 0.8, 0.5, 0.3, 0.1] + [0] * 30 + [0.6, 0.9, 0.4, 0.2] + [0] * 60 + [0.7, 0.3] + [0] * 40
recorder = event + [0] * 200

def delayed(signal, lag, noise):
    return ([0] * lag + signal)[:len(signal)] if lag >= 0 else signal[-lag:] + [0] * -lag

cameras = {name: [v + random.uniform(-noise, noise) for v in delayed(recorder, lag, noise)]
           for name, lag, noise in (("cam A (close)", 3, 0.05), ("cam B (wide)", 29, 0.1), ("cam C (late start)", 118, 0.08))}

def score(a, b, lag):
    raise NotImplementedError
@@expected
cam A (close): offset 3 samples = 3 ms = 0.07 frames at 25 fps (score 0.957); if all sound travel: 1.0 m
cam B (wide): offset 29 samples = 29 ms = 0.72 frames at 25 fps (score 0.876); if all sound travel: 9.9 m
cam C (late start): offset 118 samples = 118 ms = 2.95 frames at 25 fps (score 0.938); if all sound travel: 40.5 m
@@requires
def score
math.sqrt(
343
@@hint
For lag L compare `a[i]` with `b[i + L]` wherever both indexes are valid; score = Σab / sqrt(Σa² · Σb²). Try lags from 0 to 199 and keep the maximum. Distance = lag seconds × 343 m/s.
@@solution
import math
import random

RATE = 1000  # envelope samples per second
random.seed(4)
event = [0] * 50 + [1.0, 0.8, 0.5, 0.3, 0.1] + [0] * 30 + [0.6, 0.9, 0.4, 0.2] + [0] * 60 + [0.7, 0.3] + [0] * 40
recorder = event + [0] * 200

def delayed(signal, lag, noise):
    return ([0] * lag + signal)[:len(signal)] if lag >= 0 else signal[-lag:] + [0] * -lag

cameras = {name: [v + random.uniform(-noise, noise) for v in delayed(recorder, lag, noise)]
           for name, lag, noise in (("cam A (close)", 3, 0.05), ("cam B (wide)", 29, 0.1), ("cam C (late start)", 118, 0.08))}

def score(a, b, lag):
    pairs = [(a[i], b[i + lag]) for i in range(len(a)) if 0 <= i + lag < len(b)]
    dot = sum(x * y for x, y in pairs)
    norm = math.sqrt(sum(x * x for x, _ in pairs) * sum(y * y for _, y in pairs))
    return dot / norm if norm else 0.0

for name, track in cameras.items():
    best = max(range(200), key=lambda lag: score(recorder, track, lag))
    ms = best / RATE * 1000
    print(f"{name}: offset {best} samples = {ms:.0f} ms = {ms / 40:.2f} frames at 25 fps "
          f"(score {score(recorder, track, best):.3f}); if all sound travel: {best / RATE * 343:.1f} m")

@@exercise Measure pacing and cut to the beat
From a list of shot durations in frames, compute the ASL, the shortest and longest shots, and a histogram in 1-second buckets. Then, given the music tempo, snap each cut point to the nearest beat and report how many frames each cut moved.
@@starter
FPS = 24
BPM = 100
shots = [58, 31, 46, 22, 70, 35, 18, 52, 28, 96, 40, 26]

def snap(frame, beat_frames):
    raise NotImplementedError
@@expected
12 shots, 21.75 s, ASL 1.81 s, shortest 0.75 s, longest 4.00 s
  0-1 s: ##
  1-2 s: ######
  2-3 s: ###
  3-4 s: 
  4-5 s: #
beat = 14.4 frames at 100 BPM
  cut at  58 ->  58 (+0)
  cut at  89 ->  86 (-3)
  cut at 135 -> 130 (-5)
  cut at 157 -> 158 (+1)
  cut at 227 -> 230 (+3)
  cut at 262 -> 259 (-3)
  cut at 280 -> 274 (-6)
  cut at 332 -> 331 (-1)
  cut at 360 -> 360 (+0)
  cut at 456 -> 461 (+5)
  cut at 496 -> 490 (-6)
largest move: -6 frames
@@requires
def snap
60 / BPM
round(
@@hint
Cut points are the running totals of the durations (excluding the final end). `beat_frames = 60 / BPM * FPS`; snapping is `round(frame / beat_frames) * beat_frames`, rounded to a whole frame. Bucket = `duration // FPS`.
@@solution
from collections import Counter
from itertools import accumulate

FPS = 24
BPM = 100
shots = [58, 31, 46, 22, 70, 35, 18, 52, 28, 96, 40, 26]

def snap(frame, beat_frames):
    return round(round(frame / beat_frames) * beat_frames)

total = sum(shots)
print(f"{len(shots)} shots, {total / FPS:.2f} s, ASL {total / len(shots) / FPS:.2f} s, "
      f"shortest {min(shots) / FPS:.2f} s, longest {max(shots) / FPS:.2f} s")
buckets = Counter(d // FPS for d in shots)
for second in range(max(buckets) + 1):
    print(f"  {second}-{second + 1} s: {'#' * buckets[second]}")
beat = 60 / BPM * FPS
print(f"beat = {beat:.1f} frames at {BPM} BPM")
moves = []
for cut in list(accumulate(shots))[:-1]:
    target = snap(cut, beat)
    moves.append(target - cut)
    print(f"  cut at {cut:>3} -> {target:>3} ({target - cut:+d})")
print("largest move:", max(moves, key=abs), "frames")
