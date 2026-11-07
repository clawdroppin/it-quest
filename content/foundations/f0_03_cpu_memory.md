@@lesson f0-cpu-memory Inside the CPU and Memory
@@text
The first lesson gave you the parts list. This one opens up the two parts that decide how fast a computer *feels*: the processor and its memory. When you read a laptop spec sheet ("Intel Core i7-1355U, 10 cores, 12 threads, 16 GB LPDDR5") or decide whether more RAM will fix a slow PC, this is the knowledge you use.

## What a CPU actually does

A CPU repeats one loop billions of times per second:

1. **Fetch** the next instruction from memory.
2. **Decode** it ("add these two numbers", "jump to there if zero", "load this value").
3. **Execute** it in an arithmetic unit.
4. **Write back** the result to a register or memory.

The speed of that loop is the **clock speed**, in gigahertz (GHz): 3 GHz is three billion cycles per second. Clock speed alone is a poor comparison between different CPU generations, because newer designs do more work per cycle (**IPC**, instructions per cycle).

## Cores and threads

- A **core** is a complete processor. A 10-core CPU can run 10 things truly at the same time.
- **Hyper-threading** (Intel) or **SMT** (AMD) lets one core run two **threads**, sharing its parts. It helps by 20–30%, not 100%.
- Modern laptop CPUs mix **performance cores** (P-cores, fast, power-hungry) and **efficiency cores** (E-cores, slower, frugal). Apple's M-series chips do the same.

More cores help when there are many things to do at once: compiling code, video exports, virtual machines, a busy server. A single slow Excel macro uses one core, so a CPU with fewer but faster cores may run it quicker.

In Task Manager, a CPU stuck at 100% on **one** process usually means that program is busy or stuck; a CPU at 100% spread across many processes means the machine is genuinely underpowered for the workload (or has malware mining cryptocurrency).

## Architectures: x86 and ARM

- **x86-64** (Intel, AMD): most Windows PCs and servers.
- **ARM**: phones, tablets, Apple Silicon Macs, Raspberry Pi, many cloud servers (AWS Graviton) and newer Windows laptops (Snapdragon X).

Programs are compiled for one architecture. ARM Windows and macOS can run x86 programs through **emulation**, usually a bit slower; drivers can't be emulated, so old printers and VPN clients may not work on ARM laptops. Check before buying them for staff.

## The memory hierarchy

The CPU is much faster than RAM, so there are layers of smaller, faster memory in between:

| Level | Typical size | Time to read | Where |
|---|---|---|---|
| Registers | a few hundred bytes | under 1 ns | inside each core |
| L1 cache | 32–64 KB per core | ~1 ns | inside each core |
| L2 cache | 1–2 MB per core | ~4 ns | next to each core |
| L3 cache | 8–64 MB shared | ~15 ns | on the CPU chip |
| RAM | 8–64 GB | ~80 ns | memory sticks |
| NVMe SSD | 1–4 TB | ~20,000 ns | M.2 slot |
| HDD | 2–20 TB | ~5,000,000 ns | SATA |

If one CPU cycle took one second, reading RAM would take about four minutes, an NVMe SSD most of a day, and a hard disk almost six months. That's why a full RAM that pushes the system onto the disk (swapping) makes everything crawl.

## RAM in practice

- **DDR4** and **DDR5** are the current desktop/laptop generations; they are not interchangeable (different notches).
- **SO-DIMM** sticks are the small laptop size; **DIMM** is the desktop size. Many thin laptops have **soldered** RAM (LPDDR) that can't be upgraded, so buy enough at purchase time.
- **Dual channel**: two matching sticks in the right slots double the memory bandwidth. A single 16 GB stick is noticeably slower than 2 × 8 GB for integrated graphics.
- **ECC** RAM detects and fixes bit errors; servers use it.
- How much? In 2026, 8 GB is the bare minimum for Windows 11 office work, 16 GB is the sensible standard for staff laptops, 32 GB+ for developers, VMs and video work.

## Is it the CPU or the RAM?

Open Task Manager → Performance (Windows) or `htop` (Linux):

| Symptom | Likely bottleneck | Fix |
|---|---|---|
| CPU 100%, memory fine | CPU | find the process; close it, update it, or a faster CPU |
| Memory 90%+, disk busy | RAM (swapping) | close programs, browser tabs; add RAM |
| Disk 100%, CPU and RAM fine | storage | check for a failing HDD, move to SSD, stop indexing/updates |
| Everything low but it's slow | network or the app's server | test the network |

## 32-bit vs 64-bit

A 32-bit system can address at most 2³² bytes = 4 GB of memory. All current systems are 64-bit; Windows 11 doesn't exist in 32-bit. You'll still meet 32-bit applications (installed in `C:\Program Files (x86)`).

@@quiz
A CPU has 8 cores with hyper-threading. How many threads can it run at once?
- 8
* 16
- 4
- 64
? Each core runs two threads with hyper-threading/SMT.

@@quiz
Why does a computer with full RAM become dramatically slower?
* It moves memory to the disk (swapping), and the disk is thousands of times slower than RAM
- RAM gets hotter when full
- The CPU turns itself off
- Full RAM corrupts files
? Look at the memory hierarchy table.

@@quiz
A staff member's thin laptop has 8 GB of soldered LPDDR5 RAM. Can you add a stick?
- Yes, any DDR5 stick fits
* No: soldered memory can't be upgraded; the laptop would need replacing
- Yes, but only SO-DIMM DDR3
- Only with a BIOS update
? Buy enough RAM when ordering laptops with soldered memory.

@@quiz
Why might an old printer driver not work on a new Snapdragon (ARM) Windows laptop?
- ARM laptops have no USB ports
* Drivers are compiled for x86 and can't run under emulation on ARM
- Printers need Intel CPUs
- Windows on ARM has no print spooler
? Applications can be emulated; drivers can't.

@@flashcards
clock speed :: cycles per second, in GHz
IPC :: instructions per cycle; why newer CPUs are faster at the same GHz
core :: an independent processor inside the CPU
thread (hyper-threading/SMT) :: two instruction streams sharing one core
P-core / E-core :: performance and efficiency cores
x86-64 / ARM :: the two main CPU architectures
cache (L1/L2/L3) :: small fast memory between the CPU and RAM
DDR5 :: current RAM generation
SO-DIMM :: laptop RAM size
dual channel :: two matched sticks, double bandwidth
ECC :: error-correcting memory for servers
32-bit limit :: 4 GB of addressable memory

@@drill
How many bytes can a 32-bit address reach? (answer in GB)
= 4
= 4 GB
? 2^32 = 4,294,967,296 bytes.

@@drill
A CPU runs at 3 GHz. How many cycles is that per second? (in billions)
= 3
= 3 billion
? Giga = billion.

@@exercise Bottleneck finder
Write `bottleneck(cpu, mem, disk)` that takes three usage percentages and returns the likely bottleneck as text, following the lesson's table: `"RAM (swapping)"` if memory is at least 90 and disk at least 80; otherwise `"CPU"` if CPU is at least 90; otherwise `"storage"` if disk is at least 90; otherwise `"network or server"`. Print the result for the four sample machines.
@@lang python
@@starter
def bottleneck(cpu, mem, disk):
    raise NotImplementedError

machines = [("PC-014", 100, 45, 10), ("PC-022", 35, 96, 100), ("PC-031", 20, 40, 100), ("PC-040", 12, 30, 5)]
for name, cpu, mem, disk in machines:
    print(name, bottleneck(cpu, mem, disk))
@@expected
PC-014 CPU
PC-022 RAM (swapping)
PC-031 storage
PC-040 network or server
@@requires
def bottleneck
>= 90
@@hint
Check the swapping case first, because a swapping machine also has a busy disk and you don't want it reported as "storage".
@@solution
def bottleneck(cpu, mem, disk):
    if mem >= 90 and disk >= 80:
        return "RAM (swapping)"
    if cpu >= 90:
        return "CPU"
    if disk >= 90:
        return "storage"
    return "network or server"

machines = [("PC-014", 100, 45, 10), ("PC-022", 35, 96, 100), ("PC-031", 20, 40, 100), ("PC-040", 12, 30, 5)]
for name, cpu, mem, disk in machines:
    print(name, bottleneck(cpu, mem, disk))

@@exercise If a CPU cycle took a second
Using the access times from the memory hierarchy table (L1 1 ns, RAM 80 ns, NVMe 20,000 ns, HDD 5,000,000 ns) and a 3 GHz CPU (one cycle = 1/3 ns), scale each time so that **one cycle = one second**, and print it in the most readable unit: seconds under 120, minutes under 120, hours under 48, otherwise days. Round to one decimal place.
@@lang python
@@starter
CYCLE_NS = 1 / 3
times_ns = {"L1 cache": 1, "RAM": 80, "NVMe SSD": 20_000, "HDD": 5_000_000}

def human(seconds):
    raise NotImplementedError

for name, ns in times_ns.items():
    scaled = ns / CYCLE_NS          # seconds in the "one cycle = one second" world
    print(f"{name}: {human(scaled)}")
@@expected
L1 cache: 3.0 seconds
RAM: 4.0 minutes
NVMe SSD: 16.7 hours
HDD: 173.6 days
@@requires
def human
/ 60
@@hint
`if seconds < 120: return f"{seconds:.1f} seconds"`, then divide by 60 for minutes, by 3600 for hours, by 86400 for days.
@@solution
CYCLE_NS = 1 / 3
times_ns = {"L1 cache": 1, "RAM": 80, "NVMe SSD": 20_000, "HDD": 5_000_000}

def human(seconds):
    if seconds < 120:
        return f"{seconds:.1f} seconds"
    if seconds / 60 < 120:
        return f"{seconds / 60:.1f} minutes"
    if seconds / 3600 < 48:
        return f"{seconds / 3600:.1f} hours"
    return f"{seconds / 86400:.1f} days"

for name, ns in times_ns.items():
    scaled = ns / CYCLE_NS
    print(f"{name}: {human(scaled)}")
