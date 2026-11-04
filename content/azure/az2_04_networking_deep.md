@@lesson az2-network-deep Azure Networking in Depth: VNets, NSGs, Routing, Load Balancing and Hybrid
@@text
Virtual networking is 15–20% of AZ-104 and the area where most real deployments go wrong. This lesson covers address planning, NSGs and ASGs, how Azure picks a route (system routes, UDRs, BGP), peering and gateways, the four load balancing services, private access to PaaS and DNS.

## VNets and subnets

- A **VNet** is regional, with one or more address spaces (RFC 1918 recommended; don't overlap anything you'll connect). **Subnets** divide it.
- Azure reserves **5 addresses** per subnet: **.0** network, **.1** default gateway, **.2 and .3** Azure DNS mapping, and the **last** (broadcast). A /29 is the smallest usable subnet (3 usable addresses).
- Special subnets with fixed names: **GatewaySubnet** (VPN/ExpressRoute gateway, /27 or larger recommended), **AzureBastionSubnet** (/26+), **AzureFirewallSubnet** (/26).
- **Public IPs**: Standard SKU (static, zone-redundant, secure by default: closed until an NSG allows traffic).

## Network security groups

- Attached to a **subnet** and/or a **NIC**. Inbound traffic is evaluated by the **subnet NSG first, then the NIC NSG**; outbound the other way round. Both must allow.
- Rules have **priority 100–4096**, lower first; first match wins. **Stateful**: return traffic is allowed.
- **Default rules** (priority 65000+, can't be deleted, can be overridden by lower numbers):

| Priority | Inbound | Outbound |
|---|---|---|
| 65000 | AllowVNetInBound | AllowVNetOutBound |
| 65001 | AllowAzureLoadBalancerInBound | AllowInternetOutBound |
| 65500 | DenyAllInBound | DenyAllOutBound |

- **Service tags** (`Internet`, `VirtualNetwork`, `AzureLoadBalancer`, `Storage.WestEurope`, `AzureCloud`) replace IP lists. **Application security groups (ASGs)** group NICs by role (`asg-web`, `asg-db`) so rules read "allow asg-web → asg-db on 1433".
- Troubleshoot with **Network Watcher**: **IP flow verify** (which rule allows/denies), **effective security rules**, **next hop**, **connection troubleshoot**, NSG/VNet **flow logs**.

## Routing

Every subnet has **system routes**: within the VNet, to peered VNets, to on-premises via the gateway (BGP), and `0.0.0.0/0` → **Internet**. **User-defined routes (UDRs)** in a **route table** override them, e.g. `0.0.0.0/0` → **virtual appliance** (Azure Firewall/NVA private IP) to force traffic through inspection.

Route selection: **longest prefix match** first; on a tie, **UDR > BGP > system route**. Next hop types: Virtual network, Virtual network gateway, Internet, **Virtual appliance**, **None** (drop).

## Connecting networks

- **VNet peering** (regional or global): low-latency backbone, **non-transitive**; options: allow forwarded traffic, **use remote gateway / allow gateway transit** (spokes use the hub's VPN gateway). Hub-and-spoke or **Virtual WAN** for scale.
- **VPN Gateway**: **site-to-site** (IPsec, needs a local network gateway representing on-premises), **point-to-site** (individual clients: OpenVPN/IKEv2 with certificate or Entra auth), VNet-to-VNet. Active-active and zone-redundant SKUs.
- **ExpressRoute**: private connection via a provider (not over the internet), higher bandwidth, SLA; private peering and Microsoft peering; Global Reach links sites; add a VPN as backup or MACsec/IPsec for encryption.

## Load balancing

| Service | Layer | Scope | Use |
|---|---|---|---|
| **Azure Load Balancer** | 4 (TCP/UDP) | regional | VMs/VMSS, internal or public, HA ports |
| **Application Gateway** | 7 (HTTP/S) | regional | path/host routing, TLS termination, **WAF**, autoscale |
| **Front Door** | 7 | global | global HTTP apps, CDN caching, WAF, fast failover |
| **Traffic Manager** | DNS | global | route by priority, weighted, performance, geographic; any endpoint |

Load Balancer needs **health probes**, a **backend pool**, **load-balancing rules** (and inbound NAT rules for per-VM access). Standard SKU is zone-redundant and closed by default.

## Private access and DNS

- **Service endpoints**: optimised route from a subnet to a PaaS service's public endpoint; the service firewall can then allow that subnet.
- **Private endpoints** (Private Link): a NIC with a private IP for one PaaS resource; needs **private DNS zones** (e.g. `privatelink.blob.core.windows.net`) linked to the VNet so names resolve to the private IP.
- **Azure DNS**: public zones (host your domain) and **private DNS zones** with **auto-registration** of VM records. **Azure DNS Private Resolver** connects on-premises DNS.
- **Azure Firewall** (stateful, FQDN rules, threat intelligence), **DDoS Protection**, **NAT Gateway** (outbound SNAT for a subnet).

@@quiz
How many usable addresses does a /28 subnet have in Azure?
- 14
* 11
- 16
- 13
? 16 minus the 5 reserved.

@@quiz
An NSG has rule 300 deny TCP 3389 from Internet and rule 200 allow TCP 3389 from 203.0.113.10. What happens to RDP from 203.0.113.10?
* Allowed: rule 200 has the lower number and matches first
- Denied: deny rules win
- Denied by the default rules
- Allowed only if the NIC has no NSG
? Lower priority numbers are evaluated first.

@@quiz
Traffic to the internet must go through an Azure Firewall at 10.0.1.4. What do you configure?
- A service endpoint
* A route table with 0.0.0.0/0 next hop Virtual appliance 10.0.1.4 on the subnets
- VNet peering
- A NAT gateway
? UDRs override the system default route.

@@quiz
Spokes A and B are both peered with a hub. Can A talk to B through the peerings?
- Yes, peering is transitive
* No; route through an NVA/firewall in the hub, use Virtual WAN, or peer A and B
- Only with global peering
- Only for UDP
? Peering is non-transitive.

@@quiz
A global web app needs layer 7 routing, caching at the edge and WAF. Which service?
- Azure Load Balancer
- Traffic Manager
* Azure Front Door
- Application Gateway
? Application Gateway is regional.

@@quiz
A private endpoint for a storage account was created but VMs still resolve the public IP. What's missing?
- A service endpoint
* A private DNS zone (privatelink.blob.core.windows.net) linked to the VNet
- A NAT gateway
- An NSG rule
? Name resolution must return the private IP.

@@flashcards
reserved per subnet :: 5 (.0, .1, .2, .3, last)
.1 :: default gateway
GatewaySubnet :: VPN/ExpressRoute gateways
NSG priority :: 100–4096, lower first
NSG state :: stateful
inbound order :: subnet NSG, then NIC NSG
DenyAllInBound :: priority 65500
service tag :: named group of IP ranges
ASG :: group NICs by role
IP flow verify :: which NSG rule applies
UDR :: user-defined route
route tie-break :: UDR > BGP > system
next hop None :: drop traffic
peering :: non-transitive
gateway transit :: spokes use hub gateway
site-to-site VPN :: IPsec + local network gateway
ExpressRoute :: private provider connection
Load Balancer :: layer 4 regional
Application Gateway :: layer 7 regional + WAF
Front Door :: layer 7 global + CDN
Traffic Manager :: DNS-based global
private endpoint :: private IP for a PaaS resource
private DNS zone :: privatelink name resolution

@@drill
What is the lowest-numbered (highest) priority you can give a custom NSG rule?
= 100
? Up to 4096.

@@drill
Which Network Watcher tool tells you which NSG rule allowed or denied a packet?
= IP flow verify
= ip flow verify
? Next hop checks routing.

@@exercise Evaluate NSGs on subnet and NIC
Evaluate inbound connections through the subnet NSG then the NIC NSG: in each, sort rules by priority, the first matching rule decides (source by CIDR or service tag `Internet` = any non-RFC1918 address, `VirtualNetwork` = 10.0.0.0/16; destination port or `*`). Append the default inbound rules to each NSG. Print which rule decided at each NSG and the final result.
@@lang python
@@starter
import ipaddress

DEFAULTS = [(65000, "AllowVNetInBound", "VirtualNetwork", "*", "allow"), (65001, "AllowAzureLoadBalancerInBound", "AzureLoadBalancer", "*", "allow"), (65500, "DenyAllInBound", "*", "*", "deny")]
subnet_nsg = [(100, "allow-https", "Internet", 443, "allow"), (110, "allow-rdp-admin", "203.0.113.10/32", 3389, "allow"), (4000, "deny-rdp", "*", 3389, "deny")]
nic_nsg = [(100, "allow-https", "*", 443, "allow"), (200, "deny-rdp-internet", "Internet", 3389, "deny")]
connections = [("198.51.100.8", 443), ("203.0.113.10", 3389), ("198.51.100.8", 3389), ("10.0.2.5", 3389), ("198.51.100.8", 22)]

def source_matches(source, ip):
    raise NotImplementedError
@@expected
198.51.100.8 -> :443: allowed (subnet 100 allow-https allow; NIC 100 allow-https allow)
203.0.113.10 -> :3389: denied (subnet 110 allow-rdp-admin allow; NIC 200 deny-rdp-internet deny)
198.51.100.8 -> :3389: denied (subnet 4000 deny-rdp deny)
10.0.2.5 -> :3389: denied (subnet 4000 deny-rdp deny)
198.51.100.8 -> :22: denied (subnet 65500 DenyAllInBound deny)
@@requires
def source_matches
sorted(
RFC1918
@@hint
`Internet` matches when the address is in none of the `RFC1918` networks (don't use `is_private`: it also treats documentation ranges like 198.51.100.0/24 as private); `*` matches anything; `AzureLoadBalancer` only matches 168.63.129.16.
@@solution
import ipaddress

DEFAULTS = [(65000, "AllowVNetInBound", "VirtualNetwork", "*", "allow"), (65001, "AllowAzureLoadBalancerInBound", "AzureLoadBalancer", "*", "allow"), (65500, "DenyAllInBound", "*", "*", "deny")]
subnet_nsg = [(100, "allow-https", "Internet", 443, "allow"), (110, "allow-rdp-admin", "203.0.113.10/32", 3389, "allow"), (4000, "deny-rdp", "*", 3389, "deny")]
nic_nsg = [(100, "allow-https", "*", 443, "allow"), (200, "deny-rdp-internet", "Internet", 3389, "deny")]
connections = [("198.51.100.8", 443), ("203.0.113.10", 3389), ("198.51.100.8", 3389), ("10.0.2.5", 3389), ("198.51.100.8", 22)]

RFC1918 = [ipaddress.ip_network(n) for n in ("10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16")]

def source_matches(source, ip):
    addr = ipaddress.ip_address(ip)
    if source == "*":
        return True
    if source == "Internet":
        return not any(addr in net for net in RFC1918)
    if source == "VirtualNetwork":
        return addr in ipaddress.ip_network("10.0.0.0/16")
    if source == "AzureLoadBalancer":
        return ip == "168.63.129.16"
    return addr in ipaddress.ip_network(source)

def evaluate(nsg, ip, port):
    for priority, name, source, dport, action in sorted(nsg + DEFAULTS):
        if source_matches(source, ip) and dport in ("*", port):
            return priority, name, action
    return None

for ip, port in connections:
    result = "allowed"
    steps = []
    for label, nsg in (("subnet", subnet_nsg), ("NIC", nic_nsg)):
        priority, name, action = evaluate(nsg, ip, port)
        steps.append(f"{label} {priority} {name} {action}")
        if action == "deny":
            result = "denied"
            break
    print(f"{ip} -> :{port}: {result} ({'; '.join(steps)})")

@@exercise Pick the effective route
Given a subnet's system routes, BGP routes and UDRs, find the effective route for each destination: longest prefix wins; on a tie, UDR beats BGP beats system. Print the chosen prefix, source and next hop, and `dropped` when the next hop type is `None`.
@@lang python
@@starter
import ipaddress

routes = [
    ("10.0.0.0/16", "system", "VirtualNetwork", ""),
    ("10.1.0.0/16", "system", "VNetPeering", ""),
    ("0.0.0.0/0", "system", "Internet", ""),
    ("192.168.0.0/16", "bgp", "VirtualNetworkGateway", "10.0.255.4"),
    ("0.0.0.0/0", "udr", "VirtualAppliance", "10.0.1.4"),
    ("10.1.5.0/24", "udr", "VirtualAppliance", "10.0.1.4"),
    ("192.168.50.0/24", "udr", "None", ""),
    ("10.0.0.0/16", "udr", "VirtualAppliance", "10.0.1.4"),
]
destinations = ["10.0.3.7", "10.1.5.20", "10.1.9.9", "192.168.10.1", "192.168.50.9", "8.8.8.8"]
PREFERENCE = {"udr": 0, "bgp": 1, "system": 2}
@@expected
10.0.3.7      -> 10.0.0.0/16      (udr) VirtualAppliance 10.0.1.4
10.1.5.20     -> 10.1.5.0/24      (udr) VirtualAppliance 10.0.1.4
10.1.9.9      -> 10.1.0.0/16      (system) VNetPeering
192.168.10.1  -> 192.168.0.0/16   (bgp) VirtualNetworkGateway 10.0.255.4
192.168.50.9  -> 192.168.50.0/24  (udr) dropped
8.8.8.8       -> 0.0.0.0/0        (udr) VirtualAppliance 10.0.1.4
@@requires
PREFERENCE[
prefixlen
@@hint
Among matching routes pick `min(..., key=lambda r: (-network.prefixlen, PREFERENCE[source]))`.
@@solution
import ipaddress

routes = [
    ("10.0.0.0/16", "system", "VirtualNetwork", ""),
    ("10.1.0.0/16", "system", "VNetPeering", ""),
    ("0.0.0.0/0", "system", "Internet", ""),
    ("192.168.0.0/16", "bgp", "VirtualNetworkGateway", "10.0.255.4"),
    ("0.0.0.0/0", "udr", "VirtualAppliance", "10.0.1.4"),
    ("10.1.5.0/24", "udr", "VirtualAppliance", "10.0.1.4"),
    ("192.168.50.0/24", "udr", "None", ""),
    ("10.0.0.0/16", "udr", "VirtualAppliance", "10.0.1.4"),
]
destinations = ["10.0.3.7", "10.1.5.20", "10.1.9.9", "192.168.10.1", "192.168.50.9", "8.8.8.8"]
PREFERENCE = {"udr": 0, "bgp": 1, "system": 2}

for dest in destinations:
    ip = ipaddress.ip_address(dest)
    matching = [(ipaddress.ip_network(p), src, hop, addr) for p, src, hop, addr in routes if ip in ipaddress.ip_network(p)]
    net, src, hop, addr = min(matching, key=lambda r: (-r[0].prefixlen, PREFERENCE[r[1]]))
    target = "dropped" if hop == "None" else f"{hop} {addr}".strip()
    print(f"{dest:<13} -> {str(net):<16} ({src}) {target}")
