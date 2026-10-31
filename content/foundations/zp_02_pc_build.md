@@lesson xp-pc_build PC Build Compatibility Checker
@@text
Before you buy parts, check they fit together. Describe a build in build.json (CPU, motherboard, RAM, GPU, PSU, case) and the checker tells you: does the CPU socket match the board, is the RAM the right generation, is the PSU big enough with headroom, does the GPU fit in the case.

**You'll practise:** hardware basics: sockets, RAM generations, form factors, power · JSON · Python dicts and if statements

## Milestones

1. **Parts**: fill parts.json with a few real CPUs, boards, RAM kits, GPUs, PSUs and cases.
2. **Socket and RAM**: compare the CPU's socket and RAM type with the board's.
3. **Power**: add up typical watts and recommend a PSU with 30% headroom.
4. **Size**: check the board's form factor and the GPU length against the case.
5. **Report**: a clear list of ✔ and ✘ lines with the reason for each problem.

## You're done when

- A compatible build gets all ✔ and each kind of mistake is explained in plain words.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"What's the difference between ATX, micro-ATX and mini-ITX?"*
- *"How much PSU headroom is sensible, and why?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project pc_build PC Build Compatibility Checker
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# PC Build Compatibility Checker

Before you buy parts, check they fit together. Describe a build in build.json (CPU, motherboard, RAM, GPU, PSU, case) and the checker tells you: does the CPU socket match the board, is the RAM the right generation, is the PSU big enough with headroom, does the GPU fit in the case.

## Milestones

- [ ] 1. Parts: fill parts.json with a few real CPUs, boards, RAM kits, GPUs, PSUs and cases.
- [ ] 2. Socket and RAM: compare the CPU's socket and RAM type with the board's.
- [ ] 3. Power: add up typical watts and recommend a PSU with 30% headroom.
- [ ] 4. Size: check the board's form factor and the GPU length against the case.
- [ ] 5. Report: a clear list of ✔ and ✘ lines with the reason for each problem.

## Done when

- [ ] A compatible build gets all ✔ and each kind of mistake is explained in plain words.
@@file check_build.py
"""Check a PC build for compatibility: python check_build.py build.json"""
import json
import sys


def check(build: dict) -> list[str]:
    """Problems with the build, as plain sentences. Empty means it's compatible."""
    problems = []
    if build["cpu"]["socket"] != build["motherboard"]["socket"]:
        problems.append(f"The {build['cpu']['name']} needs socket {build['cpu']['socket']}, "
                        f"but the board has {build['motherboard']['socket']}.")
    # TODO: RAM type, power with headroom, form factor, GPU length
    return problems


if __name__ == "__main__":
    build = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "build.json", encoding="utf-8"))
    problems = check(build)
    print("\n".join("✘ " + p for p in problems) or "✔ Everything fits together.")
@@file build.json
{
  "cpu": { "name": "Ryzen 5 7600", "socket": "AM5", "ram": "DDR5", "watts": 65 },
  "motherboard": { "name": "B650M board", "socket": "AM5", "ram": "DDR5", "form_factor": "micro-ATX" },
  "ram": { "name": "32 GB DDR5-6000", "type": "DDR5" },
  "gpu": { "name": "RTX 4060", "watts": 115, "length_mm": 240 },
  "psu": { "name": "550 W 80+ Gold", "watts": 550 },
  "case": { "name": "Compact case", "form_factors": ["micro-ATX", "mini-ITX"], "max_gpu_mm": 320 }
}
