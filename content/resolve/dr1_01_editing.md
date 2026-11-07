@@lesson dr1-editing Editing on the Cut and Edit Pages
@@concepts lists, maps, strings, functions
@@lang python
@@text
Editing is storytelling with time: choosing shots, ordering them, and deciding exactly which frame to cut on. Resolve gives you two editing pages: **Cut** for speed and **Edit** for full control.

## The Cut page

Built for fast turnarounds (news, YouTube, social):

- **Source tape**: view all clips in a bin as one long tape and scrub through everything.
- **Dual timeline**: the upper timeline shows the whole edit, the lower one a zoomed view for precise trims, so you never zoom in and out.
- **Smart insert, append, ripple overwrite, close-up, place on top, source overwrite** buttons; **Sync bin** and **multicam** with synced angles.
- **Smart Reframe** (Studio): automatically reframes 16:9 to 9:16 for vertical video, following the subject.
- **Scene cut detection**, **boring detector** (flags long shots), **jump cut** detection, quick export to YouTube/Vimeo/TikTok.
- Speed editor keyboard support (Blackmagic's DaVinci Speed Editor).

## The Edit page

The classic non-linear editor (NLE):

| Area | Use |
|---|---|
| **Media Pool** | bins of clips |
| **Source viewer** | mark In (I) and Out (O) on a clip |
| **Timeline viewer** | the program |
| **Timeline** | video tracks (V1, V2...; higher tracks on top) and audio tracks (A1, A2...) |
| **Inspector** | transform, crop, composite mode, speed, stabilisation, lens correction, per-clip audio |
| **Effects Library** | transitions, titles, generators, Fusion effects and titles, Open FX, audio FX |

## Essential edits

| Edit | Shortcut | What it does |
|---|---|---|
| **Insert** | F9 | puts the source in at the playhead and pushes later clips right |
| **Overwrite** | F10 | replaces whatever is there, timeline length unchanged |
| **Replace** | F11 | swaps a clip, matching the frame under the playhead |
| **Place on top** | F12 | into the first free track above |
| **Append at end** | Shift+F12 | adds after the last clip |
| **Fit to fill** | Shift+F11 | retimes the source to fill the marked gap |
| **Ripple overwrite** | Shift+F10 | replaces and adjusts the length |

Use **J / K / L** to play backwards / stop / forwards (tap repeatedly for faster), **I / O** for In and Out, **Ctrl+B** or **B** (blade tool) to cut, **Ctrl+Shift+[ / ]** to ripple-trim start/end to the playhead, **Shift+Delete** (or Delete in some layouts) for a **ripple delete** that closes the gap, **Backspace** for a lift that leaves a gap. **N** toggles snapping, **Alt+Y** selects all clips forward on a track.

## Trimming

Trim mode (**T**) uses the cursor's position on a clip edit to choose:

- **Ripple**: lengthen or shorten a clip; everything after moves.
- **Roll**: move an edit point between two clips; total length unchanged.
- **Slip**: change which part of the source a clip shows without moving it (drag the middle of a clip).
- **Slide**: move a clip between its neighbours, which adjust their lengths.

The exercises implement ripple delete and roll/slip maths on a list of clips.

## Building the story

- **Cut on action**: cut during a movement so the eye follows it across the edit.
- **J-cuts and L-cuts**: audio leads (J) or trails (L) the picture: natural dialogue and smooth transitions. Unlink audio and video with Ctrl+Shift+L or Alt-click to select one part.
- **B-roll** on V2 over an interview on V1; **cutaways** hide jump cuts.
- **Match cuts**: similar shape or motion across a cut.
- **Pacing**: shorter shots raise energy; hold longer for emotion.
- Use **markers** (M) for notes and beats of the music; colour-code them. Cut to the beat for music videos and motion graphics edits.
- **Compound clips** (nest several clips into one) and **multiple timelines** (versions: v1, v2, 30 s cut, vertical cut).

## Transitions, titles, speed and effects

- **Transitions**: Cross Dissolve is the workhorse; Dip to Color; Smooth Cut hides jump cuts; Fusion transitions for stylised looks. Transitions need **handles**: extra source media beyond the clip's In and Out.
- **Titles**: Text, **Text+** (Fusion-powered, animatable per character), and Fusion title templates (lower thirds, callouts). Save your own as macros/templates with your brand colours.
- **Speed**: Ctrl+R for **Retime Controls** (speed points, ramps); **Retime curve**; Inspector → Retime and Scaling → **Optical Flow** or **Speed Warp** (Studio) for smooth slow motion.
- **Keyframes in the Inspector**: animate zoom, position, opacity, crop (Ken Burns moves, picture-in-picture).
- **Stabilisation** in the Inspector or Color page.
- **Adjustment clips**: one clip that applies effects or grades to everything below.

## EDLs, XML and AAF

Edits move between programs as **edit decision lists**: **EDL** (CMX 3600, a simple text list of events with source and record timecodes), **FCPXML** (Final Cut Pro / Premiere interchange), **AAF** (Avid and Pro Tools). File → Import/Export Timeline handles them. An EDL event looks like:

```text
001  A001C003 V     C        01:02:10:05 01:02:14:00 01:00:00:00 01:00:03:19
```

event number, reel/clip name, track (V/A), transition (C = cut), **source in**, **source out**, **record in**, **record out**. The second exercise parses lines like this.

@@quiz
Which edit inserts a clip at the playhead and pushes everything after it later?
* Insert (F9)
- Overwrite (F10)
- Replace (F11)
- Place on top (F12)
? Overwrite keeps the timeline length.

@@quiz
Which trim changes which part of the source a clip shows without moving the clip or changing its length?
- Ripple
- Roll
* Slip
- Slide
? Slide moves the clip itself between neighbours.

@@quiz
What is a J-cut?
- A cut shaped like a J on the timeline
* The audio of the next shot starts before its picture
- A jump cut
- Cutting on action
? An L-cut is the opposite: audio continues after the picture changes.

@@quiz
What does a ripple delete do?
- Leaves a gap
* Removes the clip and closes the gap, moving later clips earlier
- Deletes the media file
- Removes only the audio
? A lift (Backspace) leaves the gap.

@@quiz
What are handles?
- The keyboard shortcuts
* Extra source frames beyond a clip's In and Out points, needed for transitions
- Bezier controls for keyframes
- Track headers
? A 1-second dissolve needs at least half a second of handle on each side (centred).

@@drill
Which edit removes a section and closes the gap automatically?
= ripple delete
= ripple
? Shift+Delete on the Edit page.

@@flashcards
Cut page :: fast editing: source tape, dual timeline, smart tools
Edit page :: full NLE with tracks, inspector, effects
I / O :: mark In / Out
J / K / L :: play back / stop / forward
F9 / F10 / F11 / F12 :: insert / overwrite / replace / place on top
B / Ctrl+B :: blade tool / cut at playhead
ripple / roll / slip / slide :: the four trims
ripple delete vs lift :: close the gap vs leave it
J-cut / L-cut :: audio leads / audio trails the picture
B-roll :: cutaway footage over the main story
handles :: extra media beyond In/Out for transitions
compound clip :: nested clips as one
Text+ :: Fusion-powered animatable titles
Ctrl+R :: retime controls (speed ramps)
Optical Flow / Speed Warp :: smooth slow motion
EDL (CMX 3600) :: text list of edit events with timecodes
FCPXML / AAF :: timeline interchange formats

@@exercise Ripple, roll and slip
A track is a list of clips `{"name", "src_in", "src_out", "start"}` in frames (out exclusive), sorted by start, with no gaps. Write `length(clip)`. Write `ripple_delete(track, name)`: remove the clip and move every later clip earlier by its length; return a new list. Write `roll(track, index, frames)`: move the edit between clip `index` and `index + 1` by `frames` (positive = later): the first clip's `src_out` grows, the second's `src_in` and `start` grow by the same amount. Write `slip(clip, frames)`: shift `src_in` and `src_out` by `frames`. All functions return new dicts (don't modify the inputs).
@@starter
def length(clip):
    pass


def ripple_delete(track, name):
    pass


def roll(track, index, frames):
    pass


def slip(clip, frames):
    pass


track = [
    {"name": "wide", "src_in": 100, "src_out": 148, "start": 0},
    {"name": "close", "src_in": 20, "src_out": 92, "start": 48},
    {"name": "logo", "src_in": 0, "src_out": 60, "start": 120},
]
print(ripple_delete(track, "close"))
print(roll(track, 0, 12))
@@tests
track = [
    {"name": "wide", "src_in": 100, "src_out": 148, "start": 0},
    {"name": "close", "src_in": 20, "src_out": 92, "start": 48},
    {"name": "logo", "src_in": 0, "src_out": 60, "start": 120},
]
check(length(track[1]), 72, "length")
r = ripple_delete(track, "close")
check([c["name"] for c in r], ["wide", "logo"], "removed")
check(r[1]["start"], 48, "logo moved up by 72 frames")
check(track[2]["start"], 120, "the original track is unchanged")
rolled = roll(track, 0, 12)
check((rolled[0]["src_out"], rolled[1]["src_in"], rolled[1]["start"]), (160, 32, 60), "rolled 12 frames later")
check(sum(length(c) for c in rolled), sum(length(c) for c in track), "roll keeps the total length")
check(rolled[2], track[2], "later clips untouched")
check(slip(track[1], -10), {"name": "close", "src_in": 10, "src_out": 82, "start": 48}, "slip")
check(track[1]["src_in"], 20, "slip doesn't modify the input")
@@hint
Copy with `dict(c)` or `{**c}`. In ripple_delete, find the removed clip's length first, then rebuild the list, subtracting it from the start of every clip that started after the removed one.
@@solution
def length(clip):
    return clip["src_out"] - clip["src_in"]


def ripple_delete(track, name):
    gone = next(c for c in track if c["name"] == name)
    out = []
    for c in track:
        if c is gone:
            continue
        c = dict(c)
        if c["start"] > gone["start"]:
            c["start"] -= length(gone)
        out.append(c)
    return out


def roll(track, index, frames):
    out = [dict(c) for c in track]
    out[index]["src_out"] += frames
    out[index + 1]["src_in"] += frames
    out[index + 1]["start"] += frames
    return out


def slip(clip, frames):
    c = dict(clip)
    c["src_in"] += frames
    c["src_out"] += frames
    return c


track = [
    {"name": "wide", "src_in": 100, "src_out": 148, "start": 0},
    {"name": "close", "src_in": 20, "src_out": 92, "start": 48},
    {"name": "logo", "src_in": 0, "src_out": 60, "start": 120},
]
print(ripple_delete(track, "close"))
print(roll(track, 0, 12))

@@exercise Parse an EDL
Write `parse_event(line, fps)` for a CMX 3600 event line: split on whitespace into event, reel, track, transition, source in, source out, record in, record out, and return a dict with `event` (int), `reel`, `track`, `transition`, and the four timecodes converted to frame numbers as `src_in`, `src_out`, `rec_in`, `rec_out` (non-drop `HH:MM:SS:FF`). Write `parse_edl(text, fps)`: parse every line that starts with a digit (skip `TITLE:`, `FCM:`, comments starting with `*`, and blank lines). Then `edl_report(events, fps)` returns `{"events": n, "duration": total record frames, "reels": sorted unique reels, "mismatched": list of event numbers where source duration ≠ record duration}` (a speed change).
@@starter
def tc_frames(tc, fps):
    hh, mm, ss, ff = (int(x) for x in tc.split(":"))
    return ((hh * 60 + mm) * 60 + ss) * fps + ff


def parse_event(line, fps):
    pass


def parse_edl(text, fps):
    pass


def edl_report(events, fps):
    pass


EDL = """TITLE: PROMO_V3
FCM: NON-DROP FRAME

001  A001C003 V     C        01:02:10:05 01:02:14:00 01:00:00:00 01:00:03:19
* FROM CLIP NAME: drone_wide.mov
002  A002C011 V     C        02:10:00:00 02:10:02:00 01:00:03:19 01:00:05:19
003  A001C007 V     C        01:05:00:00 01:05:04:00 01:00:05:19 01:00:07:19
"""
events = parse_edl(EDL, 24)
print(events[0])
print(edl_report(events, 24))
@@tests
EDL = """TITLE: PROMO_V3
FCM: NON-DROP FRAME

001  A001C003 V     C        01:02:10:05 01:02:14:00 01:00:00:00 01:00:03:19
* FROM CLIP NAME: drone_wide.mov
002  A002C011 V     C        02:10:00:00 02:10:02:00 01:00:03:19 01:00:05:19
003  A001C007 V     C        01:05:00:00 01:05:04:00 01:00:05:19 01:00:07:19
"""
e = parse_event("001  A001C003 V     C        01:02:10:05 01:02:14:00 01:00:00:00 01:00:03:19", 24)
check((e["event"], e["reel"], e["track"], e["transition"]), (1, "A001C003", "V", "C"), "fields")
check(e["src_in"], tc_frames("01:02:10:05", 24), "source in as frames")
check(e["rec_out"] - e["rec_in"], 91, "record duration")
events = parse_edl(EDL, 24)
check(len(events), 3, "three events")
r = edl_report(events, 24)
check(r["events"], 3, "count")
check(r["duration"], 187, "total frames")
check(r["reels"], ["A001C003", "A001C007", "A002C011"], "reels")
check(r["mismatched"], [3], "event 3 plays 4 s of source in 2 s: a speed change")
@@hint
`line.split()` gives the eight fields. Lines to keep: `line.strip()[:1].isdigit()`.
@@solution
def tc_frames(tc, fps):
    hh, mm, ss, ff = (int(x) for x in tc.split(":"))
    return ((hh * 60 + mm) * 60 + ss) * fps + ff


def parse_event(line, fps):
    ev, reel, track, trans, si, so, ri, ro = line.split()[:8]
    return {
        "event": int(ev), "reel": reel, "track": track, "transition": trans,
        "src_in": tc_frames(si, fps), "src_out": tc_frames(so, fps),
        "rec_in": tc_frames(ri, fps), "rec_out": tc_frames(ro, fps),
    }


def parse_edl(text, fps):
    return [parse_event(l, fps) for l in text.splitlines() if l.strip()[:1].isdigit()]


def edl_report(events, fps):
    return {
        "events": len(events),
        "duration": sum(e["rec_out"] - e["rec_in"] for e in events),
        "reels": sorted({e["reel"] for e in events}),
        "mismatched": [e["event"] for e in events
                       if e["src_out"] - e["src_in"] != e["rec_out"] - e["rec_in"]],
    }


EDL = """TITLE: PROMO_V3
FCM: NON-DROP FRAME

001  A001C003 V     C        01:02:10:05 01:02:14:00 01:00:00:00 01:00:03:19
* FROM CLIP NAME: drone_wide.mov
002  A002C011 V     C        02:10:00:00 02:10:02:00 01:00:03:19 01:00:05:19
003  A001C007 V     C        01:05:00:00 01:05:04:00 01:00:05:19 01:00:07:19
"""
events = parse_edl(EDL, 24)
print(events[0])
print(edl_report(events, 24))
