@@lesson n2-wireless-wan Wireless, WAN and Cloud Networking
@@text
## Enterprise wireless

- **SSID** is the network name; **BSSID** the MAC of one access point radio; an **ESS** is several APs sharing one SSID so users **roam**.
- Companies use **lightweight APs** managed by a **wireless LAN controller** (or a cloud controller), instead of configuring each AP.
- **Channel width**: 20 MHz is more reliable in crowded places; 40/80/160 MHz is faster. Plan channels so neighbouring APs don't overlap, and do a **site survey** (heat maps) before installing.
- Antennas: **omnidirectional** (all around, office ceilings), **directional** (Yagi, parabolic: point-to-point links).
- Security: **WPA3-Personal** (SAE) for small sites, **WPA2/3-Enterprise** with **802.1X** and a **RADIUS** server for companies, a **captive portal** for guests.
- Interference: microwaves, Bluetooth and cordless phones on 2.4 GHz; walls, metal and water absorb signal. Signal strength is measured in **dBm** (−30 excellent, −67 good for voice/video, −80 unusable).

## WAN connections

| Option | Notes |
|---|---|
| **Fiber** (FTTH, metro Ethernet) | fastest, symmetrical; Japan has excellent 1–10 Gb/s fiber (NTT FLET'S, NURO) |
| **Cable** (DOCSIS) | shared coax; fast download, slower upload |
| **DSL** | old copper phone lines |
| **Cellular** 4G/5G | backup links, branch sites, IoT |
| **Satellite** | remote areas; LEO (Starlink) has much lower latency than geostationary |
| **MPLS** | private carrier network with QoS between company sites |
| **SD-WAN** | software that uses several cheap links (internet, LTE, MPLS) together, choosing the best path per application |

**VPNs**: **site-to-site** (two offices joined over the internet with IPsec), **client-to-site** (remote access for staff; **full tunnel** sends everything through the company, **split tunnel** only company traffic). **Clientless** VPN works in a browser over TLS.

## Cloud networking

In AWS and Azure you build networks in software:

- A **VPC** (AWS) / **VNet** (Azure) is your private network in the cloud, split into **public** and **private subnets**.
- An **internet gateway** connects public subnets to the internet; a **NAT gateway** lets private subnets reach out without being reachable.
- **Security groups** (stateful, per instance/NIC) and **network ACLs** (stateless, per subnet) filter traffic.
- Connect to the office with a **VPN** or a private line (**AWS Direct Connect**, **Azure ExpressRoute**).
- Deployment models: public, private, hybrid; service models IaaS, PaaS, SaaS. **IaC** (Terraform, CloudFormation, Bicep) defines it all as code.

@@quiz
Users walk around an office building and stay connected to the same Wi-Fi network as they move between access points. What makes this possible?
* Several APs with the same SSID forming an ESS, so clients roam
- A directional antenna
- WPS
- A captive portal
? An extended service set shares one SSID across APs.

@@quiz
A company wants each employee to log into Wi-Fi with their own directory account. What's needed?
* WPA2/WPA3-Enterprise with 802.1X and a RADIUS server
- WPA3-Personal with a long passphrase
- MAC filtering
- A hidden SSID
? Enterprise mode authenticates each user individually.

@@quiz
A company with 20 branches wants to combine cheap internet links and LTE backup, steering video calls over the best path automatically. What technology fits?
* SD-WAN
- MPLS only
- A Yagi antenna
- Dial-up
? SD-WAN chooses paths per application across multiple links.

@@quiz
A remote worker's VPN sends only company traffic through the tunnel, and YouTube goes directly to the internet. What is this?
* Split tunnel
- Full tunnel
- Site-to-site VPN
- Clientless VPN
? Split tunnelling saves bandwidth but means internet traffic isn't inspected by the company.

@@quiz
In AWS, servers in a private subnet need to download updates from the internet but must not be reachable from it. What do they use?
* A NAT gateway
- An internet gateway with public IPs
- Direct Connect
- A captive portal
? NAT gateways allow outbound-only connections.

@@quiz
A Wi-Fi client shows a signal of −82 dBm. What does that mean?
* The signal is very weak; expect drops and slow speeds
- The signal is excellent
- The client is too close to the AP
- The AP is on 6 GHz
? Around −67 dBm or better is needed for reliable voice and video.

@@drill
Which Wi-Fi security standard is the current, strongest one?
= WPA3
? WPA2 is still common; avoid WEP and WPA.

@@flashcards
SSID / BSSID :: Network name / one AP radio's MAC
ESS :: Several APs sharing an SSID for roaming
wireless LAN controller :: Centrally manages lightweight APs
site survey :: Measuring coverage and interference before/after installing APs
omnidirectional vs directional antenna :: All around vs pointed (Yagi, parabolic)
802.1X :: Port/Wi-Fi authentication with RADIUS
captive portal :: Login web page for guest Wi-Fi
dBm :: Signal strength; closer to 0 is stronger
MPLS :: Carrier private WAN with QoS
SD-WAN :: Software-chosen paths across multiple WAN links
site-to-site VPN :: Connects networks, usually IPsec
split vs full tunnel :: Only company traffic vs all traffic through the VPN
VPC / VNet :: Your private network in AWS / Azure
NAT gateway :: Outbound internet for private subnets
security group vs NACL :: Stateful per instance vs stateless per subnet
Direct Connect / ExpressRoute :: Private lines into AWS / Azure

@@exercise Choose an SD-WAN path per application
An SD-WAN edge has three links and measures each one. Write `best_link(app, links)` that picks, for each application, the cheapest link (lowest `cost`) that meets the app's needs: latency at or below `max_ms`, loss at or below `max_loss` %, and `up`. If none qualifies, fall back to the up link with the lowest latency and mark it `(degraded)`. Print the choice for each application, then again after the fibre link goes down.
@@lang python
@@starter
links = {
    "fibre": {"latency": 8, "loss": 0.1, "cost": 1, "up": True},
    "mpls": {"latency": 12, "loss": 0.0, "cost": 3, "up": True},
    "lte": {"latency": 45, "loss": 1.5, "cost": 2, "up": True},
}
apps = {
    "voice": {"max_ms": 30, "max_loss": 0.5},
    "erp": {"max_ms": 60, "max_loss": 0.2},
    "backup": {"max_ms": 200, "max_loss": 2.0},
}

def best_link(app, links):
    raise NotImplementedError


for state in ["all links up", "fibre down"]:
    if state == "fibre down":
        links["fibre"]["up"] = False
        links["mpls"]["loss"] = 0.8
    print(state + ":")
    for app in apps:
        print(f"  {app}: {best_link(app, links)}")
@@expected
all links up:
  voice: fibre
  erp: fibre
  backup: fibre
fibre down:
  voice: mpls (degraded)
  erp: mpls (degraded)
  backup: lte
@@requires
def best_link
min(
@@hint
Filter the links that meet every requirement, then `min(..., key=cost)`. The fallback is a second `min` by latency over the links that are up.
@@solution
links = {
    "fibre": {"latency": 8, "loss": 0.1, "cost": 1, "up": True},
    "mpls": {"latency": 12, "loss": 0.0, "cost": 3, "up": True},
    "lte": {"latency": 45, "loss": 1.5, "cost": 2, "up": True},
}
apps = {
    "voice": {"max_ms": 30, "max_loss": 0.5},
    "erp": {"max_ms": 60, "max_loss": 0.2},
    "backup": {"max_ms": 200, "max_loss": 2.0},
}

def best_link(app, links):
    need = apps[app]
    good = [name for name, l in links.items()
            if l["up"] and l["latency"] <= need["max_ms"] and l["loss"] <= need["max_loss"]]
    if good:
        return min(good, key=lambda name: links[name]["cost"])
    up = [name for name, l in links.items() if l["up"]]
    return min(up, key=lambda name: links[name]["latency"]) + " (degraded)"

for state in ["all links up", "fibre down"]:
    if state == "fibre down":
        links["fibre"]["up"] = False
        links["mpls"]["loss"] = 0.8
    print(state + ":")
    for app in apps:
        print(f"  {app}: {best_link(app, links)}")
