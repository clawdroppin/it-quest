@@lesson cp0-qos-wireless-deep QoS and Wireless in Depth: DSCP, Queuing, Shaping, RF Math and Roaming
@@text
ENCOR's architecture and infrastructure domains test QoS with surprisingly precise questions: which DSCP value is AF31 or what the token bucket's Tc is. Wireless (how many dBm a 100 mW radio puts out, whether a link budget closes) left ENCOR in version 1.2 and now belongs to the wireless concentration exams, but it is still everyday enterprise knowledge. This lesson gives the rules and numbers, then the design concepts (trust boundaries, queuing, roaming) that sit around them.

## Marking

- **DSCP** (6 bits in the IP header): **EF = 46** (voice), **AFxy = 8x + 2y** (x = class 1–4, y = drop precedence 1–3: AF41 = 34, AF31 = 26, AF21 = 18, AF11 = 10), **CSx = 8x** (CS3 = 24 call signalling, CS6 = 48 routing), default/best effort 0.
- Layer 2: 802.1p **CoS** (3 bits in the 802.1Q tag, 0–7; voice = 5). IP Precedence = the top 3 bits of DSCP.
- **Trust boundary**: as close to the source as possible (the access switch port, trusting only an IP phone's markings via CDP/LLDP); re-mark untrusted traffic.

## Queuing and congestion

- **LLQ** (low latency queue): strict priority for voice/video, **policed** so it can't starve others (typically ≤ 33% of the link).
- **CBWFQ**: bandwidth guarantees per class during congestion.
- **WRED**: drops lower-priority TCP packets early as queues fill (avoids tail drop and TCP global synchronisation).
- Voice targets: one-way latency **≤ 150 ms**, jitter **≤ 30 ms**, loss **≤ 1%**.

## Policing vs shaping

| | Policing | Shaping |
|---|---|---|
| Excess traffic | dropped or re-marked | **buffered** and sent later |
| Direction | inbound or outbound | outbound |
| Effect | no added delay, TCP retransmits | adds delay, smooth output |
| Typical use | provider edge, enforcing contracts | customer side, matching a sub-rate WAN |

Token bucket: **CIR** (committed rate), **Bc** (committed burst, bits), **Tc = Bc / CIR** (time interval). Shaping a 1 Gbps port to a 200 Mbps service with Bc = 2,000,000 bits gives Tc = 10 ms.

## RF math

- **dBm = 10 × log₁₀(mW)**: 1 mW = 0 dBm, 10 mW = 10 dBm, 100 mW = 20 dBm. **+3 dB ≈ double the power, +10 dB = ×10**.
- Antenna gain in **dBi**; **EIRP = transmit power (dBm) + antenna gain (dBi) − cable loss (dB)**.
- **Free-space path loss** (dB) = 20 log₁₀(d in km) + 20 log₁₀(f in MHz) + 32.44.
- Received signal = EIRP − path loss + receive antenna gain. It must exceed the receiver **sensitivity** with a **fade margin** (e.g. 10 dB). Good Wi-Fi: RSSI better than **−67 dBm** for voice, SNR ≥ 25 dB.

## Wireless architecture and roaming

- Deployment: autonomous APs, **centralised WLC** (CAPWAP: control UDP 5246, data 5247), FlexConnect (local switching at branches), cloud-managed, SD-Access fabric wireless.
- AP modes: local, FlexConnect, monitor, sniffer, rogue detector, SE-Connect, bridge/mesh.
- Roaming: **intra-controller** (same WLC), **inter-controller Layer 2** (same subnet), **Layer 3** (anchor/foreign controllers with a tunnel, client keeps its IP). Fast roaming: **802.11r** (FT), 802.11k (neighbour lists), 802.11v (BSS transition).
- Channel planning in 2.4 GHz: **1, 6, 11** non-overlapping; 5 GHz and 6 GHz offer many more channels (DFS rules in 5 GHz).

@@quiz
What is the DSCP value of AF31?
- 24
* 26
- 30
- 34
? 8 × 3 + 2 × 1.

@@quiz
Which DSCP value is used for voice bearer traffic?
- 34
- 24
* 46
- 48
? Expedited forwarding.

@@quiz
A provider must enforce a contracted rate and drop excess traffic. Which tool?
* Policing
- Shaping
- WRED
- LLQ
? Shaping buffers instead.

@@quiz
CIR is 50 Mbps and Bc is 500,000 bits. What is Tc?
- 1 ms
* 10 ms
- 100 ms
- 0.1 ms
? 500,000 / 50,000,000.

@@quiz
How many dBm is 100 mW?
- 10
* 20
- 100
- 30
? 10 × log₁₀(100).

@@quiz
Transmit power 17 dBm, antenna gain 6 dBi, cable loss 2 dB. What is the EIRP?
- 25 dBm
* 21 dBm
- 19 dBm
- 15 dBm
? 17 + 6 − 2.

@@quiz
Which ports does CAPWAP use between APs and the WLC?
- TCP 5246/5247
* UDP 5246 (control) and 5247 (data)
- UDP 1812/1813
- TCP 22/443
? 1812/1813 are RADIUS.

@@flashcards
EF :: DSCP 46, voice
AFxy :: 8x + 2y
AF41 :: 34
AF11 :: 10
CS3 :: 24
CoS voice :: 5
trust boundary :: as close to the source as possible
LLQ :: strict priority, policed
CBWFQ :: bandwidth per class
WRED :: early drop, avoids global sync
voice latency :: ≤ 150 ms one-way
voice jitter :: ≤ 30 ms
policing :: drop/re-mark excess
shaping :: buffer excess
Tc :: Bc / CIR
dBm :: 10 log₁₀(mW)
+3 dB :: double power
+10 dB :: ×10 power
EIRP :: Tx + gain − cable loss
FSPL :: 20log d + 20log f + 32.44
voice RSSI :: −67 dBm or better
CAPWAP :: UDP 5246/5247
802.11r :: fast transition
2.4 GHz channels :: 1, 6, 11

@@drill
What is the DSCP decimal value of AF21?
= 18
? 8 × 2 + 2 × 1.

@@drill
How many dBm is 1 W?
= 30
= 30 dBm
? 1000 mW.

@@exercise Decode DSCP and size a shaper
Decode DSCP values into their names (EF, AFxy, CSx, default) and their IP precedence, then compute Tc for shaping configurations, and check that an LLQ priority class doesn't exceed 33% of the link.
@@lang python
@@starter
dscp_values = [46, 34, 26, 18, 10, 12, 24, 48, 0, 40]
shapers = [("branch WAN", 200_000_000, 2_000_000), ("small office", 20_000_000, 80_000), ("DC link", 1_000_000_000, 8_000_000)]
llq = [("voice+video", 100_000_000, 25_000_000), ("video wall", 50_000_000, 30_000_000)]

def dscp_name(value):
    raise NotImplementedError
@@expected
DSCP 46 = EF            binary 101110, IP precedence 5
DSCP 34 = AF41          binary 100010, IP precedence 4
DSCP 26 = AF31          binary 011010, IP precedence 3
DSCP 18 = AF21          binary 010010, IP precedence 2
DSCP 10 = AF11          binary 001010, IP precedence 1
DSCP 12 = AF12          binary 001100, IP precedence 1
DSCP 24 = CS3           binary 011000, IP precedence 3
DSCP 48 = CS6           binary 110000, IP precedence 6
DSCP  0 = default (BE)  binary 000000, IP precedence 0
DSCP 40 = CS5           binary 101000, IP precedence 5
branch WAN: CIR 200 Mbps, Bc 2,000,000 bits -> Tc 10 ms
small office: CIR 20 Mbps, Bc 80,000 bits -> Tc 4 ms
DC link: CIR 1000 Mbps, Bc 8,000,000 bits -> Tc 8 ms
voice+video: priority 25% of link -> OK
video wall: priority 60% of link -> too high, may starve other classes
@@requires
def dscp_name
>> 3
divmod(
@@hint
EF is 46 and 0 is default. For AF: `x, rest = divmod(value, 8)`; it's AF if 1 ≤ x ≤ 4 and rest in (2, 4, 6) → AF{x}{rest // 2}. CS when rest is 0. IP precedence is `value >> 3`.
@@solution
dscp_values = [46, 34, 26, 18, 10, 12, 24, 48, 0, 40]
shapers = [("branch WAN", 200_000_000, 2_000_000), ("small office", 20_000_000, 80_000), ("DC link", 1_000_000_000, 8_000_000)]
llq = [("voice+video", 100_000_000, 25_000_000), ("video wall", 50_000_000, 30_000_000)]

def dscp_name(value):
    if value == 46:
        return "EF"
    if value == 0:
        return "default (BE)"
    x, rest = divmod(value, 8)
    if 1 <= x <= 4 and rest in (2, 4, 6):
        return f"AF{x}{rest // 2}"
    if rest == 0:
        return f"CS{x}"
    return "unnamed"

for v in dscp_values:
    print(f"DSCP {v:>2} = {dscp_name(v):<13} binary {v:06b}, IP precedence {v >> 3}")

for name, cir, bc in shapers:
    print(f"{name}: CIR {cir / 1e6:g} Mbps, Bc {bc:,} bits -> Tc {bc / cir * 1000:g} ms")

for name, link, priority in llq:
    share = priority / link
    print(f"{name}: priority {share:.0%} of link -> {'OK' if share <= 0.33 else 'too high, may starve other classes'}")

@@exercise Close a wireless link budget
For each link, convert transmit power from mW to dBm, compute EIRP, free-space path loss, and the received signal, then the margin above the receiver sensitivity. A link "closes" with at least `FADE_MARGIN` dB. Also say whether the received signal meets the −67 dBm voice target.
@@lang python
@@starter
import math

FADE_MARGIN = 10
links = [
    ("office AP to laptop", 100, 4, 0, 0.03, 5200, 0, -82),
    ("warehouse AP", 50, 3, 1, 0.08, 2437, 0, -85),
    ("building bridge", 200, 23, 2, 1.5, 5500, 23, -78),
    ("long bridge", 100, 16, 3, 8, 5800, 16, -80),
]

def dbm(mw):
    raise NotImplementedError
@@expected
office AP to laptop: Tx 20.0 dBm, EIRP 24.0 dBm, FSPL 76.3 dB, Rx -52.3 dBm
  margin 29.7 dB -> closes; meets -67 dBm
warehouse AP: Tx 17.0 dBm, EIRP 19.0 dBm, FSPL 78.2 dB, Rx -59.2 dBm
  margin 25.8 dB -> closes; meets -67 dBm
building bridge: Tx 23.0 dBm, EIRP 44.0 dBm, FSPL 110.8 dB, Rx -43.8 dBm
  margin 34.2 dB -> closes; meets -67 dBm
long bridge: Tx 20.0 dBm, EIRP 33.0 dBm, FSPL 125.8 dB, Rx -76.8 dBm
  margin 3.2 dB -> does NOT close; below -67 dBm
@@requires
def dbm
def fspl
math.log10(
@@hint
Each link is `(name, tx_mW, tx_gain_dBi, cable_loss_dB, distance_km, freq_MHz, rx_gain_dBi, sensitivity_dBm)`. FSPL = `20*log10(d) + 20*log10(f) + 32.44`. Received = EIRP − FSPL + rx gain; margin = received − sensitivity.
@@solution
import math

FADE_MARGIN = 10
links = [
    ("office AP to laptop", 100, 4, 0, 0.03, 5200, 0, -82),
    ("warehouse AP", 50, 3, 1, 0.08, 2437, 0, -85),
    ("building bridge", 200, 23, 2, 1.5, 5500, 23, -78),
    ("long bridge", 100, 16, 3, 8, 5800, 16, -80),
]

def dbm(mw):
    return 10 * math.log10(mw)

def fspl(km, mhz):
    return 20 * math.log10(km) + 20 * math.log10(mhz) + 32.44

for name, mw, gain, loss, km, mhz, rx_gain, sens in links:
    eirp = dbm(mw) + gain - loss
    received = eirp - fspl(km, mhz) + rx_gain
    margin = received - sens
    verdict = "closes" if margin >= FADE_MARGIN else "does NOT close"
    voice = "meets -67 dBm" if received >= -67 else "below -67 dBm"
    print(f"{name}: Tx {dbm(mw):.1f} dBm, EIRP {eirp:.1f} dBm, FSPL {fspl(km, mhz):.1f} dB, Rx {received:.1f} dBm")
    print(f"  margin {margin:.1f} dB -> {verdict}; {voice}")
