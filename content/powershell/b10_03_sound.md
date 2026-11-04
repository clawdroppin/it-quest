@@lesson b10-sound 🎓 Beyond IT: Making Sound: WAV Files and Speech
@@concepts files, numbers
@@text
> **Beyond IT scope.** Audio synthesis is music technology and signal processing, not IT administration. It's a delightful way to learn about binary file formats, which *do* come up in IT work (forensics, parsers, network captures).

## Digital audio in one paragraph

Sound is air pressure changing over time. A microphone turns it into a voltage; an audio file stores that voltage as numbers: **samples**, taken at a fixed **sample rate** (44,100 per second for CD audio, 8,000 for old telephones). Each sample is a 16-bit signed integer (-32768..32767) in CD quality. A pure tone of frequency f is `sample[n] = A · sin(2π · f · n / rate)`.

## The WAV format

A `.wav` file is a **RIFF** container: a 44-byte header followed by the raw samples, all numbers **little-endian**.

| Offset | Size | Field | Example |
|---|---|---|---|
| 0 | 4 | `RIFF` | ASCII |
| 4 | 4 | file size − 8 | |
| 8 | 4 | `WAVE` | |
| 12 | 4 | `fmt ` (with a space) | |
| 16 | 4 | format chunk size | 16 |
| 20 | 2 | audio format | 1 = PCM |
| 22 | 2 | channels | 1 = mono |
| 24 | 4 | sample rate | 8000 |
| 28 | 4 | byte rate = rate × channels × bytes per sample | 16000 |
| 32 | 2 | block align = channels × bytes per sample | 2 |
| 34 | 2 | bits per sample | 16 |
| 36 | 4 | `data` | |
| 40 | 4 | data size in bytes | |
| 44 | … | samples | |

`System.IO.BinaryWriter` writes integers in little-endian order, which is exactly what WAV needs: `$w.Write([int16]1000)` writes `E8 03`.

## Notes and music

Western music tunes A4 to 440 Hz; each semitone multiplies the frequency by 2^(1/12). MIDI note numbers make this easy: `f = 440 · 2^((note − 69) / 12)`, so middle C (note 60) is about 261.63 Hz. An **envelope** (fade in and out over a few milliseconds) stops the click you'd hear when a tone starts or stops abruptly.

## Speech synthesis

Windows has a built-in text-to-speech engine:

```powershell,norun
Add-Type -AssemblyName System.Speech
$voice = [System.Speech.Synthesis.SpeechSynthesizer]::new()
$voice.GetInstalledVoices().VoiceInfo | Select-Object Name, Culture
$voice.Rate = 0
$voice.Speak('The nightly backup has finished.')
$voice.SetOutputToWaveFile("$env:TEMP\backup.wav"); $voice.Speak('Saved to a file.'); $voice.Dispose()
```

(OK, that one is useful for IT too: a talking alert in the server room.)

## Playing a file

```powershell,norun
[System.Media.SoundPlayer]::new("$env:TEMP\tune.wav").PlaySync()
```

@@quiz
What is a sample rate of 8000 Hz?
* 8,000 amplitude measurements per second of audio
- The highest frequency in the file
- 8,000 bytes per second
- The volume
? The highest representable frequency is half of it (4 kHz, the Nyquist limit).

@@quiz
Why does BinaryWriter suit the WAV format?
- It compresses audio
* It writes integers little-endian, which is the byte order WAV (RIFF) uses
- It writes text
- It plays the sound
? `[int16]1000` becomes E8 03.

@@flashcards
sample :: one amplitude measurement
sample rate :: samples per second
16-bit PCM :: samples from -32768 to 32767
RIFF :: container format of WAV
little-endian :: least significant byte first
`BinaryWriter` :: write binary numbers
A4 :: 440 Hz
semitone :: × 2^(1/12)
MIDI note to Hz :: 440 · 2^((n − 69)/12)
envelope :: fade to avoid clicks
`SpeechSynthesizer` :: Windows text-to-speech

@@exercise Note frequencies
Write `Get-NoteFrequency NAME` that converts note names like `A4`, `C#5` or `Eb3` to frequencies:

- note letters map to semitones from C: C=0, D=2, E=4, F=5, G=7, A=9, B=11; `#` adds 1, `b` subtracts 1
- MIDI number = (octave + 1) × 12 + semitone; frequency = 440 · 2^((midi − 69) / 12)
- print a table of notes with MIDI number and frequency (two decimals, invariant culture), and the frequency ratio of a perfect fifth (C4 → G4, about 1.4983) compared with the "pure" 3/2
@@starter
'TODO'
@@expected
A4    69  440.00 Hz
C4    60  261.63 Hz
C#5   73  554.37 Hz
Eb3   51  155.56 Hz
G4    67  392.00 Hz
A0    21   27.50 Hz
fifth ratio: 1.4983 (pure 3/2 = 1.5000)
@@requires
[math]::Pow
InvariantCulture
@@hint
Parse with a regex: `if ($Name -cmatch '^([A-G])([#b]?)(-?\d)$')`. Use `$Matches[1]` for the letter, `$Matches[2]` for the accidental and `[int]$Matches[3]` for the octave. Return an object with Name, Midi and Hz; format with `$hz.ToString('0.00', $inv).PadLeft(7)`.
@@solution
$inv = [cultureinfo]::InvariantCulture
$semis = @{ C = 0; D = 2; E = 4; F = 5; G = 7; A = 9; B = 11 }

function Get-NoteFrequency([string]$Name) {
    if ($Name -cnotmatch '^([A-G])([#b]?)(-?\d)$') { throw "bad note $Name" }
    $semi = $semis[$Matches[1]]
    if ($Matches[2] -eq '#') { $semi++ } elseif ($Matches[2] -eq 'b') { $semi-- }
    $midi = ([int]$Matches[3] + 1) * 12 + $semi
    [pscustomobject]@{ Name = $Name; Midi = $midi; Hz = 440 * [math]::Pow(2, ($midi - 69) / 12) }
}

foreach ($n in 'A4', 'C4', 'C#5', 'Eb3', 'G4', 'A0') {
    $f = Get-NoteFrequency $n
    $f.Name.PadRight(5) + $f.Midi.ToString().PadLeft(3) + ' ' + $f.Hz.ToString('0.00', $inv).PadLeft(7) + ' Hz'
}
$ratio = (Get-NoteFrequency 'G4').Hz / (Get-NoteFrequency 'C4').Hz
"fifth ratio: $($ratio.ToString('0.0000', $inv)) (pure 3/2 = $((1.5).ToString('0.0000', $inv)))"

@@exercise Write a WAV file by hand
Synthesise a short tune and write the WAV file byte by byte with `BinaryWriter`:

- mono, 16-bit, 8000 Hz; the notes `C4 E4 G4 C5`, each 0.25 s, amplitude 8000, with a 10 ms linear fade-in and fade-out
- write the 44-byte header (see the table) then the samples
- read the file back and print: the four-character IDs, the header numbers (format, channels, rate, byte rate, block align, bits, data size), the total file size, and the peak sample value
- also count the **zero crossings** of the first note (sign changes between consecutive samples), which should be about 2 × frequency × duration
@@starter
$path = Join-Path ([System.IO.Path]::GetTempPath()) "tune-$PID.wav"
$notes = @(261.63, 329.63, 392.00, 523.25)
'TODO'
@@expected
ids: RIFF WAVE fmt  data
format 1, channels 1, rate 8000, byte rate 16000, block align 2, bits 16
data size: 16000 bytes, file size: 16044 bytes
peak: 8000
zero crossings in note 1: 126 (expected about 131)
@@requires
BinaryWriter
[int16]
RIFF
BitConverter
@@hint
Each note has `n = 2000` samples. Sample `i`: `$env = [math]::Min(1, [math]::Min($i, $n - 1 - $i) / 80)` (80 samples = 10 ms), value `[int16][math]::Round(8000 * $env * [math]::Sin(2 * [math]::PI * $f * $i / 8000))`. Write strings with `$w.Write([System.Text.Encoding]::ASCII.GetBytes('RIFF'))`. Read back with `$b = [System.IO.File]::ReadAllBytes($path)`: `[System.Text.Encoding]::ASCII.GetString($b, 0, 4)`, `[BitConverter]::ToInt16($b, 20)`, `[BitConverter]::ToInt32($b, 24)`. Peak: loop over the samples with `ToInt16($b, 44 + 2 * $k)`. Zero crossings: count where the sign of sample k differs from k−1, ignoring exact zeros.
@@solution
$path = Join-Path ([System.IO.Path]::GetTempPath()) "tune-$PID.wav"
$notes = @(261.63, 329.63, 392.00, 523.25)

$rate = 8000
$perNote = [int]($rate * 0.25)
$fade = [int]($rate * 0.010)
$samples = [System.Collections.Generic.List[int16]]::new()
foreach ($f in $notes) {
    for ($i = 0; $i -lt $perNote; $i++) {
        $env = [math]::Min(1, [math]::Min($i, $perNote - 1 - $i) / $fade)
        $samples.Add([int16][math]::Round(8000 * $env * [math]::Sin(2 * [math]::PI * $f * $i / $rate)))
    }
}

$dataSize = $samples.Count * 2
$ascii = [System.Text.Encoding]::ASCII
$stream = [System.IO.File]::Create($path)
$w = [System.IO.BinaryWriter]::new($stream)
try {
    $w.Write($ascii.GetBytes('RIFF')); $w.Write([int32](36 + $dataSize)); $w.Write($ascii.GetBytes('WAVE'))
    $w.Write($ascii.GetBytes('fmt ')); $w.Write([int32]16)
    $w.Write([int16]1); $w.Write([int16]1); $w.Write([int32]$rate)
    $w.Write([int32]($rate * 2)); $w.Write([int16]2); $w.Write([int16]16)
    $w.Write($ascii.GetBytes('data')); $w.Write([int32]$dataSize)
    foreach ($s in $samples) { $w.Write($s) }
} finally {
    $w.Dispose()
}

$b = [System.IO.File]::ReadAllBytes($path)
"ids: $($ascii.GetString($b, 0, 4)) $($ascii.GetString($b, 8, 4)) $($ascii.GetString($b, 12, 4)) $($ascii.GetString($b, 36, 4))"
"format $([BitConverter]::ToInt16($b, 20)), channels $([BitConverter]::ToInt16($b, 22)), rate $([BitConverter]::ToInt32($b, 24)), byte rate $([BitConverter]::ToInt32($b, 28)), block align $([BitConverter]::ToInt16($b, 32)), bits $([BitConverter]::ToInt16($b, 34))"
"data size: $([BitConverter]::ToInt32($b, 40)) bytes, file size: $($b.Length) bytes"

$peak = 0
$crossings = 0
$prev = 0
$count = [BitConverter]::ToInt32($b, 40) / 2
for ($k = 0; $k -lt $count; $k++) {
    $v = [BitConverter]::ToInt16($b, 44 + 2 * $k)
    if ([math]::Abs($v) -gt $peak) { $peak = [math]::Abs($v) }
    if ($k -lt $perNote -and $v -ne 0) {
        if ($prev -ne 0 -and [math]::Sign($v) -ne [math]::Sign($prev)) { $crossings++ }
        $prev = $v
    }
}
"peak: $peak"
"zero crossings in note 1: $crossings (expected about $([int](2 * $notes[0] * 0.25)))"
Remove-Item $path
