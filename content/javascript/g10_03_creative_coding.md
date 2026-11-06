@@lesson g10-creative 🎓 Beyond IT: Generative Art and Sound with JavaScript
@@concepts numbers, functions
@@text
> **Beyond IT scope.** Creative coding (generative art, data art, music and sound design in code) is an artistic field. It's here because JavaScript in the browser is one of its main tools, and it exercises maths and randomness in a playful way. Skip it freely if your goal is an IT job.

**Creative coding** means writing programs whose output is art: images, animations, music, installations. Popular tools are **p5.js** (a beginner-friendly drawing library based on Processing), plain Canvas/SVG, three.js for 3D, and the **Web Audio API** or **Tone.js** for sound.

## Randomness you can control

`Math.random()` can't be seeded, so you can't reproduce a picture you liked. Generative artists use a **seeded pseudo-random number generator** (PRNG): the same seed always produces the same sequence.

```javascript
function mulberry32(seed) {
  return function () {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}
const rand = mulberry32(42);
console.log([rand(), rand(), rand()].map((x) => x.toFixed(4)));
console.log(mulberry32(42)().toFixed(4), "(same seed, same first value)");
```

(Seeded PRNGs like this are for art, games and tests, never for security; use `crypto` for that.)

## Noise: natural-looking randomness

Pure random values jump around; nature changes smoothly. **Value noise** and **Perlin/simplex noise** give random-looking values that vary **smoothly** with position, used for terrain, clouds, wobbly lines and flow fields. The idea of 1D value noise: put random values at whole numbers and **interpolate smoothly** between them. Adding several **octaves** (higher frequency, lower amplitude) gives detail ("fractal noise").

## Generative patterns

```javascript,norun
// p5.js: a flow field of particles
function setup() { createCanvas(800, 600); background(10); noiseSeed(7); }
function draw() {
  for (let i = 0; i < 50; i++) {
    const x = random(width), y = random(height);
    const angle = noise(x * 0.005, y * 0.005) * TAU * 2;
    stroke(255, 20);
    line(x, y, x + cos(angle) * 20, y + sin(angle) * 20);
  }
}
```

Other classic techniques: recursion (fractal trees), L-systems (plant growth rules), cellular automata (Game of Life), circle packing, Voronoi diagrams and polar coordinates for mandalas.

## Sound with the Web Audio API

The Web Audio API is a graph of audio nodes: **sources** (oscillators, samples, microphone) → **effects** (gain, filters, delay, reverb) → **destination** (speakers).

```javascript,norun
const ctx = new AudioContext();               // must be created/resumed after a user gesture
function beep(freq, start, duration) {
  const osc = ctx.createOscillator();
  const gain = ctx.createGain();
  osc.type = "triangle";
  osc.frequency.value = freq;
  gain.gain.setValueAtTime(0.0001, start);
  gain.gain.exponentialRampToValueAtTime(0.3, start + 0.01);            // quick attack
  gain.gain.exponentialRampToValueAtTime(0.0001, start + duration);     // smooth release: no clicks
  osc.connect(gain).connect(ctx.destination);
  osc.start(start);
  osc.stop(start + duration);
}
[60, 64, 67, 72].forEach((note, i) => beep(440 * 2 ** ((note - 69) / 12), ctx.currentTime + i * 0.25, 0.4));
```

Music theory in one formula: in equal temperament, each semitone multiplies the frequency by 2^(1/12), and MIDI note 69 is A4 = 440 Hz. So `freq = 440 × 2^((note − 69) / 12)`. Scales are patterns of semitone steps: major = 2, 2, 1, 2, 2, 2, 1.

An `AnalyserNode` gives you frequency data for **audio visualisers**; schedule notes ahead with `ctx.currentTime` for precise timing (never `setTimeout`).

## Sharing your work

Sites like OpenProcessing, CodePen and fxhash host creative code; `canvas.toBlob()` exports PNGs; `MediaRecorder` records canvas animations to video.

@@quiz
Why do generative artists use seeded random number generators?
* The same seed reproduces the exact same artwork, so good results can be saved and regenerated
- They're more random than Math.random
- They're secure
- Browsers require them
? Math.random can't be seeded.

@@quiz
What's the difference between random values and noise?
- Noise is louder
* Noise varies smoothly with position, so nearby inputs give similar outputs
- Random values are always integers
- There is no difference
? Smoothness makes noise look natural.

@@quiz
What frequency is one octave above A4 (440 Hz)?
- 450 Hz
- 660 Hz
* 880 Hz
- 220 Hz
? An octave doubles the frequency: 12 semitones × 2^(1/12) each.

@@drill
Which browser API generates and plays sound from code?
= Web Audio API
= Web Audio
= AudioContext
? new AudioContext() creates oscillators and effects.

@@flashcards
Creative coding :: programming as an artistic medium
p5.js :: beginner-friendly creative coding library
Seeded PRNG :: reproducible pseudo-random sequence
Value / Perlin noise :: smooth randomness
Octaves (noise) :: layers of finer detail added together
Flow field :: particles moving along noise-driven angles
Web Audio API :: graph of audio nodes in the browser
Oscillator :: node producing a periodic waveform
Equal temperament :: each semitone is ×2^(1/12)
MIDI note 69 :: A4 = 440 Hz

@@exercise Smooth value noise
Write `createNoise(seed)` returning `noise(x)` for 1D value noise:

- random values at integer positions come from `mulberry32` seeded with `seed + i × 7919` (one fresh generator per integer, take its first value), so `noise(i)` equals that value for integer `i` (including negative integers)
- between integers, interpolate with the **smoothstep** curve: `t = x − floor(x)`, `s = t × t × (3 − 2t)`, result `a + (b − a) × s`

Then `fractal(noise, x, octaves = 4)`: sum `noise(x × 2^k) × 0.5^k` for k = 0..octaves−1, divided by the sum of the amplitudes (so the result stays in 0..1).
@@starter
function mulberry32(seed) {
  return function () {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function createNoise(seed) {
  throw new Error("TODO: not done yet");
}

function fractal(noise, x, octaves = 4) {
  throw new Error("TODO: not done yet");
}

@@tests
const n = createNoise(1);
const at = (i) => mulberry32(1 + i * 7919)();
check([n(0) === at(0), n(3) === at(3), n(-2) === at(-2)], [true, true, true], "integer points");
check(Math.abs(n(2.5) - (at(2) + at(3)) / 2) < 1e-12, true, "midpoint is the average");
const samples = Array.from({ length: 1000 }, (_, i) => n(i / 100));
check(samples.every((v) => v >= 0 && v < 1), true, "range 0..1");
const maxJump = Math.max(...samples.slice(1).map((v, i) => Math.abs(v - samples[i])));
check(maxJump < 0.03, true, "smooth: small steps between nearby samples");
check(createNoise(1)(4.2) === n(4.2) && createNoise(2)(4.2) !== n(4.2), true, "seeded");
const f = fractal(n, 1.37, 4);
check(f >= 0 && f < 1, true, "fractal stays in range");
check(fractal(n, 1.37, 1) === n(1.37), true, "one octave is plain noise");
@@hint
`const valueAt = (i) => mulberry32(seed + i * 7919)();` and `const i0 = Math.floor(x); const t = x - i0; const s = t * t * (3 - 2 * t); return valueAt(i0) + (valueAt(i0 + 1) - valueAt(i0)) * s;`.
@@solution
function mulberry32(seed) {
  return function () {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function createNoise(seed) {
  const valueAt = (i) => mulberry32(seed + i * 7919)();
  return function noise(x) {
    const i0 = Math.floor(x);
    const t = x - i0;
    const s = t * t * (3 - 2 * t);
    const a = valueAt(i0);
    const b = valueAt(i0 + 1);
    return a + (b - a) * s;
  };
}

function fractal(noise, x, octaves = 4) {
  let total = 0;
  let amplitudes = 0;
  for (let k = 0; k < octaves; k++) {
    const amp = 0.5 ** k;
    total += noise(x * 2 ** k) * amp;
    amplitudes += amp;
  }
  return total / amplitudes;
}

@@exercise Notes, scales and chords
Write:

- `noteToFreq(note)`: MIDI note number → frequency in Hz, rounded to 2 decimals (`440 × 2^((note − 69) / 12)`)
- `parseNote(name)`: names like `"C4"`, `"F#3"`, `"Bb5"`, `"A-1"` → MIDI number, with `C4 = 60` (note letters C D E F G A B are 0 2 4 5 7 9 11 semitones above C; `#` adds 1, `b` subtracts 1; octave can be negative). Throw `Error("bad note <name>")` for anything else.
- `scale(root, pattern)`: MIDI numbers for one octave starting at `root` using a step pattern, e.g. major `[2, 2, 1, 2, 2, 2, 1]` → 8 notes including the octave
- `chord(root, type)`: `"major"` = root + 0, 4, 7; `"minor"` = 0, 3, 7; `"dim"` = 0, 3, 6; `"7"` = 0, 4, 7, 10. Unknown types throw `Error("unknown chord <type>")`.
@@starter
function noteToFreq(note) {
  throw new Error("TODO: not done yet");
}

function parseNote(name) {
  throw new Error("TODO: not done yet");
}

function scale(root, pattern) {
  throw new Error("TODO: not done yet");
}

function chord(root, type) {
  throw new Error("TODO: not done yet");
}

@@tests
check([noteToFreq(69), noteToFreq(81), noteToFreq(60)], [440, 880, 261.63], "frequencies");
check(["C4", "A4", "F#3", "Bb5", "C-1", "B#3"].map(parseNote), [60, 69, 54, 82, 0, 60], "note names");
const bad = ["H4", "C", "C#", "4C", "Cx4"].map((n) => { try { parseNote(n); return "parsed"; } catch (e) { return e.message; } });
check(bad, ["bad note H4", "bad note C", "bad note C#", "bad note 4C", "bad note Cx4"], "bad names");
check(scale(60, [2, 2, 1, 2, 2, 2, 1]), [60, 62, 64, 65, 67, 69, 71, 72], "C major");
check(chord(parseNote("A3"), "minor").map(noteToFreq), [220, 261.63, 329.63], "A minor chord");
let msg = "";
try { chord(60, "sus9"); } catch (e) { msg = e.message; }
check(msg, "unknown chord sus9", "unknown chord");
@@hint
Parse with `/^([A-G])([#b]?)(-?\d+)$/`. MIDI = `(octave + 1) × 12 + letter offset + accidental`. Scales: start with `[root]` and add each step to the previous note.
@@solution
const LETTERS = { C: 0, D: 2, E: 4, F: 5, G: 7, A: 9, B: 11 };
const CHORDS = { major: [0, 4, 7], minor: [0, 3, 7], dim: [0, 3, 6], 7: [0, 4, 7, 10] };

function noteToFreq(note) {
  return Math.round(440 * 2 ** ((note - 69) / 12) * 100) / 100;
}

function parseNote(name) {
  const m = /^([A-G])([#b]?)(-?\d+)$/.exec(name);
  if (!m) throw new Error(`bad note ${name}`);
  const accidental = m[2] === "#" ? 1 : m[2] === "b" ? -1 : 0;
  return (Number(m[3]) + 1) * 12 + LETTERS[m[1]] + accidental;
}

function scale(root, pattern) {
  const notes = [root];
  for (const step of pattern) notes.push(notes.at(-1) + step);
  return notes;
}

function chord(root, type) {
  if (!Object.hasOwn(CHORDS, type)) throw new Error(`unknown chord ${type}`);
  return CHORDS[type].map((i) => root + i);
}
