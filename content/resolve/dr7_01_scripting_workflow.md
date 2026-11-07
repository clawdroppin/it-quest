@@lesson dr7-scripting Scripting, Collaboration and a Complete Workflow
@@concepts maps, lists, strings, automation
@@lang python
@@text
Resolve can be driven from **Python** (and Lua): import media, build timelines, apply LUTs and grades, add render jobs and render, all without clicking. Combined with a good folder structure and collaboration features, that's how small teams deliver like big ones.

## The scripting API

Enable it in Preferences → System → General → **External scripting using: Local** (Studio allows network scripting; the free version runs scripts from the **Workspace → Scripts** menu and the Console). Documentation ships with Resolve: Help → Documentation → Developer → `Scripting/README.txt`.

Scripts placed in the Scripts folder appear in Workspace → Scripts:

- Windows: `%PROGRAMDATA%\Blackmagic Design\DaVinci Resolve\Fusion\Scripts\Utility\`
- macOS: `/Library/Application Support/Blackmagic Design/DaVinci Resolve/Fusion/Scripts/Utility/`
- Linux: `/opt/resolve/Fusion/Scripts/Utility/`

The object hierarchy:

```text
resolve = app.GetResolve()                      # inside Resolve (or DaVinciResolveScript.scriptapp("Resolve") outside)
pm = resolve.GetProjectManager()
project = pm.GetCurrentProject()                # or pm.CreateProject("Promo")
media_pool = project.GetMediaPool()
storage = resolve.GetMediaStorage()

# Import a folder of clips into a new bin and build a timeline
bin = media_pool.AddSubFolder(media_pool.GetRootFolder(), "Day 1")
media_pool.SetCurrentFolder(bin)
clips = media_pool.ImportMedia(["D:/footage/day1"])
timeline = media_pool.CreateTimelineFromClips("Assembly", clips)

# Markers and metadata
timeline.AddMarker(48, "Blue", "Logo", "Logo hits here", 1)
clips[0].SetMetadata("Keywords", "drone")

# Grade: apply a LUT to node 1 of every clip on V1
for item in timeline.GetItemListInTrack("video", 1):
    item.SetLUT(1, "Blackmagic Design/Blackmagic 4.6K Film to Video.cube")

# Render
project.SetCurrentRenderFormatAndCodec("mp4", "H264")
project.SetRenderSettings({"TargetDir": "D:/exports", "CustomName": "promo_v3", "FormatWidth": 1920, "FormatHeight": 1080})
job = project.AddRenderJob()
project.StartRendering(job)
```

Other useful calls: `resolve.OpenPage("color")`, `project.GetSetting()` / `SetSetting()`, `timeline.GetSetting()`, `media_pool.AppendToTimeline()`, `timeline_item.AddFusionComp()`, `project.LoadRenderPreset()`, `project.GetRenderJobStatus()`, `timeline.Export(path, resolve.EXPORT_EDL, ...)`, `GetCurrentVideoItem()`, `SetCDL({...})` for CDL values, `ExportStills` from the Gallery.

**Fusion scripting** (Lua or Python) automates comps: `comp.AddTool("Text", -32768, -32768)`, set inputs, connect tools; **Fuses** are custom Fusion tools written in Lua.

**Workflow Integration Plugins** (Studio) build custom panels in Resolve with Electron/JavaScript. **DCTL** (DaVinci Color Transform Language, CUDA-like) writes custom colour effects that run on the GPU.

## Automation ideas

- Watch-folder ingest: copy card dumps, create bins by date and camera, generate proxies.
- Auto-assemble daily selects from metadata ("good take" flag).
- Batch-apply input LUTs or CSTs by camera model.
- Version renders: the same timeline as YouTube 4K, 1080p client review with burn-ins and a 9:16 cut, with names from a template.
- Export marker lists to CSV for clients or subtitles.

The exercises plan these jobs in plain Python, then you can paste the logic into a Resolve script.

## Collaboration

- **Blackmagic Cloud**: host project libraries online; multiple editors, colourists and audio engineers work on one project at the same time; **Bin locking**, **timeline compare**, and chat. **Presentations** share review links with clients who leave timecoded comments.
- **Multi-user** with a shared PostgreSQL database on a local network (Studio).
- **Proxies with Blackmagic Cloud Storage/Dropbox sync** for remote teams.

## Folder structure and naming

A predictable structure saves hours:

```text
2026-11_Promo_AppLaunch/
  01_Footage/      A001_CamA/  B001_CamB/  Drone/
  02_Audio/        Recorder/  Music/  SFX/
  03_Graphics/     Logos/  Lottie/  Blender_Renders/
  04_Project/      Resolve_Backups/
  05_Exports/      Review/  Masters/  Social/
  06_Docs/         Script/  Shotlist/  Licences/
```

Names like `Promo_AppLaunch_v03_YT4K_2026-11-02.mp4` tell everyone what they're looking at. Never "final_FINAL2.mp4".

## A complete creative workflow (Blender + Resolve + your app)

1. **Plan**: script, storyboard (Grease Pencil), shot list, music.
2. **Capture**: screen recordings of your Tauri app at 60 fps, camera footage in log, room tone, voice-over at 48 kHz.
3. **Create**: 3D device mock-ups and logo animations in Blender (EXR sequences), Lottie micro-interactions rendered from Glaxnimate, icons from Inkscape.
4. **Edit** on the Cut/Edit pages to music; titles in Text+ with your brand's colours (the theme tokens from the Vector UI course).
5. **Fusion**: screen replacements, motion graphics, callouts.
6. **Color**: CST/colour management, matching, a subtle look; keep UI screen recordings neutral.
7. **Fairlight**: dialogue clean-up, music ducking, loudness to −14 LUFS.
8. **Deliver**: YouTube 4K, 9:16 social cut, a GIF/WebM loop for your README, a ProRes master and project archive.

@@quiz
What do you need to enable before running external Python scripts against Resolve?
* Preferences → System → General → External scripting using: Local (or Network in Studio)
- The Fairlight page
- A Blackmagic Cloud account
- ACES colour science
? Scripts in the Scripts folder can also run from the Workspace → Scripts menu.

@@quiz
Which object do you get from `project.GetMediaPool()`?
- The render queue
* The Media Pool, used to make bins, import media and create timelines
- The current timeline
- The Gallery
? Then `ImportMedia`, `AddSubFolder`, `CreateTimelineFromClips`...

@@quiz
What is DCTL?
- A subtitle format
* DaVinci Color Transform Language, for writing custom GPU colour effects
- A delivery codec
- A Fusion particle tool
? It's C-like and runs on CUDA, OpenCL or Metal.

@@quiz
Which Resolve feature lets several people work on one project simultaneously over the internet?
- Render cache
* Blackmagic Cloud project libraries (with bin locking)
- Smart Bins
- The Cut page
? Multi-user also works with a local PostgreSQL server in Studio.

@@drill
Which two languages can script DaVinci Resolve? Name the one that isn't Lua.
= Python
? Lua is the other.

@@flashcards
External scripting :: Preferences → System → General
GetResolve / GetProjectManager / GetCurrentProject :: entry points of the API
GetMediaPool :: bins, imports, timelines
CreateTimelineFromClips :: build a timeline in code
AddMarker :: add timeline markers from scripts
SetLUT :: apply a LUT to a node
AddRenderJob / StartRendering :: queue and render
Fuse :: custom Fusion tool in Lua
DCTL :: custom GPU colour transforms
Workflow Integration Plugin :: custom Resolve panel (Studio)
Blackmagic Cloud :: online collaborative project libraries
Presentations :: client review links with comments
naming template :: project_version_format_date
watch folder :: automatic ingest of new media

@@exercise Plan render jobs
A delivery spec is a list of dicts `{"name", "width", "height", "codec", "mbps"}`. Write `job_name(project, version, spec, date)` returning `"<project>_v<version two digits>_<name>_<date>"`, e.g. `"Promo_v03_YT4K_2026-11-02"`. Write `render_jobs(project, version, specs, date, seconds)` returning a list of dicts per spec: `{"file": job_name + extension, "size_mb": rounded to 1}` where the extension is `.mov` for codecs `"ProRes"` and `"DNxHR"`, `.mp4` otherwise, and size uses the spec's bitrate. Then `total_size(jobs)` returns the total megabytes rounded to 1, and `vertical(specs)` returns the names of specs taller than they are wide.
@@starter
def job_name(project, version, spec, date):
    pass


def render_jobs(project, version, specs, date, seconds):
    pass


def total_size(jobs):
    pass


def vertical(specs):
    pass


SPECS = [
    {"name": "YT4K", "width": 3840, "height": 2160, "codec": "H265", "mbps": 45},
    {"name": "Review", "width": 1920, "height": 1080, "codec": "H264", "mbps": 8},
    {"name": "Reel", "width": 1080, "height": 1920, "codec": "H264", "mbps": 16},
    {"name": "Master", "width": 3840, "height": 2160, "codec": "ProRes", "mbps": 880},
]
jobs = render_jobs("Promo", 3, SPECS, "2026-11-02", 90)
for j in jobs:
    print(j)
print(total_size(jobs), vertical(SPECS))
@@tests
SPECS = [
    {"name": "YT4K", "width": 3840, "height": 2160, "codec": "H265", "mbps": 45},
    {"name": "Review", "width": 1920, "height": 1080, "codec": "H264", "mbps": 8},
    {"name": "Reel", "width": 1080, "height": 1920, "codec": "H264", "mbps": 16},
    {"name": "Master", "width": 3840, "height": 2160, "codec": "ProRes", "mbps": 880},
]
check(job_name("Promo", 3, SPECS[0], "2026-11-02"), "Promo_v03_YT4K_2026-11-02", "naming template")
jobs = render_jobs("Promo", 3, SPECS, "2026-11-02", 90)
check(jobs[0], {"file": "Promo_v03_YT4K_2026-11-02.mp4", "size_mb": 506.2}, "YouTube 4K")
check(jobs[3]["file"].endswith(".mov"), True, "ProRes goes in a .mov")
check(jobs[3]["size_mb"], 9900.0, "a big master")
check(total_size(jobs), 10676.2, "total")
check(vertical(SPECS), ["Reel"], "the 9:16 version")
check(job_name("X", 12, {"name": "A"}, "d"), "X_v12_A_d", "two-digit versions")
@@hint
`f"{version:02d}"` pads the version. Size is `mbps * seconds / 8`.
@@solution
def job_name(project, version, spec, date):
    return f"{project}_v{version:02d}_{spec['name']}_{date}"


def render_jobs(project, version, specs, date, seconds):
    jobs = []
    for spec in specs:
        ext = ".mov" if spec["codec"] in ("ProRes", "DNxHR") else ".mp4"
        jobs.append({"file": job_name(project, version, spec, date) + ext,
                     "size_mb": round(spec["mbps"] * seconds / 8, 1)})
    return jobs


def total_size(jobs):
    return round(sum(j["size_mb"] for j in jobs), 1)


def vertical(specs):
    return [s["name"] for s in specs if s["height"] > s["width"]]


SPECS = [
    {"name": "YT4K", "width": 3840, "height": 2160, "codec": "H265", "mbps": 45},
    {"name": "Review", "width": 1920, "height": 1080, "codec": "H264", "mbps": 8},
    {"name": "Reel", "width": 1080, "height": 1920, "codec": "H264", "mbps": 16},
    {"name": "Master", "width": 3840, "height": 2160, "codec": "ProRes", "mbps": 880},
]
jobs = render_jobs("Promo", 3, SPECS, "2026-11-02", 90)
for j in jobs:
    print(j)
print(total_size(jobs), vertical(SPECS))

@@exercise Organise ingest by metadata
Clips arrive as dicts `{"file", "camera", "date", "fps", "good"}`. Write `bins(clips)` returning a dict mapping `"<date>/<camera>"` to a sorted list of file names (what a watch-folder script would create). Write `selects(clips)`: file names of clips flagged `good`, in their original order. Write `fps_mismatches(clips, timeline_fps)`: sorted file names whose fps differs from the timeline. Then `markers_csv(markers)`: markers are `(frame, color, name)`; return CSV text with a header `frame,color,name` and one line per marker sorted by frame, names containing commas wrapped in double quotes.
@@starter
def bins(clips):
    pass


def selects(clips):
    pass


def fps_mismatches(clips, timeline_fps):
    pass


def markers_csv(markers):
    pass


CLIPS = [
    {"file": "A001_C003.mov", "camera": "A", "date": "2026-11-01", "fps": 23.976, "good": True},
    {"file": "B001_C001.mp4", "camera": "B", "date": "2026-11-01", "fps": 29.97, "good": False},
    {"file": "A001_C001.mov", "camera": "A", "date": "2026-11-01", "fps": 23.976, "good": False},
    {"file": "DJI_0042.mp4", "camera": "Drone", "date": "2026-11-02", "fps": 23.976, "good": True},
]
print(bins(CLIPS))
print(selects(CLIPS), fps_mismatches(CLIPS, 23.976))
print(markers_csv([(240, "Blue", "Logo hit"), (48, "Red", "Fix colour, shot 2")]))
@@tests
CLIPS = [
    {"file": "A001_C003.mov", "camera": "A", "date": "2026-11-01", "fps": 23.976, "good": True},
    {"file": "B001_C001.mp4", "camera": "B", "date": "2026-11-01", "fps": 29.97, "good": False},
    {"file": "A001_C001.mov", "camera": "A", "date": "2026-11-01", "fps": 23.976, "good": False},
    {"file": "DJI_0042.mp4", "camera": "Drone", "date": "2026-11-02", "fps": 23.976, "good": True},
]
check(bins(CLIPS), {
    "2026-11-01/A": ["A001_C001.mov", "A001_C003.mov"],
    "2026-11-01/B": ["B001_C001.mp4"],
    "2026-11-02/Drone": ["DJI_0042.mp4"],
}, "bins by date and camera")
check(selects(CLIPS), ["A001_C003.mov", "DJI_0042.mp4"], "good takes")
check(fps_mismatches(CLIPS, 23.976), ["B001_C001.mp4"], "the 29.97 camera")
csv_text = markers_csv([(240, "Blue", "Logo hit"), (48, "Red", "Fix colour, shot 2")])
check(csv_text, 'frame,color,name\n48,Red,"Fix colour, shot 2"\n240,Blue,Logo hit', "CSV with quoting")
check(markers_csv([]), "frame,color,name", "header only")
@@hint
Use `dict.setdefault(key, []).append(file)` and sort each list at the end. Join the CSV lines with `"\n"`.
@@solution
def bins(clips):
    out = {}
    for c in clips:
        out.setdefault(f"{c['date']}/{c['camera']}", []).append(c["file"])
    return {k: sorted(v) for k, v in out.items()}


def selects(clips):
    return [c["file"] for c in clips if c["good"]]


def fps_mismatches(clips, timeline_fps):
    return sorted(c["file"] for c in clips if c["fps"] != timeline_fps)


def markers_csv(markers):
    lines = ["frame,color,name"]
    for frame, color, name in sorted(markers):
        if "," in name:
            name = '"' + name + '"'
        lines.append(f"{frame},{color},{name}")
    return "\n".join(lines)


CLIPS = [
    {"file": "A001_C003.mov", "camera": "A", "date": "2026-11-01", "fps": 23.976, "good": True},
    {"file": "B001_C001.mp4", "camera": "B", "date": "2026-11-01", "fps": 29.97, "good": False},
    {"file": "A001_C001.mov", "camera": "A", "date": "2026-11-01", "fps": 23.976, "good": False},
    {"file": "DJI_0042.mp4", "camera": "Drone", "date": "2026-11-02", "fps": 23.976, "good": True},
]
print(bins(CLIPS))
print(selects(CLIPS), fps_mismatches(CLIPS, 23.976))
print(markers_csv([(240, "Blue", "Logo hit"), (48, "Red", "Fix colour, shot 2")]))
