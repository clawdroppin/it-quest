@@lesson ua0-timing Timing and Choreography: Durations, Distance, Staggers and Timelines
@@lang python
@@text
Easing decides how a motion feels, and **timing** decides whether it feels right at all. Too slow and the interface feels sluggish. Too fast and people miss what changed. When several things move together, **choreography** (the order and overlap of their motions) tells the eye where to look. This lesson turns the usual guidance into numbers you can compute.

## Durations

| Motion | Typical duration |
|---|---|
| Micro feedback (button press, toggle, ripple) | 80–150 ms |
| Small elements entering or changing (tooltip, chip, checkbox) | 150–200 ms |
| Medium (menus, cards expanding, snackbars) | 200–300 ms |
| Large or full-screen (sheets, page transitions) | 300–500 ms |

- **Distance and size**: bigger areas and longer travel need more time, but **not proportionally**. Doubling the distance should add only about 30–40% to the duration. A common model is duration ∝ distance^0.5, clamped to a range.
- **Exits are faster than entrances** (about 75% of the time). Users have already decided to dismiss, so don't make them watch.
- **Mobile vs desktop**: desktop motions are usually shorter (~150–200 ms), because the pointer is precise and screens are big but motion distances on them are short relative to attention.
- Over **500 ms** for UI motion feels like waiting.

## Choreography

- **Stagger**: items entering one after another, 20–50 ms apart. Cap the total, so the 30th list item doesn't wait a second and a half: stagger only the first ~10 items, or shrink the step as the count grows.
- **Sequence vs overlap**: animate the container first, then its contents, overlapping by 30–50% rather than waiting for each to finish.
- **Shared element / continuity**: an element that moves from one place to another (a thumbnail growing into a detail view) should keep its identity, so the eye follows it.
- **One focal motion**: if everything moves, nothing stands out.

## Reviewing motion like an editor

Timing is easiest to judge when you stop watching the whole animation and look at it the way an editor scrubs a cut. A few habits help:

- **Slow it down.** Browser dev tools can play animations at 25% or 10% speed (Chrome and Firefox both have an Animations panel). At reduced speed you can see an element that starts a frame too late, an overshoot that's bigger than intended, or two moves that collide.
- **Step frame by frame.** Record the screen and step through it in a video player. Count frames: at 60 fps, 200 ms is 12 frames, and a 50 ms stagger is 3 frames. If an item's motion is only visible for two or three frames, it reads as a flicker, not a move.
- **Watch the start and the end separately.** Most problems sit at the edges: an element that jumps when the animation begins because its starting state wasn't set, or a little snap at the end because the final keyframe doesn't match the resting style.
- **Check it with real content.** A list stagger that feels good with five items can take forever with fifty. Cap the total: either cap the number of staggered items (animate the first eight, show the rest at once) or shrink the stagger as the count grows so the whole group finishes within about 400–500 ms.
- **Try it repeatedly.** An animation you see once in a demo might be seen hundreds of times a day by a real user. Anything that delays an action they repeat often should be short or removed.

## Choreography checklist

Before you ship a sequence, run through these questions:

1. Does every move have a **reason**: showing where something came from, where it went, or what changed?
2. Is there one **lead** element, with the others following it rather than competing?
3. Do related elements **share an easing curve and a direction**, so they look like one gesture?
4. Does the sequence still make sense with **reduced motion** turned on, using fades or instant changes instead of movement?
5. Does any element wait for an animation to finish before it accepts input? It shouldn't; interfaces must stay usable during motion.

@@quiz
About how long should a button press feedback animation last?
* 80–150 ms
- 300–500 ms
- 1 second
- 600 ms
? Feedback must feel instant.

@@quiz
Should an exit animation take longer or shorter than the matching entrance?
- Longer
* Shorter
- The same
- Exits shouldn't be animated
? The user has already moved on.

@@quiz
A list of 40 items staggers in at 50 ms each. What goes wrong?
* The last item waits 2 seconds; cap the stagger to the first items or shrink the step
- Nothing
- Staggers only work up to 3 items
- The browser drops all animations
? Total time matters more than the step.

@@quiz
A panel travels twice as far as before. How should its duration change?
- Double
* Grow by roughly a third, not double
- Stay exactly the same
- Halve
? Duration grows sub-linearly with distance.

@@quiz
A card expands and its content fades in. Which choreography feels best?
- Content fades in only after the card fully finishes
* The content starts while the card is still expanding, overlapping by about a third
- Both start and end at exactly the same time
- Content first, then the card
? Overlap keeps it fluid.

@@quiz
What does a shared-element transition preserve?
* The element's identity, so the eye follows it from one view to the next
- Its colour only
- The page URL
- The scroll position
? A thumbnail growing into a detail view.

@@flashcards
micro feedback :: 80–150 ms
small elements :: 150–200 ms
medium transitions :: 200–300 ms
large transitions :: 300–500 ms
over 500 ms :: feels like waiting
exit vs entrance :: exits about 75% of the entrance time
distance scaling :: sub-linear, about √distance
stagger :: 20–50 ms between items
cap the stagger :: only the first ~10 items, or shrink the step
overlap :: start the next motion before the last ends
shared element :: keeps identity across views
focal motion :: one thing moves most

@@drill
An entrance takes 240 ms. About how long should the exit take, at 75%?
= 180
= 180ms
? 240 × 0.75.

@@drill
With a 30 ms stagger, when does the 6th item start (the first starts at 0)?
= 150
= 150ms
? 5 × 30.

@@exercise Duration from distance and size
Compute durations with a distance model, `base × (distance / reference) ** 0.5`, clamped between a minimum and maximum. Exits get 75% of the entrance time, and desktop takes 85% of mobile. Print a table for several motions.
@@concepts numbers, functions
@@starter
BASE_MS, REFERENCE_PX = 200, 200
MIN_MS, MAX_MS = 100, 450
motions = [("switch thumb", 20), ("tooltip", 40), ("menu", 180), ("bottom sheet", 600), ("full page", 1400)]

def duration(distance, exit=False, desktop=False):
    raise NotImplementedError
@@expected
motion            px  enter  exit  desktop
switch thumb      20    100   100      100
tooltip           40    100   100      100
menu             180    190   142      161
bottom sheet     600    346   260      294
full page       1400    450   397      450
@@requires
def duration
** 0.5
MAX_MS
@@hint
Compute the raw time, multiply by 0.75 for exits and 0.85 for desktop, then clamp with `max(MIN_MS, min(MAX_MS, t))` and round to whole milliseconds.
@@solution
BASE_MS, REFERENCE_PX = 200, 200
MIN_MS, MAX_MS = 100, 450
motions = [("switch thumb", 20), ("tooltip", 40), ("menu", 180), ("bottom sheet", 600), ("full page", 1400)]

def duration(distance, exit=False, desktop=False):
    t = BASE_MS * (distance / REFERENCE_PX) ** 0.5
    if exit:
        t *= 0.75
    if desktop:
        t *= 0.85
    return round(max(MIN_MS, min(MAX_MS, t)))

print(f"{'motion':<14}{'px':>6}{'enter':>7}{'exit':>6}{'desktop':>9}")
for name, px in motions:
    print(f"{name:<14}{px:>6}{duration(px):>7}{duration(px, exit=True):>6}{duration(px, desktop=True):>9}")

@@exercise Build a choreographed timeline
Schedule a card-opening sequence: a container, a header that starts when 60% of the container is done, then list items staggered by a step that shrinks for long lists (capped total stagger), and an action button last. Print each motion's start and end, and the total length.
@@concepts lists, loops, numbers
@@starter
CONTAINER_MS, ITEM_MS, STAGGER_MS, MAX_STAGGER_TOTAL = 260, 180, 40, 240

def timeline(item_count):
    raise NotImplementedError
@@expected
4 items, total 606 ms
  container    0.0 ->  260.0 ms
  header     156.0 ->  356.0 ms
  item 1     236.0 ->  416.0 ms
  item 2     276.0 ->  456.0 ms
  item 3     316.0 ->  496.0 ms
  item 4     356.0 ->  536.0 ms
  button     446.0 ->  606.0 ms
20 items, total 726 ms
  container    0.0 ->  260.0 ms
  header     156.0 ->  356.0 ms
  item 1     236.0 ->  416.0 ms
  item 2     248.6 ->  428.6 ms
  ...
  item 20    476.0 ->  656.0 ms
  button     566.0 ->  726.0 ms
@@requires
def timeline
MAX_STAGGER_TOTAL
min(
@@hint
The container runs 0–260. The header starts at 0.6 × 260 and lasts 200. Items start when the header starts plus 80 ms, with a step of `min(STAGGER_MS, MAX_STAGGER_TOTAL / max(1, n - 1))`. The button starts when the last item is halfway done and lasts 160. Return a list of `(name, start, end)`.
@@solution
CONTAINER_MS, ITEM_MS, STAGGER_MS, MAX_STAGGER_TOTAL = 260, 180, 40, 240

def timeline(item_count):
    events = [("container", 0, CONTAINER_MS)]
    header_start = 0.6 * CONTAINER_MS
    events.append(("header", header_start, header_start + 200))
    step = min(STAGGER_MS, MAX_STAGGER_TOTAL / max(1, item_count - 1))
    first = header_start + 80
    for i in range(item_count):
        start = first + i * step
        events.append((f"item {i + 1}", start, start + ITEM_MS))
    last_start = events[-1][1]
    events.append(("button", last_start + ITEM_MS / 2, last_start + ITEM_MS / 2 + 160))
    return events

for n in (4, 20):
    events = timeline(n)
    print(f"{n} items, total {max(e for _, _, e in events):.0f} ms")
    shown = events if n <= 4 else events[:4] + [("...", 0, 0)] + events[-2:]
    for name, start, end in shown:
        print("  ..." if name == "..." else f"  {name:<9} {start:6.1f} -> {end:6.1f} ms")
