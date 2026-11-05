@@lesson dr6-framerates Frame-Rate Conversion and Streaming Ladders
@@lang python
@@text
Deliverables rarely match the shooting format exactly: a 24 fps film for a 30 fps broadcast, 50 fps footage on a 24 fps timeline, or one master turned into a ladder of streaming renditions. This lesson explains how Resolve converts frame rates (and the artefacts each method leaves), how pulldown works, and how streaming platforms choose resolutions and bitrates.

## Frame-rate conversion

Set the timeline frame rate **before** editing (it can't change later in most projects). Clips at other rates are conformed or converted per clip (*Clip Attributes* and *Retime and Scaling*):
- **Nearest** (frame repeat/drop): duplicates or drops whole frames. Sharp frames but judder.
- **Frame blend**: mixes neighbouring frames; smoother but ghosting.
- **Optical Flow** (Speed Warp in Studio): synthesises in-between frames from motion vectors; smoothest, but can warp around fast or occluded motion.
- **Conform** (change playback speed): 25 fps footage played at 24 fps runs 4% slower (with audio pitch shifted unless corrected). That's the classic PAL speed-up/down. High frame-rate footage (50/60/120 fps) conformed to 24/25 gives true **slow motion**.

## Pulldown

**3:2 pulldown** shows 4 film frames as 10 video fields (5 interlaced frames) to put 23.976 fps into 29.97 fps: frames A B C D become fields AA BBB CC DDD. The frames that combine fields from two film frames (B/C and C/D) look combed on progressive screens. Reverse ("inverse telecine") removes it. On progressive-only deliveries, prefer **2:3 repeated frames** (frame repeat) or keep the native rate if the platform allows (most streaming services accept 23.976/24/25/29.97/30).

## Streaming ladders

Adaptive streaming (HLS, DASH) encodes several **renditions** and lets the player switch with bandwidth. A ladder might be 1080p at 5–8 Mbps, 720p at 3 Mbps, 540p at 1.5 Mbps, 360p at 0.8 Mbps.
- **Bits per pixel (bpp)** = bitrate / (width × height × fps). H.264 needs around **0.08–0.1 bpp** for good quality on typical content; HEVC and AV1 about 30–50% less.
- **Per-title encoding** varies the ladder by content complexity: a cartoon needs far less than grainy sports.
- Keep the **GOP/keyframe interval** aligned across renditions (e.g. every 2 s) so players can switch cleanly.
- YouTube re-encodes everything: upload a high-quality master (high bitrate H.264/H.265, or ProRes/DNxHR for short pieces) and let it build the ladder.

@@quiz
25 fps footage is conformed to a 24 fps timeline. What happens?
* It plays 4% slower, with every frame kept
- Frames are blended
- It plays 4% faster
- Every 25th frame is dropped
? 24 / 25 = 0.96.

@@quiz
Which retime method synthesises new in-between frames from motion?
- Nearest
- Frame blend
* Optical Flow
- Conform
? It can warp at occlusions.

@@quiz
In 3:2 pulldown, how many video fields do 4 film frames become?
- 8
* 10
- 12
- 4
? 2+3+2+3.

@@quiz
1920 × 1080 at 30 fps and 5 Mbps. Roughly what bpp is that?
- 0.008
* 0.08
- 0.8
- 8
? 5,000,000 / 62,208,000.

@@quiz
Why align keyframe intervals across streaming renditions?
- To reduce audio drift
* So players can switch renditions cleanly at segment boundaries
- It's required for HDR
- To shrink files
? Segments start with a keyframe.

@@flashcards
set timeline rate :: before editing
nearest retime :: repeat or drop frames
frame blend :: mix frames, ghosting
optical flow :: synthesise frames
conform :: change playback speed
PAL speed-up :: 24 → 25, 4% faster
slow motion :: high fps conformed down
3:2 pulldown :: 4 frames → 10 fields
inverse telecine :: remove pulldown
rendition :: one rung of the ladder
bpp :: bitrate / (w × h × fps)
H.264 good quality :: about 0.08–0.1 bpp
per-title encoding :: ladder per content
keyframe interval :: aligned, e.g. 2 s

@@drill
120 fps footage conformed to 24 fps plays at what fraction of real speed? (as a percentage)
= 20%
= 20
= 20 %
? 24 / 120.

@@drill
What is the name for removing 3:2 pulldown to recover the film frames?
= inverse telecine
= IVTC
= reverse telecine
? It reconstructs 23.976 fps progressive.

@@exercise Map frames between rates
For a source and target frame rate, list which source frame each of the first target frames shows with nearest-frame mapping (`src = floor(t × src_fps / dst_fps + 1e-9)`), mark repeated and dropped frames, and generate the field pattern of 3:2 pulldown for 8 film frames.
@@starter
conversions = [(24, 30, 10), (30, 24, 8), (25, 24, 26), (24, 60, 10)]

def mapping(src_fps, dst_fps, count):
    raise NotImplementedError
@@expected
24 -> 30 fps: [0, 0, 1, 2, 3, 4, 4, 5, 6, 7]  repeats 2, dropped none
30 -> 24 fps: [0, 1, 2, 3, 5, 6, 7, 8]  repeats 0, dropped [4]
25 -> 24 fps: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26]  repeats 0, dropped [24]
24 -> 60 fps: [0, 0, 0, 1, 1, 2, 2, 2, 3, 3]  repeats 6, dropped none
3:2 pulldown: AA BB BC CD DD AA BB BC CD DD | mixed frames: 4 of 10
@@requires
def mapping
def pulldown
"ABCD"
@@hint
For each target frame t, compute the source index. A repeat is a source index equal to the previous one; dropped frames are source indexes skipped between consecutive targets. Pulldown: cycle `[2, 3]` field counts over the frame letters `"ABCD"` and group the resulting fields into pairs to show the interlaced frames.
@@solution
import math

conversions = [(24, 30, 10), (30, 24, 8), (25, 24, 26), (24, 60, 10)]

def mapping(src_fps, dst_fps, count):
    return [math.floor(t * src_fps / dst_fps + 1e-9) for t in range(count)]

def pulldown(frames):
    fields = []
    for i, frame in enumerate(frames):
        fields += [frame] * (2 if i % 2 == 0 else 3)
    return [fields[i] + fields[i + 1] for i in range(0, len(fields) - 1, 2)]

for src, dst, count in conversions:
    m = mapping(src, dst, count)
    repeats = sum(1 for a, b in zip(m, m[1:]) if a == b)
    dropped = [f for a, b in zip(m, m[1:]) for f in range(a + 1, b)]
    print(f"{src} -> {dst} fps: {m}  repeats {repeats}, dropped {dropped or 'none'}")
video = pulldown("ABCD" * 2)
print("3:2 pulldown:", " ".join(video), "| mixed frames:", sum(f[0] != f[1] for f in video), "of", len(video))

@@exercise Build a streaming ladder
For each rendition, compute the bitrate needed for a target bits-per-pixel (scaled by codec efficiency and content complexity), round up to 100 kbps, and compute the size of a 10-minute video. Drop renditions whose bitrate exceeds the platform cap or that aren't at least 1.4× the rendition below (too close to be useful).
@@starter
import math

RUNG = [(1920, 1080), (1280, 720), (960, 540), (640, 360)]
FPS = 30
BASE_BPP = 0.09  # H.264, average content
CODEC = {"h264": 1.0, "hevc": 0.65, "av1": 0.55}
CONTENT = {"animation": 0.5, "talk": 0.8, "sports": 1.6}
CAP_KBPS = 8000
MINUTES = 10

def bitrate_kbps(w, h, codec, content):
    raise NotImplementedError
@@expected
h264/talk: 1080p 4500 kbps 338 MB, 720p 2000 kbps 150 MB, 540p 1200 kbps 90 MB, 360p 500 kbps 38 MB
h264/sports: 720p 4000 kbps 300 MB, 540p 2300 kbps 172 MB, 360p 1000 kbps 75 MB  [1080p over cap (9000)]
av1/sports: 1080p 5000 kbps 375 MB, 720p 2200 kbps 165 MB, 540p 1300 kbps 98 MB, 360p 600 kbps 45 MB
hevc/animation: 1080p 1900 kbps 142 MB, 720p 900 kbps 68 MB, 540p 500 kbps 38 MB, 360p 300 kbps 22 MB
@@requires
def bitrate_kbps
math.ceil(
1.4
@@hint
`bps = w * h * FPS * BASE_BPP * CODEC[codec] * CONTENT[content]`; in kbps rounded up to 100: `math.ceil(bps / 1000 / 100) * 100`. Walk from the smallest rung upwards, keeping a rung only if under the cap and ≥ 1.4 × the last kept bitrate. Size in MB = kbps × 1000 × seconds / 8 / 1e6.
@@solution
import math

RUNG = [(1920, 1080), (1280, 720), (960, 540), (640, 360)]
FPS = 30
BASE_BPP = 0.09  # H.264, average content
CODEC = {"h264": 1.0, "hevc": 0.65, "av1": 0.55}
CONTENT = {"animation": 0.5, "talk": 0.8, "sports": 1.6}
CAP_KBPS = 8000
MINUTES = 10

def bitrate_kbps(w, h, codec, content):
    bps = w * h * FPS * BASE_BPP * CODEC[codec] * CONTENT[content]
    return math.ceil(bps / 1000 / 100) * 100

for codec, content in (("h264", "talk"), ("h264", "sports"), ("av1", "sports"), ("hevc", "animation")):
    kept, notes = [], []
    for w, h in reversed(RUNG):
        kbps = bitrate_kbps(w, h, codec, content)
        if kbps > CAP_KBPS:
            notes.append(f"{h}p over cap ({kbps})")
        elif kept and kbps < 1.4 * kept[-1][1]:
            notes.append(f"{h}p too close ({kbps})")
        else:
            kept.append((h, kbps))
    ladder = ", ".join(f"{h}p {k} kbps {k * 1000 * MINUTES * 60 / 8 / 1e6:.0f} MB" for h, k in reversed(kept))
    print(f"{codec}/{content}: {ladder}" + (f"  [{'; '.join(notes)}]" if notes else ""))
