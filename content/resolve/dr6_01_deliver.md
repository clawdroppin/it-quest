@@lesson dr6-deliver Deliver: Codecs, Formats and Export Settings
@@concepts numbers, maps, functions, strings
@@lang python
@@text
The **Deliver** page turns your timeline into files: a master for archiving, a version for YouTube, a vertical cut for social media, a ProRes file for a client, a DCP for a cinema. Understanding codecs and bitrates is what makes those files look right and stay a sensible size.

## The Deliver page

- **Render Settings** (left): presets (YouTube, Vimeo, TikTok, Twitter/X, H.264 Master, H.265 Master, ProRes, IMF, Dropbox/Frame.io upload...) or **Custom Export**.
- **Single clip** (one file for the whole timeline) or **Individual clips** (one per clip, for VFX pulls or handing shots to a colourist).
- **Render range**: entire timeline or In/Out.
- **Add to Render Queue**, then **Render All**. Queue several timelines and formats overnight; jobs can include **remote rendering** on another machine with Studio.

Custom export tabs:

| Video | Audio | File |
|---|---|---|
| format (MP4, MOV, MXF, EXR...), codec, encoder (hardware NVIDIA/AMD/Intel/Apple or software), resolution, frame rate, quality / bitrate, profile, key frames, data levels, colour space tag, **Use optimised media / render cache** | codec (AAC, Linear PCM), sample rate (48 kHz for video), bit depth, channels / stems as separate tracks | filename template, subfolders, use unique filenames |

## Codecs: delivery vs intermediate

| Codec | Type | Use |
|---|---|---|
| **H.264 (AVC)** | long-GOP, lossy, 8-bit typical | universal web delivery |
| **H.265 (HEVC)** | long-GOP, ~40% smaller than H.264 at similar quality, 10-bit HDR | 4K/HDR delivery, phones |
| **AV1** | modern open codec, efficient | YouTube, web (hardware encoders on recent GPUs) |
| **Apple ProRes** (422 Proxy/LT/422/HQ, 4444, 4444 XQ, RAW) | intra-frame, high quality | editing, masters, client hand-off (alpha with 4444) |
| **Avid DNxHR / DNxHD** (LB, SQ, HQ, HQX, 444) | intra-frame | the cross-platform ProRes alternative, MXF or MOV |
| **GoPro CineForm** | intra-frame wavelet | intermediate with alpha |
| **OpenEXR / DPX / TIFF** | image sequences | VFX plates and renders, archival |
| **JPEG 2000 (DCP)** | intra-frame | cinema projection |

**Long-GOP** codecs store full frames only occasionally (I-frames, every GOP) and the rest as differences (P/B-frames): small files, heavy to decode and edit. **Intra-frame** codecs compress each frame independently: bigger files, smooth editing, best for masters.

## Bitrate and file size

File size ≈ **bitrate × duration**. Bitrates are in **megabits** per second (Mb/s), and sizes in megabytes (MB): divide bits by 8.

- 10 minutes of 1080p H.264 at 16 Mb/s: 16 × 600 / 8 = 1,200 MB ≈ 1.2 GB.
- Rough targets for upload masters: 1080p 24–30 fps H.264 at 10–20 Mb/s; 4K at 40–80 Mb/s (H.265 roughly half). YouTube re-encodes everything, so upload generously.
- ProRes 422 HQ at 1080p 29.97 is about 220 Mb/s (≈ 100 GB/hour).

**Uncompressed** data rate = width × height × bits per pixel × fps. With **chroma subsampling**, colour is stored at lower resolution than brightness:

| Sampling | Samples per pixel (relative to luma) | Example |
|---|---|---|
| 4:4:4 | 3 (full colour) | ProRes 4444, graphics, keying |
| 4:2:2 | 2 | ProRes 422, broadcast |
| 4:2:0 | 1.5 | H.264/H.265, most cameras |

At 10-bit 4:2:2, each pixel takes 2 × 10 = 20 bits. The first exercise computes data rates and sizes.

## Preset recipes

- **YouTube 4K SDR**: MP4, H.264 or H.265, 3840 × 2160, timeline frame rate, quality "Restrict to" 60–80 Mb/s, AAC 320 kb/s, Rec.709 Gamma 2.4 tag (or Rec.709-A on Macs to avoid gamma shifts).
- **Vertical social (9:16)**: a separate 1080 × 1920 timeline (Smart Reframe on the Cut page, Studio), H.264 at 15–20 Mb/s; burn in captions (Subtitles track → export burned in).
- **Client review**: H.264 1080p at 8–10 Mb/s with a timecode burn-in (Workspace → Data Burn-In) and a watermark.
- **Master/archive**: ProRes 422 HQ or DNxHR HQX in MOV/MXF, Linear PCM audio 48 kHz 24-bit, separate stems.
- **Image sequences for VFX**: Individual clips, EXR with handles (e.g. 12 frames) and the source file name in the name.
- **Transparent graphics**: QuickTime ProRes 4444 with "Export Alpha", or PNG sequence.
- **Cinema**: Kakadu/DCP (Studio) in XYZ colour space at 24 fps; IMF for streaming platforms.

## Subtitles and captions

Add a **subtitle track** (Edit page, right-click track header → Add Subtitle Track), type or import `.srt`, or **auto-transcribe** (Studio: Timeline → Create Subtitles from Audio). Export as a sidecar `.srt`/`.vtt`, embedded, or burned in. The second exercise writes `.srt` timestamps (`HH:MM:SS,mmm`).

## Checking the output

Watch the whole render at least once (or spot check every edit point), on the target device. Check loudness, audio sync at the end of the file (drift means a frame rate mismatch), colour and gamma against the Resolve viewer, black frames, and that captions appear in time. Keep the project archive (.dra) alongside the master.

@@quiz
Which codec type is best for smooth editing and masters?
- Long-GOP (H.264)
* Intra-frame (ProRes, DNxHR)
- AV1
- GIF
? Each frame is independent, so scrubbing and cutting are light on the CPU.

@@quiz
How big is a 10-minute file at 16 Mb/s?
- 160 MB
* About 1.2 GB
- 16 GB
- 9.6 GB
? 16 × 600 = 9,600 megabits; ÷ 8 = 1,200 MB.

@@quiz
What does 4:2:0 chroma subsampling mean?
* Colour is stored at a quarter of the luma resolution (half horizontally and half vertically)
- Four audio channels
- 4K resolution at 20 fps
- Every fourth frame is a keyframe
? 4:2:2 halves colour horizontally only; 4:4:4 keeps full colour.

@@quiz
How do you export graphics with transparency from Resolve?
- H.264 MP4
* QuickTime ProRes 4444 (or PNG/EXR sequence) with Export Alpha enabled
- MP3
- Any format; alpha is automatic
? H.264 has no alpha channel.

@@quiz
The audio drifts out of sync towards the end of an export. What's the likely cause?
- The bitrate is too high
* A frame rate mismatch (for example 29.97 footage on a 30 fps timeline or a wrong export rate)
- The subtitles
- Using AAC audio
? Check clip attributes, timeline and export frame rates.

@@drill
Which video codec is the common choice for web delivery?
= H.264
= h264
= avc
= H.265
= hevc
? ProRes and DNxHR are intermediate codecs.

@@flashcards
Deliver page :: render queue and export settings
single clip vs individual clips :: one file vs one per clip
H.264 / H.265 / AV1 :: delivery codecs, increasingly efficient
ProRes / DNxHR :: intra-frame intermediate and master codecs
ProRes 4444 :: high-quality with alpha
long-GOP :: I-frames plus difference frames
intra-frame :: each frame compressed independently
bitrate :: data per second, e.g. Mb/s
size (MB) :: bitrate (Mb/s) × seconds ÷ 8
4:4:4 / 4:2:2 / 4:2:0 :: chroma subsampling levels
bit depth :: 8-bit = 256 levels, 10-bit = 1024 per channel
48 kHz :: standard video audio sample rate
Data Burn-In :: timecode and watermark overlays
.srt :: subtitle file with HH:MM:SS,mmm timestamps
DCP / IMF :: cinema package / streaming master package
Render Queue :: batch several jobs and Render All

@@exercise Data rates and file sizes
Write `size_mb(bitrate_mbps, seconds)` (rounded to 1). Write `bitrate_for(size_mb, seconds)`: the bitrate needed to hit a target size (rounded to 2). Write `uncompressed_mbps(width, height, bit_depth, sampling, fps)`: `sampling` is `"4:4:4"`, `"4:2:2"` or `"4:2:0"` (3, 2 or 1.5 samples per pixel); return megabits per second (10⁶ bits) rounded to 1. Then `compression_ratio(uncompressed, actual)` rounded to 1, and `fits(bitrate_mbps, seconds, limit_mb)`: True if the file fits in the limit.
@@starter
def size_mb(bitrate_mbps, seconds):
    pass


def bitrate_for(size_mb, seconds):
    pass


def uncompressed_mbps(width, height, bit_depth, sampling, fps):
    pass


def compression_ratio(uncompressed, actual):
    pass


def fits(bitrate_mbps, seconds, limit_mb):
    pass


raw = uncompressed_mbps(3840, 2160, 10, "4:2:0", 24)
print(size_mb(16, 600), bitrate_for(100, 60), raw, compression_ratio(raw, 60))
@@tests
check(size_mb(16, 600), 1200.0, "10 minutes at 16 Mb/s")
check(size_mb(220, 3600), 99000.0, "an hour of ProRes HQ")
check(bitrate_for(100, 60), 13.33, "100 MB for a one-minute clip")
check(uncompressed_mbps(1920, 1080, 8, "4:4:4", 30), 1493.0, "1080p 8-bit RGB")
check(uncompressed_mbps(1920, 1080, 10, "4:2:2", 25), 1036.8, "broadcast 4:2:2 10-bit")
check(uncompressed_mbps(3840, 2160, 10, "4:2:0", 24), 2986.0, "4K 10-bit 4:2:0")
check(compression_ratio(2986.0, 60), 49.8, "H.265 squeeze")
check(fits(8, 120, 120), True, "a 2-minute clip under 120 MB")
check(fits(10, 120, 120), False, "too big")
@@hint
Bits per frame = width × height × samples × bit_depth; × fps gives bits per second; ÷ 1,000,000 gives Mb/s.
@@solution
SAMPLES = {"4:4:4": 3, "4:2:2": 2, "4:2:0": 1.5}


def size_mb(bitrate_mbps, seconds):
    return round(bitrate_mbps * seconds / 8, 1)


def bitrate_for(size_mb, seconds):
    return round(size_mb * 8 / seconds, 2)


def uncompressed_mbps(width, height, bit_depth, sampling, fps):
    bits = width * height * SAMPLES[sampling] * bit_depth * fps
    return round(bits / 1_000_000, 1)


def compression_ratio(uncompressed, actual):
    return round(uncompressed / actual, 1)


def fits(bitrate_mbps, seconds, limit_mb):
    return bitrate_mbps * seconds / 8 <= limit_mb


raw = uncompressed_mbps(3840, 2160, 10, "4:2:0", 24)
print(size_mb(16, 600), bitrate_for(100, 60), raw, compression_ratio(raw, 60))

@@exercise Write an SRT file
Subtitles are a list of `(start_seconds, end_seconds, text)`. Write `srt_time(seconds)` returning `"HH:MM:SS,mmm"` (milliseconds rounded). Write `to_srt(subs)`: numbered blocks starting at 1, each `index`, `start --> end`, the text, and a blank line, joined with `"\n"` (see the example). Write `check_subs(subs, max_chars=42, min_dur=1.0)` returning a list of problems: `"N: too long"` if any text line is longer than `max_chars`, `"N: too short"` if displayed for less than `min_dur` seconds, `"N: overlaps"` if it starts before the previous one ends (N is the 1-based index), in that order per subtitle.
@@starter
def srt_time(seconds):
    pass


def to_srt(subs):
    pass


def check_subs(subs, max_chars=42, min_dur=1.0):
    pass


subs = [
    (1.0, 3.5, "Welcome to the launch of our app."),
    (3.4, 4.0, "It's fast."),
    (4.2, 7.0, "And it looks great in dark mode,\nlight mode, and every shade of grey in between."),
]
print(to_srt(subs))
print(check_subs(subs))
@@tests
check(srt_time(0), "00:00:00,000", "zero")
check(srt_time(3.5), "00:00:03,500", "three and a half seconds")
check(srt_time(3725.042), "01:02:05,042", "over an hour")
check(to_srt([(1, 2.25, "Hi")]), "1\n00:00:01,000 --> 00:00:02,250\nHi\n", "one block")
two = to_srt([(0, 1, "A"), (1.5, 3, "B")])
check(two, "1\n00:00:00,000 --> 00:00:01,000\nA\n\n2\n00:00:01,500 --> 00:00:03,000\nB\n", "blocks separated by a blank line")
subs = [
    (1.0, 3.5, "Welcome to the launch of our app."),
    (3.4, 4.0, "It's fast."),
    (4.2, 7.0, "And it looks great in dark mode,\nlight mode, and every shade of grey in between."),
]
check(check_subs(subs), ["2: too short", "2: overlaps", "3: too long"], "problems")
check(check_subs([(0, 2, "Fine")]), [], "no problems")
@@hint
Work in whole milliseconds: `ms = round(seconds * 1000)`, then divmod by 1000, 60 and 60. Each block is `f"{i}\n{start} --> {end}\n{text}\n"`; join blocks with `"\n"`.
@@solution
def srt_time(seconds):
    ms = round(seconds * 1000)
    s, ms = divmod(ms, 1000)
    m, s = divmod(s, 60)
    h, m = divmod(m, 60)
    return f"{h:02d}:{m:02d}:{s:02d},{ms:03d}"


def to_srt(subs):
    blocks = []
    for i, (start, end, text) in enumerate(subs, 1):
        blocks.append(f"{i}\n{srt_time(start)} --> {srt_time(end)}\n{text}\n")
    return "\n".join(blocks)


def check_subs(subs, max_chars=42, min_dur=1.0):
    problems = []
    prev_end = None
    for i, (start, end, text) in enumerate(subs, 1):
        if any(len(line) > max_chars for line in text.split("\n")):
            problems.append(f"{i}: too long")
        if end - start < min_dur:
            problems.append(f"{i}: too short")
        if prev_end is not None and start < prev_end:
            problems.append(f"{i}: overlaps")
        prev_end = end
    return problems


subs = [
    (1.0, 3.5, "Welcome to the launch of our app."),
    (3.4, 4.0, "It's fast."),
    (4.2, 7.0, "And it looks great in dark mode,\nlight mode, and every shade of grey in between."),
]
print(to_srt(subs))
print(check_subs(subs))
