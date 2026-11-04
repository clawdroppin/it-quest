@@lesson aw1-vpc-deep VPC in Depth: Subnets, Routing, Security Groups, NACLs and Connectivity
@@text
Networking questions on the Solutions Architect exam test whether you can build a VPC that is private by default, reaches the internet only where intended, and connects to other VPCs and on-premises networks. This lesson covers CIDR planning, route tables, the two firewall layers and the hybrid connectivity options.

## CIDR planning

- A VPC has a primary IPv4 CIDR from **/16** (65,536 addresses) to **/28** (16); you can add secondary CIDRs. Use RFC 1918 ranges and **don't overlap** with on-premises or VPCs you will peer.
- Each **subnet** lives in **one AZ**. AWS reserves **5 addresses** per subnet: network address, **.1** VPC router, **.2** DNS, **.3** reserved for future use, and the last (broadcast). A /24 gives **251** usable addresses.
- Typical layout per AZ: a **public** subnet (load balancers, NAT gateways), a **private app** subnet, a **private data** subnet. Repeat across 2–3 AZs.
- IPv6: AWS assigns a /56 to the VPC and /64s to subnets; all IPv6 addresses are globally unique, so use an **egress-only internet gateway** for outbound-only IPv6.

## Routing

Every subnet is associated with one **route table**. The `local` route (VPC CIDR) is always there. Longest prefix wins.

| Destination | Target | Makes the subnet... |
|---|---|---|
| 0.0.0.0/0 | **igw-** (internet gateway) | **public** (instances also need a public IP or Elastic IP) |
| 0.0.0.0/0 | **nat-** (NAT gateway in a public subnet) | **private with outbound internet** |
| 10.20.0.0/16 | pcx- (peering) / tgw- (Transit Gateway) | connected to another VPC |
| pl-xxxx (S3 prefix list) | vpce- (gateway endpoint) | able to reach S3/DynamoDB privately |
| 192.168.0.0/16 | vgw- / tgw- | connected to on-premises |

**NAT gateway**: managed, scales automatically, **per AZ** (deploy one per AZ for resilience), charged per hour and per GB processed. A NAT instance is the legacy do-it-yourself alternative.

## Security groups vs network ACLs

| | Security group | Network ACL |
|---|---|---|
| Applies to | ENIs (instances, ALBs, RDS...) | subnets |
| State | **stateful**: return traffic allowed automatically | **stateless**: return traffic needs its own rule (ephemeral ports 1024–65535) |
| Rules | **allow only** | allow **and deny** |
| Evaluation | all rules together | **in number order, first match**, ending with `*` deny |
| Default | deny inbound, allow outbound | default NACL allows all; custom NACLs deny all |
| References | can reference **other security groups** | CIDRs only |

Pattern: ALB SG allows 443 from 0.0.0.0/0; app SG allows 8080 **from the ALB SG**; DB SG allows 5432 **from the app SG**. Use NACLs for broad subnet-level denies (block a malicious CIDR).

## VPC endpoints

- **Gateway endpoints**: **S3 and DynamoDB** only, free, added as route table targets.
- **Interface endpoints (PrivateLink)**: an ENI with a private IP for most AWS services (and your own services exposed through an NLB); charged per hour and GB. Private DNS makes the normal service name resolve to the endpoint.
- Endpoint policies restrict what can be done through them; bucket policies can require `aws:SourceVpce`.

## Connecting VPCs

- **VPC peering**: one-to-one, **not transitive** (A–B and B–C doesn't give A–C), no overlapping CIDRs, works cross-account and cross-Region.
- **Transit Gateway**: hub-and-spoke for many VPCs and VPNs, transitive, route tables for segmentation.
- **PrivateLink**: expose one service to other VPCs without full network connectivity.

## Hybrid connectivity

| Option | Setup time | Path | Notes |
|---|---|---|---|
| **Site-to-Site VPN** | minutes | encrypted IPsec over the internet | two tunnels per connection; to a VGW or TGW; up to 1.25 Gb/s per tunnel |
| **Direct Connect** | weeks | private dedicated link (1/10/100 Gb/s dedicated, or hosted) | consistent latency, lower data-out cost; **not encrypted by default** (add MACsec or a VPN over it) |
| DX + VPN backup | | | common resilient design |
| **Client VPN** | | OpenVPN-based remote user access | |

## Visibility

**VPC Flow Logs** record accepted/rejected flows per ENI, subnet or VPC (to CloudWatch Logs or S3); **Reachability Analyzer** checks a path's configuration; **Network Access Analyzer** finds unintended access.

```text
2 123456789012 eni-0a1b2c3d 203.0.113.50 10.0.1.25 51544 22 6 3 180 1730707200 1730707260 REJECT OK
```

Fields: version, account, ENI, source, destination, source port, destination port, protocol (6 = TCP), packets, bytes, start, end, **action**, status.

@@quiz
How many usable IP addresses does a /24 subnet have in AWS?
- 254
* 251
- 256
- 250
? AWS reserves 5 addresses per subnet.

@@quiz
Instances in a private subnet need to download OS updates from the internet but must not be reachable from it. What do you add?
- An internet gateway route in the private subnet
* A NAT gateway in a public subnet and a 0.0.0.0/0 route to it from the private subnet
- A gateway endpoint
- VPC peering
? NAT allows outbound-initiated connections only.

@@quiz
A NACL allows inbound TCP 443 but clients time out. Security groups are correct. What's missing?
- An outbound rule on the security group
* An outbound NACL rule for the ephemeral ports (1024–65535)
- A route to the internet gateway
- A second NACL
? NACLs are stateless.

@@quiz
VPC A is peered with B, and B with C. Can A reach C through B?
- Yes, peering is transitive
* No, peering is not transitive; peer A and C directly or use a Transit Gateway
- Only for IPv6
- Only in the same Region
? Transit Gateway is transitive.

@@quiz
Which endpoint type provides free private access to S3 from a VPC?
* Gateway endpoint
- Interface endpoint
- NAT gateway
- Egress-only internet gateway
? Gateway endpoints exist for S3 and DynamoDB.

@@quiz
A company needs consistent low latency to AWS and encryption of the link. What's the design?
- Site-to-Site VPN only
- Direct Connect only
* Direct Connect with MACsec or an IPsec VPN over it
- VPC peering
? DX isn't encrypted by default.

@@flashcards
VPC CIDR size :: /16 to /28
subnet scope :: one AZ
reserved per subnet :: 5 (network, .1, .2, .3, broadcast)
.1 / .2 :: VPC router / DNS
public subnet :: route 0.0.0.0/0 to IGW
NAT gateway :: per AZ, outbound only
egress-only IGW :: outbound-only IPv6
security group :: stateful, allow only, ENI
NACL :: stateless, numbered, allow/deny, subnet
ephemeral ports :: 1024–65535
SG reference :: allow from another SG
gateway endpoint :: S3 and DynamoDB, free
interface endpoint :: PrivateLink ENI
VPC peering :: not transitive, no overlap
Transit Gateway :: transitive hub
Site-to-Site VPN :: IPsec, two tunnels
Direct Connect :: private link, not encrypted by default
Flow Logs :: accepted/rejected flows
Reachability Analyzer :: path configuration check

@@drill
How many IP addresses does AWS reserve in each subnet?
= 5
= five
? First four and the last.

@@drill
Which VPC firewall layer is stateless?
= NACL
= network ACL
= Network ACL
? Security groups are stateful.

@@exercise Plan VPC subnets across AZs
Split the VPC CIDR into equal subnets of `SUBNET_PREFIX`, assigning them in order: for each tier (public, app, data) one subnet per AZ. Print each subnet's name, CIDR, AZ and usable addresses (total − 5, AWS reservation), then the AWS-reserved addresses of the first subnet with their purpose, and how many subnets of that size remain unused.
@@lang python
@@starter
import ipaddress

VPC = "10.40.0.0/16"
SUBNET_PREFIX = 20
AZS = ["eu-west-1a", "eu-west-1b", "eu-west-1c"]
TIERS = ["public", "app", "data"]
@@expected
public-a  10.40.0.0/20    eu-west-1a  4091 usable
public-b  10.40.16.0/20   eu-west-1b  4091 usable
public-c  10.40.32.0/20   eu-west-1c  4091 usable
app-a     10.40.48.0/20   eu-west-1a  4091 usable
app-b     10.40.64.0/20   eu-west-1b  4091 usable
app-c     10.40.80.0/20   eu-west-1c  4091 usable
data-a    10.40.96.0/20   eu-west-1a  4091 usable
data-b    10.40.112.0/20  eu-west-1b  4091 usable
data-c    10.40.128.0/20  eu-west-1c  4091 usable
Reserved in 10.40.0.0/20:
  10.40.0.0 network address
  10.40.0.1 VPC router
  10.40.0.2 Amazon DNS
  10.40.0.3 reserved for future use
  10.40.15.255 broadcast (not supported, reserved)
Unused /20 blocks: 7
@@requires
subnets(new_prefix=
num_addresses - 5
@@hint
`pool = list(ipaddress.ip_network(VPC).subnets(new_prefix=SUBNET_PREFIX))`; take them in order with an index. Reserved: `net[0]`, `net[1]`, `net[2]`, `net[3]`, `net[-1]`.
@@solution
import ipaddress

VPC = "10.40.0.0/16"
SUBNET_PREFIX = 20
AZS = ["eu-west-1a", "eu-west-1b", "eu-west-1c"]
TIERS = ["public", "app", "data"]

pool = list(ipaddress.ip_network(VPC).subnets(new_prefix=SUBNET_PREFIX))
used = 0
first = None
for tier in TIERS:
    for az in AZS:
        net = pool[used]
        used += 1
        first = first or net
        name = f"{tier}-{az[-1]}"
        print(f"{name:<9} {str(net):<15} {az}  {net.num_addresses - 5} usable")

purposes = ["network address", "VPC router", "Amazon DNS", "reserved for future use"]
print(f"Reserved in {first}:")
for addr, purpose in zip([first[0], first[1], first[2], first[3]], purposes):
    print(f"  {addr} {purpose}")
print(f"  {first[-1]} broadcast (not supported, reserved)")
print(f"Unused /{SUBNET_PREFIX} blocks: {len(pool) - used}")

@@exercise Evaluate a network ACL and a security group
Simulate an inbound HTTPS request from a client and its response. The NACL is stateless: check the inbound request against `nacl_in` and the response (from port 443 to the client's ephemeral port) against `nacl_out`, each in rule-number order with `*` as the final deny. The security group is stateful: only inbound is checked; responses are allowed automatically. Print the decision at each step for each client.
@@lang python
@@starter
import ipaddress

nacl_in = [(100, "deny", "203.0.113.0/24", (0, 65535)), (110, "allow", "0.0.0.0/0", (443, 443)), (120, "allow", "0.0.0.0/0", (22, 22))]
nacl_out = [(100, "allow", "0.0.0.0/0", (443, 443))]
sg_in = [("0.0.0.0/0", 443), ("10.0.0.0/8", 22)]
clients = [("198.51.100.20", 52000, 443), ("203.0.113.9", 52001, 443), ("198.51.100.21", 52002, 22)]

def nacl(rules, ip, port):
    raise NotImplementedError
@@expected
198.51.100.20:52000 -> :443
  NACL in rule 110: allow
  SG: allowed (response tracked)
  NACL out to port 52000 rule *: deny (add ephemeral ports 1024-65535)
203.0.113.9:52001 -> :443
  NACL in rule 100: deny
198.51.100.21:52002 -> :22
  NACL in rule 120: allow
  SG: no allow rule, dropped
@@requires
def nacl
sorted(rules)
@@hint
Rules sorted by number; return `(number, action)` for the first rule whose CIDR contains the IP and whose port range contains the port, or `("*", "deny")`.
@@solution
import ipaddress

nacl_in = [(100, "deny", "203.0.113.0/24", (0, 65535)), (110, "allow", "0.0.0.0/0", (443, 443)), (120, "allow", "0.0.0.0/0", (22, 22))]
nacl_out = [(100, "allow", "0.0.0.0/0", (443, 443))]
sg_in = [("0.0.0.0/0", 443), ("10.0.0.0/8", 22)]
clients = [("198.51.100.20", 52000, 443), ("203.0.113.9", 52001, 443), ("198.51.100.21", 52002, 22)]

def nacl(rules, ip, port):
    for number, action, cidr, (low, high) in sorted(rules):
        if ipaddress.ip_address(ip) in ipaddress.ip_network(cidr) and low <= port <= high:
            return number, action
    return "*", "deny"

def sg(ip, port):
    return any(ipaddress.ip_address(ip) in ipaddress.ip_network(cidr) and port == p for cidr, p in sg_in)

for ip, eph, port in clients:
    num, action = nacl(nacl_in, ip, port)
    print(f"{ip}:{eph} -> :{port}")
    print(f"  NACL in rule {num}: {action}")
    if action == "deny":
        continue
    if not sg(ip, port):
        print("  SG: no allow rule, dropped")
        continue
    print("  SG: allowed (response tracked)")
    num, action = nacl(nacl_out, ip, eph)
    print(f"  NACL out to port {eph} rule {num}: {action}" + ("" if action == "allow" else " (add ephemeral ports 1024-65535)"))
