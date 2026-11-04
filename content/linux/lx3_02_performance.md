@@lesson lx3-performance Troubleshooting CPU, Memory, Disk I/O and Network Performance
@@lang bash
@@text
Troubleshooting is the largest part of Linux+ (about 28%). "The server is slow" can mean CPU, memory, disk or network, and each has its own tools and tell-tale numbers. This lesson gives a systematic approach so you find the bottleneck instead of guessing.

## Start broad: the USE method

For every resource (CPU, memory, disk, network), check **Utilisation** (how busy), **Saturation** (how much work is queued), and **Errors**. Then zoom in on the resource that's saturated.

```bash,norun
uptime                 # load averages vs nproc
top / htop             # overview: CPU split, memory, top processes
vmstat 1 5             # r (run queue), b (blocked), si/so (swap), us/sy/id/wa (CPU)
iostat -xz 1 3         # per-disk %util, await, r/s, w/s (sysstat package)
free -h                # memory and swap
sar -u -r -d -n DEV    # history, if sysstat collection is enabled
dmesg -T | tail        # kernel messages: OOM kills, I/O errors, link changes
```

## CPU

`top`'s CPU line: `us` user, `sy` system (kernel), `ni` niced, `id` idle, **`wa` I/O wait** (CPU idle waiting on disk: the problem is storage, not CPU), `st` **steal** (the hypervisor gave your vCPU's time to other VMs: the problem is the host).

High `us` from one process → profile or scale the app. High `sy` → lots of system calls, interrupts, or context switches. Run queue (`vmstat r`) consistently above the core count → CPU saturation. Tools: `mpstat -P ALL` (per core: one core at 100% means a single-threaded bottleneck), `pidstat`, `perf top`.

## Memory

```text
               total        used        free      shared  buff/cache   available
Mem:            15Gi       9.1Gi       410Mi       220Mi       5.9Gi       5.6Gi
Swap:          4.0Gi       1.2Gi       2.8Gi
```

- **"free" being low is normal**: Linux uses spare RAM as **page cache**. Look at **available**.
- Trouble signs: low available, **swap in/out activity** (`vmstat si/so` > 0 continuously: thrashing), and the **OOM killer** in `dmesg`/`journalctl -k` ("Out of memory: Killed process 812 (mysqld)").
- Fix: find the consumer (`ps aux --sort=-rss | head`), fix leaks, set limits (systemd `MemoryMax=`, container `--memory`), add RAM, tune `vm.swappiness`.

## Disk I/O

`iostat -x`: **%util** near 100% (busy), high **await** (ms per I/O; SSDs should be under a few ms), growing queue (`aqu-sz`). `iotop` shows which process. Also check: full file systems (`df -h`, `df -i`), failing disks (`smartctl`, I/O errors in dmesg), RAID degraded (`cat /proc/mdstat`), the wrong I/O scheduler, too many small synchronous writes.

## Network

`ip -s link` (errors/drops), `ss -s` (socket counts), `ss -tan state time-wait | wc -l`, `sar -n DEV`, `iftop`/`nload` (bandwidth per connection), `ethtool eth0` (negotiated speed: a 100 Mb/s link where 1 Gb/s was expected → cable/port), `ping`/`mtr` (latency/loss), `tcpdump` (retransmissions), DNS timing with `dig`. Check MTU mismatches and firewall drops (`nft list ruleset` counters).

## Limits

`ulimit -a` (per shell), `/etc/security/limits.conf` (per user), systemd `LimitNOFILE=` (per service). "Too many open files" → raise `nofile` for the service, check for descriptor leaks (`ls /proc/PID/fd | wc -l`).

## A troubleshooting routine

1. What changed? (deployments, updates, traffic, `journalctl --since "1 hour ago"`).
2. Load and the USE checks above.
3. Identify the process and resource.
4. Fix or mitigate (restart, kill, limit, scale, roll back).
5. Verify and document; add monitoring/alerts so you see it earlier next time.

@@quiz
`top` shows 45% `wa` and low user CPU. Where is the bottleneck?
- CPU
* Disk I/O
- Network
- Memory leak in user space
? I/O wait means CPUs are idle waiting for storage.

@@quiz
A VM shows high `st` (steal) time. What does it mean?
* The hypervisor is giving the vCPU's time to other VMs; the host is overcommitted
- A process is stealing memory
- Swap is being used
- The kernel is under attack
? Raise it with the hosting team or move the VM.

@@quiz
`free -h` shows 300 MiB free but 6 GiB available. Is the server short of memory?
- Yes, free memory is critical
* No: the page cache is using spare RAM and can be reclaimed; check "available"
- Yes, swap is required
- It depends on the CPU
? Low free with high available is normal.

@@quiz
`dmesg` shows "Out of memory: Killed process 812 (mysqld)". What happened?
- The disk filled up
* The kernel's OOM killer ended mysqld to free memory
- MySQL crashed due to a bug
- SELinux blocked MySQL
? Find the memory consumer and set limits.

@@quiz
One core is at 100% while seven are idle, and the app is slow. What does it suggest?
* A single-threaded bottleneck
- Disk saturation
- Network congestion
- Too much RAM
? mpstat -P ALL shows per-core usage.

@@flashcards
USE method :: utilisation, saturation, errors
load average :: compare with nproc
vmstat r :: run queue
vmstat b :: processes blocked on I/O
vmstat si/so :: swap in/out (thrashing)
wa :: CPU waiting for I/O
st :: steal time from the hypervisor
mpstat -P ALL :: per-core CPU
available memory :: what apps can still use
page cache :: spare RAM caching files
OOM killer :: kills processes when memory runs out
iostat -x %util / await :: disk busy / latency
iotop :: I/O per process
/proc/mdstat :: software RAID status
ethtool :: negotiated link speed
ulimit -n :: open file limit
LimitNOFILE :: systemd file limit
journalctl --since :: logs from a time window

@@drill
Which vmstat columns show swap activity?
= si so
= si/so
= si and so
? Swap in and swap out.

@@drill
Which command shows per-disk utilisation and latency?
= iostat -x
= iostat
? From the sysstat package.

@@exercise Diagnose from vmstat
The variable `vm` holds `vmstat 1` samples (header line, then numbers). Average each column over the samples and diagnose: run queue `r` above the core count (`CORES`) → `CPU saturated`; `wa` average over 20 → `disk I/O bottleneck`; `si` or `so` average above 0 → `swapping`; `st` above 10 → `hypervisor steal`. Print the averages for r, b, si, so, us, sy, id, wa, st and each diagnosis, or `no bottleneck`.
@@starter
CORES=4
vm=' r  b   swpd   free   buff  cache   si   so    bi    bo   in   cs us sy id wa st
 6  3 812000 102400  20480 409600   40   85  2400  3100 2100 4800 35 10 15 38  2
 7  4 830000  98000  20480 401000   60  120  2800  2900 2300 5100 32 12 14 40  2
 5  3 845000  95000  20000 398000   55   90  2600  3300 2200 4900 30 11 17 40  2'

echo "$vm"
@@expected
r=6.0 b=3.3 si=51.7 so=98.3 us=32.3 sy=11.0 id=15.3 wa=39.3 st=2.0
CPU saturated (run queue 6 > 4 cores)
disk I/O bottleneck
swapping
@@requires
-v cores="$CORES"
NR == 1
@@hint
On the header line (`NR == 1`), store column names: `for (i = 1; i <= NF; i++) name[i] = $i`. For data lines, sum into `sum[name[i]] += $i`. In END divide by `NR - 1`.
@@solution
CORES=4
vm=' r  b   swpd   free   buff  cache   si   so    bi    bo   in   cs us sy id wa st
 6  3 812000 102400  20480 409600   40   85  2400  3100 2100 4800 35 10 15 38  2
 7  4 830000  98000  20480 401000   60  120  2800  2900 2300 5100 32 12 14 40  2
 5  3 845000  95000  20000 398000   55   90  2600  3300 2200 4900 30 11 17 40  2'

echo "$vm" | awk -v cores="$CORES" '
NR == 1 { for (i = 1; i <= NF; i++) name[i] = $i; next }
{ for (i = 1; i <= NF; i++) sum[name[i]] += $i }
END {
    n = NR - 1
    split("r b si so us sy id wa st", show, " ")
    line = ""
    for (k = 1; k <= 9; k++) { c = show[k]; avg[c] = sum[c] / n; line = line sprintf("%s=%.1f ", c, avg[c]) }
    sub(/ $/, "", line)
    print line
    found = 0
    if (avg["r"] > cores) { print "CPU saturated (run queue " avg["r"] " > " cores " cores)"; found = 1 }
    if (avg["wa"] > 20) { print "disk I/O bottleneck"; found = 1 }
    if (avg["si"] > 0 || avg["so"] > 0) { print "swapping"; found = 1 }
    if (avg["st"] > 10) { print "hypervisor steal"; found = 1 }
    if (!found) print "no bottleneck"
}'

@@exercise Read free -m output
`free -m` output is in `freeout`. Print the percentage of memory that is **available** and of swap that is used (integers), then a verdict: `memory pressure` if available is under 10%, `swap in use: check vmstat si/so` if swap use is over 25%, otherwise `memory OK`. Both warnings can apply.
@@starter
freeout='               total        used        free      shared  buff/cache   available
Mem:           15872        9320         410         220        6142        5730
Swap:           4096        1210        2886'

echo "$freeout"
@@expected
available: 36%, swap used: 29%
swap in use: check vmstat si/so
@@requires
/^Mem:/
/^Swap:/
@@hint
Capture `total` and `available` (`$2` and `$7`) on the Mem line and `$2`, `$3` on the Swap line; print in END.
@@solution
freeout='               total        used        free      shared  buff/cache   available
Mem:           15872        9320         410         220        6142        5730
Swap:           4096        1210        2886'

echo "$freeout" | awk '
/^Mem:/ { avail = $7 / $2 * 100 }
/^Swap:/ { swap = ($2 > 0) ? $3 / $2 * 100 : 0 }
END {
    printf "available: %d%%, swap used: %d%%\n", avail, swap
    ok = 1
    if (avail < 10) { print "memory pressure"; ok = 0 }
    if (swap > 25) { print "swap in use: check vmstat si/so"; ok = 0 }
    if (ok) print "memory OK"
}'
