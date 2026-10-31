@@lesson f0-hardware How a Computer Works
@@text
Every IT job starts here. When a user says "my computer is slow", you need a picture of what's inside to know where to look.

## The main parts

| Part | Job | Analogy |
|---|---|---|
| **CPU** (processor) | runs instructions, billions per second | the cook |
| **RAM** (memory) | holds the programs and data in use *right now*; emptied at power off | the kitchen counter |
| **Storage** (SSD, HDD) | keeps files and programs when the power is off | the fridge and pantry |
| **Motherboard** | connects everything; has the **firmware** (BIOS/UEFI) | the kitchen itself |
| **GPU** | draws the screen; also used for AI and video work | a pastry specialist |
| **PSU** (power supply) | turns wall power into the voltages parts need | the gas line |
| **NIC** (network card) | wired (Ethernet) or wireless (Wi-Fi) networking | the delivery door |

"Slow" usually means one of these is the **bottleneck**: CPU at 100%, RAM full (the system *swaps* to disk, which is much slower), a nearly full or failing disk, or a slow network. Task Manager (Windows) or `top` (Linux) shows you which in seconds.

## Bits, bytes and units

Computers store everything as **bits**: 0 or 1. Eight bits are a **byte**, enough for one English letter.

| Unit | Size | Example |
|---|---|---|
| KB | about a thousand bytes | a short text file |
| MB | about a million | a photo, a song |
| GB | about a billion | RAM (16 GB), a film |
| TB | about a trillion | a hard disk |

Two traps that come up on exams and in real tickets:

- Disk makers use **1000** (1 TB = 1,000,000,000,000 bytes); Windows counts in **1024**, so a "1 TB" disk shows as about **931 GB**. Nothing is missing.
- Network speeds are in **bits** per second: a 1 **Gbps** line moves at most about 125 **MB** (bytes) per second.

## Binary and hexadecimal

Binary is base 2: each position is worth double the one to its right.

| 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |

`11000000` = 128 + 64 = **192**, the first number of the private IP address `192.168.1.10`. You'll do this a lot in networking.

**Hexadecimal** (base 16, digits 0–9 then A–F) is a short way to write bytes: one hex digit is 4 bits, so a byte is two hex digits. `FF` = 255. MAC addresses (`3C:52:82:1A:9F:04`), IPv6 addresses and colours (`#FF8800`) are written in hex.

## Storage types

- **HDD**: spinning platters; cheap, large, slow, fragile. Clicking noises mean back it up *now*.
- **SSD** (SATA): no moving parts, several times faster.
- **NVMe SSD**: plugs into an M.2 slot and talks over PCIe, many times faster again.
- **RAID** combines disks: RAID 1 mirrors (survives one disk dying), RAID 0 stripes (fast, no safety), RAID 5/6 use parity, RAID 10 mirrors and stripes. **RAID is not a backup**: delete a file and it's deleted on every disk.

@@quiz
A user's PC is very slow. Task Manager shows memory at 98% and the disk at 100%. What's the most likely cause?
* Not enough RAM, so Windows is swapping memory to the disk
- The CPU is too slow
- The network cable is loose
- The monitor resolution is too high
? When RAM is full, the OS moves memory pages to the disk (the page file), and the disk becomes the bottleneck.

@@quiz
A customer complains their new 2 TB drive shows only about 1.81 TB in Windows. What do you tell them?
* That's normal: the maker counts 1 TB as 10^12 bytes, Windows counts in powers of 1024
- The drive is faulty and should be replaced
- Windows reserves 10% for itself
- It needs to be formatted as FAT32
? 2 × 10^12 / 1024^4 ≈ 1.82. Same bytes, different units.

@@quiz
What is binary `00001010` in decimal?
- 8
* 10
- 12
- 1010
? 8 + 2 = 10.

@@quiz
Which statement about RAID 1 is true?
* It mirrors data to two disks, so it survives one disk failing, but it is not a backup
- It doubles the capacity of two disks
- It protects against deleted files
- It needs at least five disks
? Mirroring protects against hardware failure only; deletions, ransomware and fire hit both copies.

@@quiz
Your internet plan is 100 Mbps. About how fast can a large download go?
* About 12 MB per second
- About 100 MB per second
- About 800 MB per second
- About 1 MB per second
? 100 megabits ÷ 8 = 12.5 megabytes, a little less after overhead.

@@flashcards
CPU :: Runs instructions; the "brain"
RAM :: Fast, temporary memory for running programs; emptied when powered off
SSD / HDD :: Storage without / with moving parts
NVMe :: SSDs on the PCIe bus (M.2 slot): much faster than SATA
UEFI / BIOS :: Motherboard firmware that starts the computer
byte :: 8 bits
1 Gbps :: About 125 MB per second
hexadecimal :: Base 16 (0-9, A-F); one digit = 4 bits
RAID 1 :: Mirroring; survives one disk failure
RAID 0 :: Striping; faster, no redundancy
swap / page file :: Disk space used when RAM is full: slow

@@drill
What is binary `11111111` in decimal?
= 255
? 128+64+32+16+8+4+2+1. The largest value one byte can hold.

@@drill
What is decimal 192 in 8-bit binary?
= 11000000
= 1100 0000
? 128 + 64.

@@exercise Binary converter
Write a function `to_binary(n)` that returns the 8-bit binary text of a number 0–255 **without** using `bin()` or `format()`: repeatedly take `n % 2` and divide by 2. Print the binary of 10, 192 and 255.
@@lang python
@@starter
def to_binary(n):
    bits = ""
    # build the string from the right: n % 2 is the last bit
    raise NotImplementedError

for n in [10, 192, 255]:
    print(n, to_binary(n))
@@expected
10 00001010
192 11000000
255 11111111
@@requires
% 2
@@hint
Loop 8 times: `bits = str(n % 2) + bits`, then `n = n // 2`.
@@solution
def to_binary(n):
    bits = ""
    for _ in range(8):
        bits = str(n % 2) + bits
        n = n // 2
    return bits

for n in [10, 192, 255]:
    print(n, to_binary(n))
