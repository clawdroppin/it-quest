@@lesson xp-number_systems Number Systems and Logic Trainer
@@text
The FE exam always has questions on number conversion, two's complement, floating point and logic circuits. Build a drill program that generates random questions of each kind, checks your answers, explains the method when you're wrong, and tracks which kinds you miss most.

**You'll practise:** binary, octal, hex · two's complement · bit shifts · AND/OR/XOR/NAND truth tables

## Milestones

1. **Conversions**: random numbers between decimal, binary and hex.
2. **Two's complement**: 8-bit negative numbers both ways.
3. **Shifts**: logical vs arithmetic shift results.
4. **Logic**: random 2-input expressions and their truth tables.
5. **Stats**: save results and show your weakest question type.

## You're done when

- You can score 18/20 in each category without paper.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Explain two's complement of -45 in 8 bits step by step."*
- *"What's the difference between an arithmetic and a logical right shift?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project number_systems Number Systems and Logic Trainer
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Number Systems and Logic Trainer

The FE exam always has questions on number conversion, two's complement, floating point and logic circuits. Build a drill program that generates random questions of each kind, checks your answers, explains the method when you're wrong, and tracks which kinds you miss most.

## Milestones

- [ ] 1. Conversions: random numbers between decimal, binary and hex.
- [ ] 2. Two's complement: 8-bit negative numbers both ways.
- [ ] 3. Shifts: logical vs arithmetic shift results.
- [ ] 4. Logic: random 2-input expressions and their truth tables.
- [ ] 5. Stats: save results and show your weakest question type.

## Done when

- [ ] You can score 18/20 in each category without paper.
@@file drill.py
"""FE number system drills: python drill.py"""
import random


def ask_binary() -> bool:
    n = random.randint(0, 255)
    answer = input(f"{n} in binary (8 bits)? ").strip()
    ok = answer == format(n, "08b")
    print("✔" if ok else f"✘ {format(n, '08b')}  (divide by 2, read the remainders upwards)")
    return ok


if __name__ == "__main__":
    score = sum(ask_binary() for _ in range(5))
    print(f"{score}/5")
    # TODO: hex, two's complement, shifts, logic gates, saved statistics
