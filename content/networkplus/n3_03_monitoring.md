@@lesson n3-monitoring Network Monitoring, Metrics and Logs
@@text
Domain 3 (network operations) asks how you know the network is healthy before users call. That means collecting the right data (SNMP, flows, logs, packet captures), knowing which metrics matter, setting baselines and alerting on deviations.

## SNMP

- **Manager** (the NMS: Zabbix, PRTG, LibreNMS, SolarWinds) polls **agents** on devices over UDP **161**; agents send unsolicited **traps** (and acknowledged **informs**) to UDP **162**.
- Data is organised in the **MIB** (management information base); each value has an **OID** (object identifier) like `1.3.6.1.2.1.2.2.1.10` (ifInOctets: bytes received on an interface).
- Versions: **v1/v2c** use plaintext **community strings** (`public` = read-only default, change it!); **v3** adds **authentication and encryption** (authPriv). Use v3.
- Interface counters are **cumulative**: to get bandwidth you take two samples and divide the difference by time. 64-bit counters (ifHCInOctets) avoid wrapping on fast links.

## Flow data

**NetFlow**, **IPFIX**, **sFlow** export summaries of conversations: source/destination IP and port, protocol, bytes, packets, time. Answers "who is using the bandwidth?" and spots unusual traffic (a server suddenly sending gigabytes to an unknown country). Flow is cheaper than full packet capture.

## Packet capture

**Wireshark**/**tcpdump** capture full packets: the deepest view, for protocol problems. Capture from a **SPAN/mirror port** or a **network TAP** (a hardware device that copies traffic passively, never drops). Use capture filters (`host 10.0.0.5 and port 443`) to limit data. Captures may contain sensitive data: handle them carefully.

## Logs

- **Syslog** (UDP/TCP **514**, TLS 6514): devices send messages to a central server.
- Severity levels 0–7: **0 Emergency, 1 Alert, 2 Critical, 3 Error, 4 Warning, 5 Notice, 6 Informational, 7 Debug** ("Every Awesome Cisco Engineer Will Need Ice cream Daily"). Lower is more severe. Logging level 4 means levels 0–4 are sent.
- **SIEM**: collects and correlates logs from everything, alerts on patterns.
- Log types: traffic logs, audit logs (who changed config), system logs.
- **Log retention** and time sync (NTP) make logs usable.

## Key metrics

| Metric | Meaning |
|---|---|
| **Bandwidth** | capacity of the link |
| **Throughput** | data actually transferred |
| **Utilisation** | throughput ÷ bandwidth (sustained > 70–80% means congestion) |
| **Latency** | delay (one-way or round trip) |
| **Jitter** | variation in latency (bad for voice/video) |
| **Packet loss** | % of packets that never arrive |
| **Errors** | CRC, runts, giants, input/output errors |
| **Discards/drops** | packets dropped because buffers were full |
| **CPU/memory/temperature** | device health |
| **Uptime/availability** | % of time the service worked |

Availability in "nines": **99.9%** ≈ 8.77 h of downtime per year; **99.99%** ≈ 52.6 min; **99.999%** ≈ 5.26 min.

## Baselines and alerts

A **baseline** is what "normal" looks like (Monday 9 a.m. utilisation, typical latency). Alert on **thresholds** derived from it and on **anomalies**. Too many alerts → alert fatigue, so tune them.

## Other operations tools

- **APIs** and **streaming telemetry** (push-based, more frequent than SNMP polling).
- **Port scanners** and **network discovery** to keep the inventory accurate.
- **Configuration monitoring**: back up configs, detect drift.
- **Environmental sensors**: temperature, humidity, power, water in the server room.
- **Dashboards** and **reports** for capacity planning (when will this link hit 80%?).

@@quiz
Which SNMP version should be used to authenticate and encrypt monitoring traffic?
- v1
- v2c
* v3
- Any version with community "private"
? v1/v2c send community strings in plaintext.

@@quiz
Which syslog severity is more serious: 2 or 5?
* 2 (Critical)
- 5 (Notice)
- They're the same
- It depends on the vendor
? Lower numbers are more severe.

@@quiz
A WAN link feels slow and you need to know which hosts and applications are using it. Which data source is best?
- SNMP uptime
* NetFlow/IPFIX flow records
- Syslog severity 7
- ping
? Flows show who talks to whom and how much.

@@quiz
Users complain voice calls are choppy, but average latency is fine. Which metric should you check?
- Bandwidth
* Jitter
- Uptime
- MTU
? Variation in delay breaks real-time audio.

@@quiz
An SLA promises 99.99% availability. About how much downtime is that per year?
- 8.8 hours
* 53 minutes
- 5 minutes
- 3.6 days
? 0.01% of 525,960 minutes.

@@flashcards
SNMP ports :: 161 polling, 162 traps
MIB / OID :: data tree / one value's identifier
community string :: SNMPv1/v2c plaintext password
SNMPv3 :: authentication and encryption
NetFlow / IPFIX :: conversation summaries
TAP :: passive hardware traffic copy
syslog 0 :: Emergency
syslog 7 :: Debug
syslog port :: 514 (TLS 6514)
SIEM :: log collection and correlation
utilisation :: throughput ÷ bandwidth
jitter :: variation in latency
99.9% :: about 8.8 h downtime/year
99.999% :: about 5 min downtime/year
baseline :: normal performance for comparison
streaming telemetry :: devices push metrics

@@drill
On which UDP port does an SNMP manager receive traps?
= 162
? Polling uses 161.

@@drill
Which syslog severity number is Warning?
= 4
? 0 Emergency ... 7 Debug.

@@exercise Calculate interface utilisation from SNMP counters
Two SNMP polls 300 seconds apart give `ifHCInOctets` and `ifHCOutOctets` for each interface. Compute in and out throughput in Mb/s (octets × 8 ÷ seconds ÷ 1,000,000), utilisation as a percentage of interface speed (the higher of in and out), and flag above 80% as `CONGESTED` and above 60% as `WATCH`. Print one line per interface, rounded to 1 decimal.
@@lang python
@@starter
INTERVAL = 300
interfaces = {
    "Gi1/0/1 (uplink)": {"speed_mbps": 1000, "in": (10_000_000_000, 41_500_000_000), "out": (5_000_000_000, 12_500_000_000)},
    "Gi1/0/2 (server)": {"speed_mbps": 1000, "in": (2_000_000_000, 2_600_000_000), "out": (8_000_000_000, 32_000_000_000)},
    "Te1/0/1 (core)": {"speed_mbps": 10000, "in": (900_000_000_000, 1_050_000_000_000), "out": (700_000_000_000, 820_000_000_000)},
    "Gi1/0/9 (printer)": {"speed_mbps": 100, "in": (1_000_000, 1_900_000), "out": (500_000, 800_000)},
}
@@expected
Gi1/0/1 (uplink): in 840.0 Mb/s, out 200.0 Mb/s, 84.0% CONGESTED
Gi1/0/2 (server): in 16.0 Mb/s, out 640.0 Mb/s, 64.0% WATCH
Te1/0/1 (core): in 4000.0 Mb/s, out 3200.0 Mb/s, 40.0% OK
Gi1/0/9 (printer): in 0.0 Mb/s, out 0.0 Mb/s, 0.0% OK
@@requires
INTERVAL
* 8
@@hint
`mbps = (after - before) * 8 / INTERVAL / 1_000_000`.
@@solution
INTERVAL = 300
interfaces = {
    "Gi1/0/1 (uplink)": {"speed_mbps": 1000, "in": (10_000_000_000, 41_500_000_000), "out": (5_000_000_000, 12_500_000_000)},
    "Gi1/0/2 (server)": {"speed_mbps": 1000, "in": (2_000_000_000, 2_600_000_000), "out": (8_000_000_000, 32_000_000_000)},
    "Te1/0/1 (core)": {"speed_mbps": 10000, "in": (900_000_000_000, 1_050_000_000_000), "out": (700_000_000_000, 820_000_000_000)},
    "Gi1/0/9 (printer)": {"speed_mbps": 100, "in": (1_000_000, 1_900_000), "out": (500_000, 800_000)},
}

def mbps(before_after):
    before, after = before_after
    return (after - before) * 8 / INTERVAL / 1_000_000

for name, i in interfaces.items():
    rx, tx = mbps(i["in"]), mbps(i["out"])
    util = max(rx, tx) / i["speed_mbps"] * 100
    flag = "CONGESTED" if util > 80 else "WATCH" if util > 60 else "OK"
    print(f"{name}: in {rx:.1f} Mb/s, out {tx:.1f} Mb/s, {util:.1f}% {flag}")

@@exercise Filter and summarise syslog
Parse syslog lines of the form `<PRI>timestamp host message`. The severity is `PRI % 8` and the facility is `PRI // 8`. Keep only messages at or more severe than the configured `LEVEL`, print them with their severity name, then print a count per host of the kept messages, most first.
@@lang python
@@starter
SEVERITY = ["Emergency", "Alert", "Critical", "Error", "Warning", "Notice", "Informational", "Debug"]
LEVEL = 4
lines = [
    "<187>Nov 04 09:12:01 core-sw1 %LINK-3-UPDOWN: Interface Te1/0/1, changed state to down",
    "<190>Nov 04 09:12:05 core-sw1 %SYS-6-LOGGINGHOST: logging to 10.0.9.5 started",
    "<188>Nov 04 09:13:44 fw01 High CPU 91% for 5 minutes",
    "<185>Nov 04 09:14:02 fw01 Power supply 2 failed",
    "<191>Nov 04 09:14:10 ap-3f debug: client roam",
    "<187>Nov 04 09:15:30 core-sw1 %LINK-3-UPDOWN: Interface Te1/0/1, changed state to up",
    "<189>Nov 04 09:16:00 access-sw4 %SYS-5-CONFIG_I: Configured from console by admin",
]
@@expected
[Error] 09:12:01 core-sw1: %LINK-3-UPDOWN: Interface Te1/0/1, changed state to down
[Warning] 09:13:44 fw01: High CPU 91% for 5 minutes
[Alert] 09:14:02 fw01: Power supply 2 failed
[Error] 09:15:30 core-sw1: %LINK-3-UPDOWN: Interface Te1/0/1, changed state to up
core-sw1: 2
fw01: 2
@@requires
% 8
Counter(
@@hint
`pri = int(line[1:line.index(">")])`; the host is the fourth whitespace-separated field after the `>`.
@@solution
from collections import Counter

SEVERITY = ["Emergency", "Alert", "Critical", "Error", "Warning", "Notice", "Informational", "Debug"]
LEVEL = 4
lines = [
    "<187>Nov 04 09:12:01 core-sw1 %LINK-3-UPDOWN: Interface Te1/0/1, changed state to down",
    "<190>Nov 04 09:12:05 core-sw1 %SYS-6-LOGGINGHOST: logging to 10.0.9.5 started",
    "<188>Nov 04 09:13:44 fw01 High CPU 91% for 5 minutes",
    "<185>Nov 04 09:14:02 fw01 Power supply 2 failed",
    "<191>Nov 04 09:14:10 ap-3f debug: client roam",
    "<187>Nov 04 09:15:30 core-sw1 %LINK-3-UPDOWN: Interface Te1/0/1, changed state to up",
    "<189>Nov 04 09:16:00 access-sw4 %SYS-5-CONFIG_I: Configured from console by admin",
]

hosts = Counter()
for line in lines:
    end = line.index(">")
    pri = int(line[1:end])
    severity = pri % 8
    if severity > LEVEL:
        continue
    month, day, time, host, message = line[end + 1:].split(" ", 4)
    hosts[host] += 1
    print(f"[{SEVERITY[severity]}] {time} {host}: {message}")
for host, count in hosts.most_common():
    print(f"{host}: {count}")
