@@lesson le4-display-audio Displays and Sound: Wayland, Multiple Monitors, Scaling, PipeWire and Bluetooth Audio
@@concepts text-processing, numbers, conditions
@@lang bash
@@text
Two things decide whether a Linux desktop feels polished: whether the screens look right and whether sound comes out of the right device. Both changed a lot in recent years. **Wayland** replaced X11 as the default display system on most distros, and **PipeWire** replaced PulseAudio and JACK for audio. This lesson explains how they work, how to set up multiple monitors and scaling, and how to fix the common sound problems from the terminal.

## Wayland and X11

- **X11** (Xorg) is the old display server from the 1980s. Apps talk to the X server, and any app can read the screen and the keyboard of any other, which is convenient for screenshot tools and terrible for security.
- **Wayland** is a protocol. The **compositor** (GNOME's Mutter, KDE's KWin, Sway, Hyprland) is the display server itself, so apps can't spy on each other. Screen sharing and screenshots go through **xdg-desktop-portal**, which asks your permission.
- **XWayland** runs old X11 apps inside a Wayland session. Most work fine, though some old apps look blurry with fractional scaling.
- Check which one you're on with `echo $XDG_SESSION_TYPE` (`wayland` or `x11`). You can usually choose at the login screen (the gear icon).
- When to fall back to X11: some remote desktop tools, a few old screen recorders or automation tools (xdotool), and occasionally NVIDIA driver problems. With NVIDIA, use driver 555 or newer, which supports explicit sync, for a smooth Wayland session.

## Multiple monitors and scaling

- Each monitor has a **resolution**, a **refresh rate**, a **scale** and a **position** in a shared logical desktop. Positions are in **logical pixels** = physical pixels ÷ scale.
  - A 3840×2160 monitor at 150% scale is 2560×1440 logical pixels wide and tall.
  - A 1920×1080 monitor placed to its right starts at x = 2560.
- Set them in GNOME Settings → Displays or KDE System Settings → Display. In Sway or Hyprland, write them in the config. On X11, `xrandr` does it from the command line: `xrandr --output HDMI-1 --mode 1920x1080 --rate 144 --right-of eDP-1`.
- **Fractional scaling** (125%, 150%): KDE supports it well. In GNOME it's enabled by default in recent versions. Blurry apps are usually X11 apps, or Electron apps that need `--ozone-platform=wayland`.
- **Refresh rate**: a 144 Hz monitor is often set to 60 Hz by default. Check the Display settings, and use DisplayPort or HDMI 2.0+ cables for high refresh rates at high resolutions.
- **Variable refresh rate (VRR, FreeSync/G-Sync)**: enable it in KDE's display settings or with GNOME's experimental setting, which is good for games.
- **Night light** reduces blue light in the evening and is built into GNOME and KDE.

## Audio with PipeWire

- **PipeWire** is the sound server: it routes audio between apps and devices. **WirePlumber** is its session manager (it decides the defaults and what connects to what). PulseAudio and JACK apps work unchanged through compatibility layers (`pipewire-pulse`, `pipewire-jack`).
- Terms: a **sink** is an output (speakers, headphones, HDMI audio), a **source** is an input (microphone), and a **stream** is an app playing or recording.
- Tools:
  - `wpctl status` shows devices, sinks, sources and streams. The default is marked with `*`.
  - `wpctl set-default ID` changes the default device.
  - `wpctl set-volume @DEFAULT_AUDIO_SINK@ 50%` sets the volume, and `wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle` mutes.
  - `pactl list short sinks` gives the PulseAudio-compatible view.
  - **pavucontrol** (a GUI) lets you move a running app to another output, which is the quickest fix when sound plays from the wrong device.
  - **Helvum** or **qpwgraph** show the routing graph.
- If sound disappears, work through it in order:
  1. Is the right default output selected?
  2. Is it muted?
  3. Is the app's stream going to another sink?
  4. Is PipeWire running? `systemctl --user status pipewire wireplumber`, and restart with `systemctl --user restart pipewire wireplumber`.
  5. Does the kernel see the card? `aplay -l`.
- **HDMI audio** needs the monitor's speakers enabled and the right **profile** (in pavucontrol's Configuration tab).

## Bluetooth audio

- Pair with GNOME/KDE settings or `bluetoothctl` (`power on`, `scan on`, `pair XX:XX...`, `connect`, `trust`).
- **Profiles**: **A2DP** is high-quality playback (codecs SBC, AAC, aptX, LDAC). **HSP/HFP** is the headset mode that enables the microphone, but the sound quality is much lower. When a call app opens the microphone, the headset switches to HFP and music sounds terrible. That's expected: use a separate microphone to keep A2DP quality.
- Newer devices with **LE Audio** (LC3 codec) improve this, and PipeWire supports it with recent kernels and BlueZ.
- If pairing fails, remove the device, restart Bluetooth (`sudo systemctl restart bluetooth`), and check `journalctl -u bluetooth`.

@@quiz
How do you check whether your session is Wayland or X11?
* echo $XDG_SESSION_TYPE
- uname -a
- xrandr --version
- wpctl status
? It prints wayland or x11.

@@quiz
Why can't a Wayland app record other windows directly?
* The compositor isolates apps; screen capture goes through a permission portal
- Wayland doesn't support video
- Only root can take screenshots
- It can, exactly like X11
? xdg-desktop-portal asks the user.

@@quiz
A 2560×1440 monitor at 125% scale. How wide is it in logical pixels?
* 2048
- 2560
- 3200
- 1440
? 2560 ÷ 1.25.

@@quiz
Sound plays through the monitor's HDMI speakers instead of the headphones. What's the quickest fix?
* Change the default sink (wpctl set-default) or move the app's stream in pavucontrol
- Reinstall PipeWire
- Reboot into X11
- Unplug the monitor permanently
? Then check the default stays set.

@@quiz
Music on Bluetooth headphones becomes muffled when you join a video call. Why?
* The headset switched from A2DP to HFP to enable the microphone
- The battery is low
- PipeWire crashed
- Wayland reduced the bitrate
? Use a separate microphone to keep A2DP.

@@quiz
Which component decides default devices and connections in a PipeWire setup?
* WirePlumber
- Xorg
- ALSA mixer
- systemd-logind
? PipeWire's session manager.

@@flashcards
X11 :: old display server, apps can see each other
Wayland :: compositor is the display server
compositor :: Mutter, KWin, Sway, Hyprland
XWayland :: runs X11 apps under Wayland
xdg-desktop-portal :: permission prompts for screen sharing
$XDG_SESSION_TYPE :: wayland or x11
logical pixels :: physical ÷ scale
xrandr :: X11 monitor setup from the terminal
VRR :: variable refresh rate (FreeSync, G-Sync)
PipeWire :: sound and video server
WirePlumber :: PipeWire session manager
sink / source / stream :: output / input / app audio
wpctl status :: devices and streams, * marks the default
pavucontrol :: move app audio between devices
A2DP :: high-quality Bluetooth playback
HSP/HFP :: Bluetooth headset mode with a microphone
LE Audio :: newer Bluetooth audio with LC3
bluetoothctl :: pair and connect from the terminal

@@drill
A 3840-pixel-wide monitor is scaled to 200%. How wide is it in logical pixels?
= 1920
? 3840 ÷ 2.

@@drill
Which Bluetooth profile gives high-quality music playback? (acronym)
= A2DP
= a2dp
? HFP is for calls.

@@exercise Lay out monitors side by side
`monitors` lists each monitor as `name width height scale` (physical pixels) in left-to-right order. Compute each one's logical size and x position when placed side by side with their tops aligned, the total desktop size, and print an `xrandr`-style command line for each (using `--pos` in physical pixels on X11, where scaling isn't applied, so physical positions are the running total of physical widths).
@@starter
monitors='eDP-1 2880 1800 2
DP-1 3840 2160 1.5
HDMI-1 1920 1080 1'

# Your code here
@@expected
eDP-1   2880x1800 at scale 2    -> logical 1440x900  at x=0
DP-1    3840x2160 at scale 1.5  -> logical 2560x1440 at x=1440
HDMI-1  1920x1080 at scale 1    -> logical 1920x1080 at x=4000
desktop (logical): 5920x1440
xrandr --output eDP-1 --mode 2880x1800 --pos 0x0
xrandr --output DP-1 --mode 3840x2160 --pos 2880x0
xrandr --output HDMI-1 --mode 1920x1080 --pos 6720x0
@@requires
awk
--pos
@@hint
In awk the fields are name `$1`, width `$2`, height `$3` and scale `$4`, so the logical width is `$2 / $4`. Keep a running `x` for logical positions and `px` for physical ones. Track the tallest logical height for the desktop size. Print `xrandr --output NAME --mode WxH --pos PXx0`.
@@solution
monitors='eDP-1 2880 1800 2
DP-1 3840 2160 1.5
HDMI-1 1920 1080 1'

printf '%s\n' "$monitors" | awk '
    {
        lw = $2 / $4; lh = $3 / $4
        printf "%-7s %4dx%-4d at scale %-4s -> logical %4dx%-4d at x=%d\n", $1, $2, $3, $4, lw, lh, x
        cmd[NR] = sprintf("xrandr --output %s --mode %dx%d --pos %dx0", $1, $2, $3, px)
        x += lw; px += $2
        if (lh > h) h = lh
    }
    END {
        printf "desktop (logical): %dx%d\n", x, h
        for (i = 1; i <= NR; i++) print cmd[i]
    }'

@@exercise Find out why there's no sound
`status` holds `wpctl status`-style output for the audio sinks and streams (sinks marked `*` are the default; volumes and `[MUTED]` are shown), plus which sink each stream plays to. Report the default sink, whether it's muted or very quiet (under 10%), streams playing to a different sink than the default, and print the `wpctl` commands that would fix each problem.
@@starter
status='Sinks:
  46. Built-in Audio Speaker      [vol: 0.65]
* 52. HDMI Monitor Audio          [vol: 0.40 MUTED]
  61. WH-1000XM4 (A2DP)           [vol: 0.05]
Streams:
  80. Firefox -> 46
  85. Spotify -> 61
  90. Zoom -> 52'

# Your code here
@@expected
default sink: 52 HDMI Monitor Audio
  HDMI Monitor Audio is muted -> wpctl set-mute 52 0
  WH-1000XM4 (A2DP) is at 5% -> wpctl set-volume 61 50%
  Firefox plays to Built-in Audio Speaker, not the default -> move it in pavucontrol, or wpctl set-default 46
  Spotify plays to WH-1000XM4 (A2DP), not the default -> move it in pavucontrol, or wpctl set-default 61
@@requires
awk
set-mute
@@hint
In awk, track the section from the `Sinks:` and `Streams:` headers. For sink lines (keep them in order in an array, since `for (k in arr)` order isn't defined), the default has `*` as its first field; take the ID (strip the dot), the name (the text between the ID and the `[`), the volume and whether `MUTED` appears. For stream lines, the last field is the sink ID. In `END`, print the findings and commands such as `wpctl set-mute 52 0`, `wpctl set-volume 61 50%` and `wpctl set-default 46`.
@@solution
status='Sinks:
  46. Built-in Audio Speaker      [vol: 0.65]
* 52. HDMI Monitor Audio          [vol: 0.40 MUTED]
  61. WH-1000XM4 (A2DP)           [vol: 0.05]
Streams:
  80. Firefox -> 46
  85. Spotify -> 61
  90. Zoom -> 52'

printf '%s\n' "$status" | awk '
    /^Sinks:/ { section = "sinks"; next }
    /^Streams:/ { section = "streams"; next }
    section == "sinks" {
        def = ($1 == "*"); if (def) sub(/^\* */, ""); else sub(/^ */, "")
        id = $1; sub(/\./, "", id)
        name = $0; sub(/^[0-9]+\. */, "", name); sub(/ *\[.*$/, "", name)
        match($0, /vol: [0-9.]+/); vol = substr($0, RSTART + 5, RLENGTH - 5)
        sinks[id] = name; volume[id] = vol; muted[id] = ($0 ~ /MUTED/); ids[++m] = id
        if (def) default_id = id
    }
    section == "streams" { app = $2; target = $NF; streams[app] = target; order[++n] = app }
    END {
        print "default sink: " default_id " " sinks[default_id]
        for (j = 1; j <= m; j++) {
            id = ids[j]
            if (muted[id]) printf "  %s is muted -> wpctl set-mute %s 0\n", sinks[id], id
            else if (volume[id] < 0.10) printf "  %s is at %d%% -> wpctl set-volume %s 50%%\n", sinks[id], volume[id] * 100, id
        }
        for (i = 1; i <= n; i++) {
            app = order[i]; t = streams[app]
            if (t != default_id) printf "  %s plays to %s, not the default -> move it in pavucontrol, or wpctl set-default %s\n", app, sinks[t], t
        }
    }'
