@@lesson fe3-system-calcs テクノロジ系 Calculations: CPU, Memory, Disks, Networks and Data Sizes
@@text
Several 科目A questions every year are short calculations about computer systems: how many instructions per second a CPU executes, the effective access time with a cache, how long a disk access or a file transfer takes, and how much storage an image or a RAID array needs. The formulas are few and the questions repeat with new numbers. This lesson lists them with their Japanese names.

## CPU performance

- **クロック周波数** (clock frequency) f; **クロックサイクル時間** = 1 / f (2 GHz → 0.5 ns).
- **CPI** (cycles per instruction): average cycles per instruction, weighted by the **命令ミックス** (instruction mix).
- **MIPS** = f / (CPI × 10⁶). A 2 GHz CPU with CPI 4 → 500 MIPS.
- Execution time = instructions × CPI / f.
- **パイプライン** (pipeline): k stages, n instructions → (k + n − 1) cycles instead of k × n (ideal).

## Memory and cache

- **実効アクセス時間** (effective access time) = **h × cache time + (1 − h) × main memory time**, where h is the **ヒット率** (hit rate).
- Writes: **ライトスルー** (write-through: cache and memory together) vs **ライトバック** (write-back: memory updated when the line is evicted).
- **Interleaving** splits memory into banks for parallel access.

## Disks

- Average access time = **平均シーク時間** (seek) + **平均回転待ち時間** (rotational latency = half a rotation) + **データ転送時間** (transfer).
- 6,000 rpm = 100 rotations/s → one rotation 10 ms → average latency **5 ms**.
- **RAID**: RAID0 (striping, capacity n × size), RAID1 (mirroring, capacity n/2), RAID5 (parity, capacity (n − 1) × size, survives 1 failure), RAID6 ((n − 2) × size, 2 failures).

## Networks

- Transfer time = **data size (bits) / (line speed × 伝送効率 (transmission efficiency))**. 
- Units: line speeds use k = 10³, M = 10⁶; file sizes in bytes × 8 for bits.
- Example: 10 MB over 100 Mbps at 80% efficiency → 80 × 10⁶ bits / (80 × 10⁶ bps) = **1 s**.
- **回線利用率** (line utilisation) = traffic in bits per second / line speed.

## Data sizes

- Image: **pixels × bits per pixel**. 1,920 × 1,080 at 24-bit colour (フルカラー) = 6,220,800 bytes ≈ 6.2 MB.
- Audio (PCM): **sampling frequency × quantisation bits × channels × seconds**. CD: 44.1 kHz × 16 bits × 2 × 60 s ≈ 10.6 MB per minute.
- Video: frame size × frames per second × seconds (before compression).

## Reliability (recap from fe3_01)

MTBF / (MTBF + MTTR); series multiply; parallel 1 − (1 − A)(1 − B).

@@quiz
A 2.4 GHz CPU executes instructions with an average CPI of 3. What is its MIPS rating?
- 400
* 800
- 2,400
- 7,200
? 2.4 × 10⁹ / (3 × 10⁶).

@@quiz
Cache access is 10 ns, main memory 60 ns, hit rate 90%. What is the effective access time?
- 10 ns
* 15 ns
- 35 ns
- 54 ns
? 0.9 × 10 + 0.1 × 60.

@@quiz
A disk spins at 6,000 rpm. What is the average rotational latency?
- 10 ms
* 5 ms
- 6 ms
- 1 ms
? Half of one 10 ms rotation.

@@quiz
How long does a 30 MB file take over a 100 Mbps line at 60% efficiency?
- 0.3 s
- 2.4 s
* 4 s
- 5 s
? 240 × 10⁶ bits / 60 × 10⁶ bps.

@@quiz
Five 2 TB disks form a RAID5 array. What is the usable capacity?
- 10 TB
* 8 TB
- 6 TB
- 5 TB
? One disk's worth for parity.

@@quiz
A 4-stage pipeline executes 10 instructions. How many cycles does it need (ideal)?
- 40
* 13
- 10
- 14
? k + n − 1.

@@flashcards
cycle time :: 1 / clock frequency
MIPS :: f / (CPI × 10⁶)
execution time :: instructions × CPI / f
pipeline cycles :: k + n − 1
実効アクセス時間 :: h × cache + (1 − h) × memory
ライトスルー :: write cache and memory together
ライトバック :: write memory on eviction
回転待ち時間 :: half a rotation
6,000 rpm latency :: 5 ms
access time :: seek + latency + transfer
RAID5 capacity :: (n − 1) × size
RAID6 capacity :: (n − 2) × size
RAID1 capacity :: half
transfer time :: bits / (speed × efficiency)
回線利用率 :: traffic bps / line speed
image size :: pixels × bits per pixel
PCM audio :: Hz × bits × channels × s
フルカラー :: 24 bits per pixel

@@drill
What is the average rotational latency in ms of a 7,500 rpm disk?
= 4
= 4 ms
= 4ms
? 125 rotations/s → 8 ms per rotation → 4 ms.

@@drill
How many cycles does an ideal 5-stage pipeline need for 20 instructions?
= 24
? 5 + 20 − 1.

@@exercise CPU, cache and pipeline calculations
Compute CPI from an instruction mix, then MIPS and execution time for each CPU; the effective access time for cache configurations (including a two-level cache: L1 hit, else L2 hit, else memory); and pipeline cycle counts.
@@lang python
@@starter
mix = [("arithmetic", 0.5, 2), ("load/store", 0.3, 5), ("branch", 0.2, 3)]
cpus = [("A", 2.0e9), ("B", 3.2e9)]
INSTRUCTIONS = 1.2e9
caches = [("L1 only", [(0.9, 10)], 80), ("L1 95%", [(0.95, 10)], 80), ("L1 + L2", [(0.9, 2), (0.8, 12)], 80)]
pipelines = [(4, 10), (5, 100), (6, 50)]

def cpi(mix):
    raise NotImplementedError
@@expected
CPI = 3.1
CPU A: 2.0 GHz -> 645 MIPS, 1.2e+09 instructions in 1.86 s
CPU B: 3.2 GHz -> 1,032 MIPS, 1.2e+09 instructions in 1.16 s
L1 only: effective access 17.0 ns
L1 95%: effective access 13.5 ns
L1 + L2: effective access 4.4 ns
4-stage pipeline, 10 instructions: 13 cycles (vs 40 without pipelining)
5-stage pipeline, 100 instructions: 104 cycles (vs 500 without pipelining)
6-stage pipeline, 50 instructions: 55 cycles (vs 300 without pipelining)
@@requires
def cpi
def effective_access
1e6
@@hint
CPI = Σ share × cycles. For multi-level caches: `time = 0; remaining = 1.0; for hit, t in levels: time += remaining * hit * t; remaining *= 1 - hit`, then add `remaining * memory`. This is the "hierarchical" model where each level's time is paid only by accesses that stop there.
@@solution
mix = [("arithmetic", 0.5, 2), ("load/store", 0.3, 5), ("branch", 0.2, 3)]
cpus = [("A", 2.0e9), ("B", 3.2e9)]
INSTRUCTIONS = 1.2e9
caches = [("L1 only", [(0.9, 10)], 80), ("L1 95%", [(0.95, 10)], 80), ("L1 + L2", [(0.9, 2), (0.8, 12)], 80)]
pipelines = [(4, 10), (5, 100), (6, 50)]

def cpi(mix):
    return sum(share * cycles for _, share, cycles in mix)

def effective_access(levels, memory):
    time, remaining = 0.0, 1.0
    for hit, t in levels:
        time += remaining * hit * t
        remaining *= 1 - hit
    return time + remaining * memory

c = cpi(mix)
print(f"CPI = {c:.1f}")
for name, f in cpus:
    mips = f / (c * 1e6)
    seconds = INSTRUCTIONS * c / f
    print(f"CPU {name}: {f / 1e9:.1f} GHz -> {mips:,.0f} MIPS, {INSTRUCTIONS:.1e} instructions in {seconds:.2f} s")
for name, levels, memory in caches:
    print(f"{name}: effective access {effective_access(levels, memory):.1f} ns")
for k, n in pipelines:
    print(f"{k}-stage pipeline, {n} instructions: {k + n - 1} cycles (vs {k * n} without pipelining)")

@@exercise Transfer times, data sizes and RAID capacity
Compute transfer times for files over lines with given efficiency, line utilisation for a traffic load, sizes of images and audio, and usable capacity of RAID arrays. Use 10⁶ for M in line speeds and in MB here (as the exam usually states).
@@lang python
@@starter
transfers = [("backup", 600, 100, 0.8), ("photo", 6, 10, 0.5), ("video", 1500, 1000, 0.75)]
traffic = (400, 2_000, 50)
images = [("Full HD", 1920, 1080, 24), ("4K", 3840, 2160, 24), ("HD 256 colours", 1280, 720, 8)]
audio = [("CD 1 min", 44_100, 16, 2, 60), ("voice 10 min", 8_000, 8, 1, 600)]
raids = [("RAID0", 4, 2), ("RAID1", 2, 4), ("RAID5", 5, 4), ("RAID6", 6, 4)]

def transfer_seconds(mb, mbps, efficiency):
    raise NotImplementedError
@@expected
backup: 600 MB over 100 Mbps at 80% -> 60.0 s
photo: 6 MB over 10 Mbps at 50% -> 9.6 s
video: 1500 MB over 1000 Mbps at 75% -> 16.0 s
line utilisation: 400 msg/s x 2000 B on 50 Mbps = 12.8%
Full HD image: 6,220,800 bytes = 6.22 MB
4K image: 24,883,200 bytes = 24.88 MB
HD 256 colours image: 921,600 bytes = 0.92 MB
CD 1 min: 10.58 MB
voice 10 min: 4.80 MB
RAID0 with 4 x 2 TB: 8 TB usable
RAID1 with 2 x 4 TB: 4 TB usable
RAID5 with 5 x 4 TB: 16 TB usable
RAID6 with 6 x 4 TB: 16 TB usable
@@requires
def transfer_seconds
def raid_capacity
* 8
@@hint
Seconds = `mb * 8 / (mbps * efficiency)`. Utilisation = messages per second × bytes × 8 / line bps: `traffic` is `(messages per second, bytes per message, line Mbps)`. RAID: 0 → n × size, 1 → n/2 × size, 5 → (n − 1) × size, 6 → (n − 2) × size.
@@solution
transfers = [("backup", 600, 100, 0.8), ("photo", 6, 10, 0.5), ("video", 1500, 1000, 0.75)]
traffic = (400, 2_000, 50)
images = [("Full HD", 1920, 1080, 24), ("4K", 3840, 2160, 24), ("HD 256 colours", 1280, 720, 8)]
audio = [("CD 1 min", 44_100, 16, 2, 60), ("voice 10 min", 8_000, 8, 1, 600)]
raids = [("RAID0", 4, 2), ("RAID1", 2, 4), ("RAID5", 5, 4), ("RAID6", 6, 4)]

def transfer_seconds(mb, mbps, efficiency):
    return mb * 8 / (mbps * efficiency)

def raid_capacity(level, disks, size):
    return {"RAID0": disks, "RAID1": disks / 2, "RAID5": disks - 1, "RAID6": disks - 2}[level] * size

for name, mb, mbps, eff in transfers:
    print(f"{name}: {mb} MB over {mbps} Mbps at {eff:.0%} -> {transfer_seconds(mb, mbps, eff):.1f} s")

per_second, size, line = traffic
utilisation = per_second * size * 8 / (line * 1e6)
print(f"line utilisation: {per_second} msg/s x {size} B on {line} Mbps = {utilisation:.1%}")

for name, w, h, bpp in images:
    size_bytes = w * h * bpp / 8
    print(f"{name} image: {size_bytes:,.0f} bytes = {size_bytes / 1e6:.2f} MB")
for name, hz, bits, channels, seconds in audio:
    size_bytes = hz * bits * channels * seconds / 8
    print(f"{name}: {size_bytes / 1e6:.2f} MB")
for level, disks, size in raids:
    print(f"{level} with {disks} x {size} TB: {raid_capacity(level, disks, size):g} TB usable")
