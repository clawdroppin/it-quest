@@lesson cp1-services-security-automation Switching, Services, Security and Automation
@@text
## Layer 2 in depth

**Spanning tree** versions:

| Version | Standard | Notes |
|---|---|---|
| PVST+ | Cisco | one classic STP per VLAN; slow (30–50 s) |
| **Rapid PVST+** | Cisco, based on 802.1w | the usual default; converges in about a second |
| **MST** | 802.1s | maps many VLANs to a few instances: scales to thousands of VLANs |

Port roles (RSTP): **root**, **designated**, **alternate** (backup towards the root), **backup**. Protection features:

- **PortFast** on access ports (skip listening/learning) with **BPDU Guard** (err-disable the port if a BPDU arrives: someone plugged in a switch).
- **Root Guard** on ports where a superior root must never appear.
- **Loop Guard** and **UDLD** against unidirectional links.

**EtherChannel** bundles links: **LACP** (802.3ad, modes active/passive) or Cisco **PAgP** (desirable/auto). Speed, duplex, VLANs and trunk settings must match on all members. Load balancing is per flow, by a hash of addresses or ports.

## First-hop redundancy

| Protocol | Standard | Active routers | Virtual MAC |
|---|---|---|---|
| **HSRP** | Cisco | one active, one standby | 0000.0c07.acXX (v1) |
| **VRRP** | open (RFC 5798) | one master | 0000.5e00.01XX |
| **GLBP** | Cisco | several forwarders: load balancing | 0007.b400.XXYY |

Use **preempt** so the preferred router takes back the role, and **object tracking** to lower its priority when its uplink fails.

## IP services

- **NTP**: accurate time matters for logs, certificates and Kerberos. Stratum = distance from the reference clock.
- **NAT/PAT**: static NAT (one-to-one), dynamic NAT (a pool), PAT (overload: many inside addresses share one outside address by port).
- **QoS**: classify and **mark** (DSCP: **EF = 46** for voice, AF classes for business data, CS6 for routing), then **queue** (LLQ gives voice strict priority), **police** (drop or re-mark above a rate) or **shape** (buffer and smooth). Mark close to the source, at the edge of the trust boundary.
- **Multicast**: IGMP between hosts and routers; **PIM sparse mode** between routers, with a **rendezvous point**; addresses 224.0.0.0/4.

## Network assurance

- **Syslog** severity levels 0–7: **e**mergency, **a**lert, **c**ritical, **e**rror, **w**arning, **n**otice, **i**nformational, **d**ebug ("Every Awesome Cisco Engineer Will Need Ice-cream Daily").
- **SNMP**: v2c (community strings, cleartext) versus **v3** (authentication and encryption: use it).
- **NetFlow / Flexible NetFlow / IPFIX**: who talked to whom, how much. The basis of traffic analysis.
- **SPAN / RSPAN / ERSPAN**: copy traffic to an analyzer port (locally / over a VLAN / over GRE).
- **IP SLA**: synthetic probes (ICMP echo, jitter) whose results can drive object tracking.
- **Model-driven telemetry**: the device *pushes* data (gRPC/gNMI), instead of being polled with SNMP.

## Security

- **AAA** with **TACACS+** (TCP 49, encrypts the whole packet, separates authorization: ideal for device administration) or **RADIUS** (UDP 1812/1813, for network access: 802.1X, VPN, Wi-Fi).
- **802.1X**: supplicant (the client) → authenticator (the switch or WLC) → authentication server (ISE). **MAB** for devices without a supplicant (printers).
- **Control plane policing (CoPP)**: rate-limits traffic *to* the router's CPU, protecting routing protocols from floods.
- **Device hardening**: SSH v2 only, `service password-encryption` (weak) and `enable secret` (strong hash), login banners, disable unused services and ports, ACLs on the VTY lines.
- **Wireless security**: WPA2/WPA3-Personal (PSK, SAE) versus Enterprise (802.1X + EAP); WPA3 adds SAE and protected management frames.
- Cisco's architecture: **Umbrella** (DNS security), **Secure Firewall** (FTD), **ISE** (identity and policy), **Secure Endpoint**, **Stealthwatch** (now Secure Network Analytics: NetFlow-based threat detection).

## Automation

The ENCOR automation domain expects you to read code and API calls:

- **Data formats**: JSON, XML and YAML. **YANG** is the *data model*: it describes what a device's configuration looks like.
- **NETCONF**: SSH port **830**, XML, operations like `<get-config>` and `<edit-config>`, with a candidate datastore and commit.
- **RESTCONF**: HTTPS, JSON or XML, REST verbs over the same YANG models: `GET https://router/restconf/data/ietf-interfaces:interfaces`.
- **REST APIs** (Catalyst Center, vManage, Meraki): HTTP status codes (200 OK, 201 Created, 204 No Content, 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 500 Server Error), tokens in headers.
- **Configuration management**: **Ansible** (agentless, over SSH, YAML playbooks: the most common in networking), Puppet and Chef (agent-based), Terraform.
- **EEM** (Embedded Event Manager): on-box scripts triggered by events (a syslog message, an interface going down).
- **Python** with Netmiko (SSH, screen scraping), NAPALM (multi-vendor), ncclient (NETCONF), or `requests` for REST.

```python,norun
import requests
r = requests.get(
    "https://10.0.0.1/restconf/data/ietf-interfaces:interfaces",
    auth=("admin", "secret"), headers={"Accept": "application/yang-data+json"}, verify=False)
for intf in r.json()["ietf-interfaces:interfaces"]["interface"]:
    print(intf["name"], intf["enabled"])
```

(`verify=False` disables certificate checks: fine in a lab, never in production.)

@@quiz
Which STP feature should be enabled on access ports so that a switch plugged in by a user shuts the port down?
* BPDU Guard
- Root Guard
- UDLD
- Loop Guard
? Combine it with PortFast on edge ports.

@@quiz
Which first-hop redundancy protocol load-balances between several routers by itself?
* GLBP
- HSRP
- VRRP
- NTP
? GLBP's AVG hands out different virtual MACs to different hosts.

@@quiz
Which DSCP value is used for voice traffic?
* EF (46)
- AF41 (34)
- CS6 (48)
- 0
? EF (Expedited Forwarding) goes into the low-latency priority queue.

@@quiz
Which AAA protocol is best for administering network devices, with per-command authorization?
* TACACS+
- RADIUS
- LDAP
- Kerberos
? TACACS+ (TCP 49) separates authentication, authorization and accounting, and encrypts the whole body.

@@quiz
Which port and transport does NETCONF use?
* SSH on TCP 830
- HTTPS on 443
- UDP 161
- Telnet on 23
? RESTCONF uses HTTPS; SNMP uses UDP 161/162.

@@quiz
What is YANG?
* A data modeling language describing a device's configuration and state
- A transport protocol
- A Cisco-only CLI
- An encryption algorithm
? NETCONF and RESTCONF carry data that's structured according to YANG models.

@@flashcards
Rapid PVST+ :: per-VLAN 802.1w spanning tree (fast)
MST :: maps VLANs to a few STP instances (802.1s)
BPDU Guard :: err-disables a PortFast port that receives a BPDU
LACP modes :: active / passive (802.3ad)
HSRP / VRRP / GLBP :: Cisco / open standard / Cisco with load balancing
DSCP EF :: 46: voice
policing vs shaping :: drop above the rate / buffer and smooth
syslog levels :: 0 emergency … 7 debug
SNMPv3 :: authentication and encryption
TACACS+ / RADIUS ports :: TCP 49 / UDP 1812-1813
CoPP :: rate-limits traffic to the control plane
NETCONF :: SSH 830, XML, YANG
RESTCONF :: HTTPS REST with YANG data
Ansible :: agentless, SSH, YAML playbooks

@@drill
Which syslog severity number is "warning"?
= 4
? 0 emergency, 1 alert, 2 critical, 3 error, 4 warning, 5 notice, 6 informational, 7 debug.

@@drill
Which TCP port does BGP use?
= 179
