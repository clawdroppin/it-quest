@@lesson xp-config_generator Branch Router Config Generator
@@text
Typing the same config on twenty branch routers causes typos. Describe each site in sites.csv (name, site number, WAN IP, LAN subnet) and generate complete, consistent IOS configs from a template: hostname, interfaces, DHCP pool, OSPF, SSH-only management, NTP, logging and banners.

**You'll practise:** IOS configuration · templates · subnet math · network automation basics

## Milestones

1. **Template**: a full branch config with `$placeholders` (string.Template).
2. **Data**: sites.csv with five sites.
3. **Derive**: gateway, DHCP range and OSPF network statements from the LAN subnet.
4. **Generate**: one .cfg file per site in out/.
5. **Test**: paste one config into Packet Tracer or CML and check it works.

## You're done when

- Five valid configs are generated and one of them works in a lab.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I compute the wildcard mask for an OSPF network statement?"*
- *"What should every branch router config include for security?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project config_generator Branch Router Config Generator
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Branch Router Config Generator

Typing the same config on twenty branch routers causes typos. Describe each site in sites.csv (name, site number, WAN IP, LAN subnet) and generate complete, consistent IOS configs from a template: hostname, interfaces, DHCP pool, OSPF, SSH-only management, NTP, logging and banners.

## Milestones

- [ ] 1. Template: a full branch config with `$placeholders` (string.Template).
- [ ] 2. Data: sites.csv with five sites.
- [ ] 3. Derive: gateway, DHCP range and OSPF network statements from the LAN subnet.
- [ ] 4. Generate: one .cfg file per site in out/.
- [ ] 5. Test: paste one config into Packet Tracer or CML and check it works.

## Done when

- [ ] Five valid configs are generated and one of them works in a lab.
@@file generate.py
"""Generate branch configs: python generate.py sites.csv"""
import csv
import ipaddress
import sys
from pathlib import Path
from string import Template

template = Template(Path("branch.tpl").read_text(encoding="utf-8"))
out = Path("out")
out.mkdir(exist_ok=True)
for site in csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "sites.csv", encoding="utf-8")):
    lan = ipaddress.ip_network(site["lan"])
    values = dict(site, gateway=str(lan.network_address + 1), mask=str(lan.netmask), wildcard=str(lan.hostmask))
    (out / f"{site['name']}.cfg").write_text(template.substitute(values), encoding="utf-8")
    print("wrote", site["name"])
@@file branch.tpl
hostname $name
!
interface GigabitEthernet0/0
 description WAN
 ip address $wan 255.255.255.252
 no shutdown
!
interface GigabitEthernet0/1
 description LAN
 ip address $gateway $mask
 no shutdown
!
router ospf 1
 network $lan_net $wildcard area $site
!
@@file sites.csv
name,site,wan,lan,lan_net
BR-OSAKA,1,203.0.113.2,10.1.1.0/24,10.1.1.0
BR-NAGOYA,2,203.0.113.6,10.1.2.0/24,10.1.2.0
