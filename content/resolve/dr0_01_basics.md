@@lesson dr0-basics DaVinci Resolve: Pages, Projects, Media and Timecode
@@concepts numbers, strings, functions
@@lang python
@@text
**DaVinci Resolve** (Blackmagic Design) is a complete post-production studio in one application: editing, visual effects and motion graphics (**Fusion**), the industry-standard **colour grading** used on Hollywood films, professional audio (**Fairlight**) and delivery. The free version is extraordinarily capable; **DaVinci Resolve Studio** (a one-time purchase) adds GPU-accelerated codecs, noise reduction, HDR tools, AI features (Magic Mask v2, voice isolation, IntelliTrack...), more Resolve FX and higher resolutions/frame rates.

This course takes you from importing your first clip to grading, compositing, mixing and delivering, with Python exercises on the maths of video: timecode, frame rates, colour, audio levels and bitrates.

## Installing

Download from blackmagicdesign.com (Windows, macOS, Linux). Requirements: a decent GPU with recent drivers (Resolve is GPU-heavy), 16–32 GB RAM, fast SSD storage for media and caches. On Linux, the free version can't decode H.264/H.265 (AAC audio too) on some systems: transcode to DNxHR or ProRes first, or use Studio.

## The pages

Tabs along the bottom switch between **pages**, each a complete workspace:

| Page | Purpose |
|---|---|
| **Media** | import, organise and preview footage, sync audio, metadata |
| **Cut** | fast editing for quick turnarounds and social media; source tape, dual timeline |
| **Edit** | traditional NLE: multiple tracks, trimming, titles, transitions, keyframes |
| **Fusion** | node-based VFX and motion graphics |
| **Color** | node-based colour correction and grading, scopes, tracking |
| **Fairlight** | audio editing, mixing, effects, mastering |
| **Deliver** | render queue and export |

All pages share the same timeline: change a clip on the Edit page, and its grade on the Color page follows it.

## Projects and databases

- The **Project Manager** (the house icon) holds projects in a **database**: local disk databases, or PostgreSQL / Blackmagic Cloud for collaboration.
- Export a project as `.drp` (settings and timelines) or **Export Project Archive** `.dra` (with media) for backups and hand-offs.
- **Project Settings** (gear icon, bottom right). Set **before** editing:
  - **Timeline resolution** (1920 × 1080, 3840 × 2160, 1080 × 1920 vertical...).
  - **Timeline frame rate**: 23.976, 24, 25, 29.97, 30, 50, 59.94, 60. *It can't be changed after media is on a timeline* (individual timelines can override it in newer versions, but plan it).
  - **Playback frame rate**, **colour science** (DaVinci YRGB, YRGB Color Managed, ACES), working colour space.
  - **Optimized media** and **render cache** formats; **Proxy** settings.
- **Preferences** (DaVinci Resolve → Preferences): media storage locations (put the cache on your fastest drive), GPU processing mode, memory, autosave (Project Save and Load → Live Save + backups!).

## Media page and media management

- Add folders under **Media Storage** browser, drag clips to the **Media Pool**. Organise in **bins** (by day, camera, scene) and **Smart Bins** (auto-filled by metadata: all clips with keyword "drone").
- **Clip attributes** (right-click): interpret frame rate (play 60 fps as 24 fps = slow motion), pixel aspect, audio channel mapping.
- **Sync audio** from an external recorder by **waveform** or **timecode**.
- **Metadata** panel: scene, take, keywords, good take flags; the **Media Pool list view** shows columns for sorting.
- **Media Management** (File → Media Management): copy, move or **trim** used media for archiving.

## Proxies and optimized media

Long-GOP codecs (H.264/H.265 from phones and mirrorless cameras) are hard to decode in real time, especially 4K 10-bit. Options:

- **Proxy media** (right-click → Generate Proxy Media, or the Blackmagic Proxy Generator app): small, edit-friendly copies (e.g. half resolution ProRes Proxy or DNxHR LB); toggle with Playback → Use Proxy. Relink to originals automatically when rendering.
- **Optimized media**: similar, managed by Resolve.
- **Render cache** (Smart/User): pre-renders effects-heavy sections (red line above the timeline turns blue when cached).
- **Timeline proxy resolution** (Playback → Timeline Proxy Resolution: Half/Quarter) for quick previews.

## Timecode

Every frame has a **timecode** `HH:MM:SS:FF`. At 25 fps, `00:00:01:00` is frame 25. Editors use timecode to log and conform shots, and EDLs (edit decision lists) use it to describe edits.

**Drop-frame timecode** exists because NTSC video runs at **29.97 fps** (30 × 1000/1001), not 30. Counting 30 frames per second, the clock would drift 3.6 seconds per hour from real time. Drop-frame timecode skips timecode **numbers** (never actual frames): it drops frame numbers 00 and 01 at the start of every minute, except every tenth minute. It's written with a semicolon: `01:00:00;00`. The exercises convert frames to timecode in both modes.

Frame rates you'll meet:

| Rate | Use |
|---|---|
| 23.976 | film look for NTSC regions (US, Japan) |
| 24 | cinema |
| 25 / 50 | PAL regions (Europe, much of the world) |
| 29.97 / 59.94 | NTSC TV and many cameras |
| 30 / 60 | web, phones, games, screen recordings |
| 119.88 / 120 | high frame rate, slow motion source |

Match the timeline to the main camera's rate; conform slow-motion footage (shot at 120) to the timeline rate to play it slowly.

@@quiz
Which page holds Resolve's node-based colour grading?
- Fusion
* Color
- Fairlight
- Media
? Fusion is node-based too, but for VFX and motion graphics.

@@quiz
Why set the timeline frame rate before editing?
* It can't easily be changed once media is on a timeline, and every edit and render depends on it
- It affects only audio
- Resolve chooses it automatically for each clip
- It changes the colour science
? Match it to your main camera and delivery.

@@quiz
What does drop-frame timecode drop?
- Actual video frames
* Timecode numbers (00 and 01 each minute, except every tenth minute), so timecode matches the clock at 29.97 fps
- Audio samples
- Every tenth frame
? No picture is lost; only the labels skip.

@@quiz
Why generate proxy media for H.264/H.265 4K footage?
- To improve the final render quality
* Long-GOP compressed footage is hard to decode in real time; proxies are light edit-friendly copies, and Resolve renders from the originals
- Proxies are required for colour grading
- To add timecode
? Toggle proxies in the Playback menu.

@@quiz
What is a Smart Bin?
- A bin that compresses clips
* A bin filled automatically by metadata rules (keywords, camera, scene)
- The render cache folder
- A bin shared online
? Great for organising large projects.

@@flashcards
Media / Cut / Edit / Fusion / Color / Fairlight / Deliver :: Resolve's pages
Resolve vs Resolve Studio :: free vs paid with GPU codecs, NR, HDR, AI tools
Project Manager :: projects in a database
.drp / .dra :: project export / project archive with media
Project Settings :: resolution, frame rate, colour science
Media Pool / bins / Smart Bins :: clip library, folders, rule-based folders
sync by waveform :: line up external audio
proxy media :: light copies for editing
render cache :: pre-rendered effects (red line turns blue)
timecode :: HH:MM:SS:FF address of a frame
drop-frame timecode :: skips numbers at 29.97 fps, written with ;
29.97 :: 30 × 1000/1001
23.976 / 24 / 25 / 29.97 / 30 :: common frame rates
Live Save :: continuous project saving

@@exercise Timecode
Write `frames_to_tc(frames, fps)` returning `"HH:MM:SS:FF"` for an integer frame rate. Write `tc_to_frames(tc, fps)` for the reverse. Write `duration(tc_in, tc_out, fps)`: the number of frames between two timecodes (out is exclusive). Then `retime(frames, source_fps, target_fps)`: when a clip shot at `source_fps` is conformed to play at `target_fps` frame for frame, return `(new_seconds, speed_percent)`: the new duration in seconds (rounded to 2) and the playback speed (rounded to 1).
@@starter
def frames_to_tc(frames, fps):
    pass


def tc_to_frames(tc, fps):
    pass


def duration(tc_in, tc_out, fps):
    pass


def retime(frames, source_fps, target_fps):
    pass


print(frames_to_tc(90061, 25), tc_to_frames("01:00:00:00", 24), retime(240, 120, 24))
@@tests
check(frames_to_tc(0, 25), "00:00:00:00", "zero")
check(frames_to_tc(25, 25), "00:00:01:00", "one second")
check(frames_to_tc(90061, 25), "01:00:02:11", "an hour and a bit")
check(tc_to_frames("01:00:00:00", 24), 86400, "one hour at 24")
check(tc_to_frames(frames_to_tc(123456, 30), 30), 123456, "round trip")
check(duration("00:00:10:00", "00:00:12:12", 24), 60, "two and a half seconds")
check(retime(240, 120, 24), (10.0, 20.0), "2 s of 120 fps as slow motion")
check(retime(50, 50, 25), (2.0, 50.0), "50 fps at 25: half speed")
@@hint
`ff = frames % fps`, `secs = frames // fps`, then `divmod` for hours and minutes. Parse with `tc.split(":")`. Conformed duration is `frames / target_fps`; speed is `target_fps / source_fps × 100`.
@@solution
def frames_to_tc(frames, fps):
    ff = frames % fps
    secs = frames // fps
    hh, rest = divmod(secs, 3600)
    mm, ss = divmod(rest, 60)
    return f"{hh:02d}:{mm:02d}:{ss:02d}:{ff:02d}"


def tc_to_frames(tc, fps):
    hh, mm, ss, ff = (int(x) for x in tc.split(":"))
    return ((hh * 60 + mm) * 60 + ss) * fps + ff


def duration(tc_in, tc_out, fps):
    return tc_to_frames(tc_out, fps) - tc_to_frames(tc_in, fps)


def retime(frames, source_fps, target_fps):
    return round(frames / target_fps, 2), round(target_fps / source_fps * 100, 1)


print(frames_to_tc(90061, 25), tc_to_frames("01:00:00:00", 24), retime(240, 120, 24))

@@exercise Drop-frame timecode
At 29.97 fps drop-frame, timecode labels 00 and 01 are skipped at the start of every minute except minutes divisible by 10. Write `frames_to_df(frames)` returning `"HH:MM:SS;FF"`. A standard algorithm: there are 17982 frames per 10 minutes and 1798 per "dropped" minute; `d, m = divmod(frames, 17982)`; if `m > 1`, add `18 * d + 2 * ((m - 2) // 1798)` to frames, else add `18 * d`; then format the adjusted number as 30 fps non-drop with `;` before the frames. Then `drift_seconds(hours)`: how many seconds of real time 30 fps non-drop timecode drifts from the clock after `hours` of 29.97 fps video (rounded to 2).
@@starter
def frames_to_df(frames):
    pass


def drift_seconds(hours):
    pass


for f in (0, 1799, 1800, 17982, 107892):
    print(f, frames_to_df(f))
print(drift_seconds(1))
@@tests
check(frames_to_df(0), "00:00:00;00", "start")
check(frames_to_df(1799), "00:00:59;29", "end of the first minute")
check(frames_to_df(1800), "00:01:00;02", "00 and 01 are skipped")
check(frames_to_df(3598), "00:02:00;02", "again at minute 2")
check(frames_to_df(17982), "00:10:00;00", "minute 10 keeps 00")
check(frames_to_df(107892), "01:00:00;00", "one real hour")
check(drift_seconds(1), 3.6, "3.6 s per hour")
check(drift_seconds(24), 86.31, "a day")
@@hint
After adjusting, `ff = n % 30`, `ss = n // 30 % 60`, `mm = n // 1800 % 60`, `hh = n // 108000`. Drift: an hour of 29.97 fps video has `3600 × 30000/1001` frames; labelling them at 30 fps gives `frames / 30` timecode seconds; the difference from 3600 is the drift.
@@solution
def frames_to_df(frames):
    d, m = divmod(frames, 17982)
    if m > 1:
        n = frames + 18 * d + 2 * ((m - 2) // 1798)
    else:
        n = frames + 18 * d
    ff = n % 30
    ss = n // 30 % 60
    mm = n // 1800 % 60
    hh = n // 108000
    return f"{hh:02d}:{mm:02d}:{ss:02d};{ff:02d}"


def drift_seconds(hours):
    real = hours * 3600
    frames = real * 30000 / 1001
    return round(real - frames / 30, 2)


for f in (0, 1799, 1800, 17982, 107892):
    print(f, frames_to_df(f))
print(drift_seconds(1))
