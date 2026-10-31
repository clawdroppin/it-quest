@@lesson n3-troubleshooting Network Troubleshooting and Tools
@@text
Network+ uses the same 7-step methodology as A+, with one useful addition: **approaches by layer**.

- **Top-down**: start at the application (does the website load?) and go down.
- **Bottom-up**: start at the cable and go up. Best when nothing works at all.
- **Divide and conquer**: start in the middle (can I ping the gateway?) and go up or down from the result.
- **Follow the path**: trace from the source to the destination, hop by hop.

## Tools

| Command / tool | Use |
|---|---|
| `ping`, `traceroute`/`tracert`, `pathping`, `mtr` | reachability, path, where packets are lost |
| `ipconfig` / `ip addr`, `ifconfig` | interface settings |
| `nslookup`, `dig` | DNS queries (`dig +short mx example.com`) |
| `arp -a` | the ARP cache |
| `netstat`, `ss` | connections and listening ports |
| `route print` / `ip route` | the routing table |
| `nmap` | port and host discovery (only on networks you're authorized to scan!) |
| `tcpdump`, **Wireshark** | packet capture and analysis |
| `iperf` | measure throughput between two hosts |
| `curl`, `openssl s_client` | test HTTP(S) and certificates |
| `show interface`, `show ip route`, `show vlan`, `show mac address-table` | on Cisco devices |
| cable tester, **TDR**/**OTDR**, toner, light meter, Wi-Fi analyzer | physical layer |

## Common problems

| Symptom | Likely cause |
|---|---|
| Link light off | cable, wrong port, disabled port, bad transceiver |
| Slow link at 100 Mb/s on a gigabit port | a damaged pair (gigabit needs all 4 pairs), **duplex/speed mismatch** |
| CRC errors and runts in interface counters | bad cable, interference, duplex mismatch |
| Works then stops every few minutes | IP **address conflict**, flapping link, STP topology changes |
| Can reach IPs but not names | DNS |
| Can reach the local subnet only | wrong **default gateway** or mask |
| One VLAN can't reach another | trunk missing the VLAN, routing, ACL |
| Fiber link down | dirty connector, wrong type (single-mode/multimode), TX/RX swapped |
| Voice choppy | **jitter**, **latency**, packet loss: check QoS |
| New site can't be reached from one network only | asymmetric routing, missing route back |
| Wi-Fi drops in one area | coverage gap, interference, channel overlap |

**Latency** is delay; **jitter** is variation in delay; **throughput** is real-world speed; **bandwidth** is capacity. **Attenuation** is signal loss over distance (why copper stops at 100 m).

@@quiz
Nothing works on a user's PC: no network at all. Which approach is usually fastest?
* Bottom-up: start with the physical layer
- Top-down: start with the application
- Reinstall the OS
- Replace the switch
? With total failure, check the cable and link light first.

@@quiz
A gigabit link keeps negotiating at 100 Mb/s. What's a likely cause?
* A damaged cable pair or a bad termination
- DNS misconfiguration
- A wrong default gateway
- An expired certificate
? 100 Mb/s Ethernet needs 2 pairs; gigabit needs all 4.

@@quiz
Which tool measures throughput between two hosts?
* iperf
- nslookup
- arp
- Wireshark only
? Run iperf as a server on one host and a client on the other.

@@quiz
A PC can ping hosts on its own subnet but nothing beyond it. What should you check?
* The default gateway (and subnet mask) setting
- The DNS server
- The MX record
- The Wi-Fi channel
? Traffic for other networks goes to the gateway.

@@quiz
Users report choppy VoIP calls, though data apps are fine. Which metrics matter most?
* Jitter, latency and packet loss
- Disk throughput
- DNS TTL
- Number of VLANs
? Voice needs consistent, low delay. QoS should prioritize it.

@@quiz
Which tool locates a break in a fiber cable, and how far away it is?
* An OTDR
- A toner probe
- A crimper
- A loopback plug
? An optical time-domain reflectometer measures reflections along the fiber (a TDR does the same for copper).

@@quiz
Interface counters on a switch port show many CRC errors. What's most likely?
* A physical problem: bad cable, interference or a duplex mismatch
- A DNS misconfiguration
- Too many VLANs
- A wrong DHCP lease time
? CRC errors mean frames arrived damaged.

@@flashcards
top-down / bottom-up :: Start at the application / at the physical layer
divide and conquer :: Start in the middle layer
traceroute / tracert :: Shows each hop to a destination
dig / nslookup :: DNS query tools
arp -a :: Show the ARP cache
nmap :: Port and host scanner (authorized use only)
Wireshark / tcpdump :: Packet capture and analysis
iperf :: Throughput test
OTDR / TDR :: Find breaks in fiber / copper
duplex mismatch :: One side full, the other half duplex: errors and slowness
latency / jitter :: Delay / variation in delay
attenuation :: Signal loss over distance
CRC errors :: Damaged frames: usually physical problems

@@drill
Which command shows the routing table on Windows?
= route print
= netstat -r
? On Linux: `ip route`.
