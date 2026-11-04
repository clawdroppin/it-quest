@@lesson cp1-automation-assurance-deep Automation and Assurance in Depth: YANG, NETCONF, RESTCONF, Telemetry, IP SLA and NetFlow
@@text
Automation is 15% of ENCOR and assurance another 10%. You're expected to read JSON and YANG-modelled data, know which protocol uses which transport and operations, interpret HTTP status codes from a REST API, and know what IP SLA, NetFlow, SNMP and model-driven telemetry each measure. This lesson covers those facts and has you write the kind of small scripts network engineers actually use: parsing interface data and comparing intended with actual configuration.

## Data models and encodings

- **YANG** describes the data (containers, lists, leaves, types); models come from IETF (e.g. `ietf-interfaces`), OpenConfig, and Cisco native.
- Encodings: **XML** (NETCONF), **JSON** or XML (RESTCONF). JSON types: object `{}`, array `[]`, string, number, true/false, null.

## NETCONF vs RESTCONF

| | NETCONF | RESTCONF |
|---|---|---|
| Transport | **SSH, port 830** | **HTTPS** (443) |
| Encoding | XML | JSON or XML |
| Operations | `<get>`, `<get-config>`, `<edit-config>`, `<copy-config>`, `<delete-config>`, `<lock>`, `<commit>` | GET, POST (create), PUT (create/replace), PATCH (merge), DELETE |
| Datastores | running, candidate, startup | unified datastore view |

RESTCONF URL: `https://router/restconf/data/ietf-interfaces:interfaces/interface=GigabitEthernet1`, header `Accept: application/yang-data+json`.

## REST APIs and status codes

- 200 OK, **201 Created**, **204 No Content** (success, nothing returned), 400 Bad Request, **401 Unauthorized** (not authenticated), **403 Forbidden** (authenticated, not allowed), 404 Not Found, 409 Conflict, 429 Too Many Requests, 500 Internal Server Error.
- Authentication: basic, tokens (Catalyst Center gets a token from `/dna/system/api/v1/auth/token`), API keys, OAuth.
- Controllers: **Catalyst Center** (formerly DNA Center) northbound REST APIs and intent-based workflows; **SD-WAN Manager** (vManage); southbound NETCONF/CLI/SNMP to devices.

## Configuration management tools

| Tool | Model | Agent | Language |
|---|---|---|---|
| **Ansible** | push | **agentless** (SSH, NETCONF, APIs) | YAML playbooks |
| Puppet | pull | agent (agentless proxies for network) | Ruby DSL |
| Chef | pull | agent | Ruby |
| SaltStack | push/pull | agent or SSH | YAML/Python |

Plus **EEM** (Embedded Event Manager) for on-box automation and Python with Netmiko/ncclient/requests off-box. Source of truth + templates (Jinja2) + version control + idempotent pushes.

## Assurance tools

- **IP SLA**: synthetic probes (ICMP echo, UDP jitter, HTTP) measuring reachability, latency, jitter, loss; tied to **object tracking** for static route or HSRP failover.
- **NetFlow / Flexible NetFlow / IPFIX**: flow records (5-tuple + counters) exported to a collector for who-talks-to-whom and capacity planning.
- **SNMP**: v2c (community strings, no encryption) vs **v3** (users, authPriv = authentication and encryption); polling vs traps/informs.
- **Model-driven telemetry**: devices **stream** YANG data (periodic or on-change) over gRPC/gNMI or NETCONF dial-in: more scalable than polling.
- **Syslog** severities 0 emergency … 7 debugging ("Every Awesome Cisco Engineer Will Need Ice cream Daily"), **SPAN/RSPAN/ERSPAN** for packet capture, **debug conditions** to limit debugs.

@@quiz
Which transport and port does NETCONF use?
* SSH, TCP 830
- HTTPS, TCP 443
- UDP 161
- gRPC 57400
? RESTCONF uses HTTPS.

@@quiz
Which RESTCONF method merges changes into existing configuration?
- PUT
* PATCH
- POST
- GET
? PUT replaces.

@@quiz
An API call returns 401. What does it mean?
- The resource doesn't exist
* The request wasn't authenticated
- Authenticated but not permitted
- The server crashed
? 403 is authenticated but not allowed.

@@quiz
Which configuration management tool is agentless and push-based?
* Ansible
- Puppet
- Chef
- SaltStack with minions
? Over SSH, NETCONF or APIs.

@@quiz
Which feature generates synthetic traffic to measure jitter and can trigger a route change through object tracking?
- NetFlow
* IP SLA
- SPAN
- SNMP traps
? Combine with track objects.

@@quiz
Which SNMP version provides authentication and encryption?
- v1
- v2c
* v3 with authPriv
- All of them
? v2c uses community strings in clear text.

@@flashcards
YANG :: data modelling language
NETCONF :: SSH 830, XML
RESTCONF :: HTTPS, JSON/XML
edit-config :: NETCONF change
candidate datastore :: edit then commit
PUT :: create or replace
PATCH :: merge
POST :: create
201 :: created
204 :: success, no content
401 :: not authenticated
403 :: not allowed
429 :: rate limited
Ansible :: agentless push, YAML
Puppet :: agent pull
EEM :: on-box automation
IP SLA :: synthetic probes
object tracking :: react to IP SLA
NetFlow :: flow records to a collector
SNMPv3 authPriv :: auth + encryption
model-driven telemetry :: streaming over gRPC
syslog 0 :: emergency
syslog 7 :: debugging
ERSPAN :: remote capture over GRE

@@drill
What TCP port does NETCONF over SSH use?
= 830
? Not 22.

@@drill
Which HTTP status code means "created"?
= 201
? 200 is OK, 204 no content.

@@exercise Parse RESTCONF interface data
Parse a RESTCONF JSON response (ietf-interfaces with operational state merged in) and report each interface's admin/oper status, IP addresses and error counters. Flag interfaces that are admin up but oper down, and those with input errors above `ERROR_LIMIT`.
@@lang python
@@starter
import json

ERROR_LIMIT = 100
response = """{
  "ietf-interfaces:interfaces": {
    "interface": [
      {"name": "GigabitEthernet1", "enabled": true, "oper-status": "up",
       "ietf-ip:ipv4": {"address": [{"ip": "10.0.0.1", "netmask": "255.255.255.0"}]},
       "statistics": {"in-errors": 3, "out-errors": 0}},
      {"name": "GigabitEthernet2", "enabled": true, "oper-status": "down",
       "ietf-ip:ipv4": {"address": [{"ip": "10.0.1.1", "netmask": "255.255.255.252"}]},
       "statistics": {"in-errors": 0, "out-errors": 0}},
      {"name": "GigabitEthernet3", "enabled": false, "oper-status": "down",
       "statistics": {"in-errors": 0, "out-errors": 0}},
      {"name": "Loopback0", "enabled": true, "oper-status": "up",
       "ietf-ip:ipv4": {"address": [{"ip": "192.0.2.1", "netmask": "255.255.255.255"}]},
       "statistics": {"in-errors": 0, "out-errors": 0}},
      {"name": "GigabitEthernet4", "enabled": true, "oper-status": "up",
       "ietf-ip:ipv4": {"address": [{"ip": "10.0.2.1", "netmask": "255.255.255.0"}]},
       "statistics": {"in-errors": 4512, "out-errors": 7}}
    ]
  }
}"""

def summarize(interface):
    raise NotImplementedError
@@expected
GigabitEthernet1  up         up    10.0.0.1/24
GigabitEthernet2  up         down  10.0.1.1/30      ADMIN UP / OPER DOWN
GigabitEthernet3  admin-down down  -
Loopback0         up         up    192.0.2.1/32
GigabitEthernet4  up         up    10.0.2.1/24      4512 input errors
@@requires
def summarize
json.loads(
ietf-ip:ipv4
@@hint
`data["ietf-interfaces:interfaces"]["interface"]` is the list. Use `.get("ietf-ip:ipv4", {}).get("address", [])` because not every interface has an address. Convert masks to prefix lengths with `ipaddress.IPv4Network(f"0.0.0.0/{mask}").prefixlen`.
@@solution
import ipaddress
import json

ERROR_LIMIT = 100
response = """{
  "ietf-interfaces:interfaces": {
    "interface": [
      {"name": "GigabitEthernet1", "enabled": true, "oper-status": "up",
       "ietf-ip:ipv4": {"address": [{"ip": "10.0.0.1", "netmask": "255.255.255.0"}]},
       "statistics": {"in-errors": 3, "out-errors": 0}},
      {"name": "GigabitEthernet2", "enabled": true, "oper-status": "down",
       "ietf-ip:ipv4": {"address": [{"ip": "10.0.1.1", "netmask": "255.255.255.252"}]},
       "statistics": {"in-errors": 0, "out-errors": 0}},
      {"name": "GigabitEthernet3", "enabled": false, "oper-status": "down",
       "statistics": {"in-errors": 0, "out-errors": 0}},
      {"name": "Loopback0", "enabled": true, "oper-status": "up",
       "ietf-ip:ipv4": {"address": [{"ip": "192.0.2.1", "netmask": "255.255.255.255"}]},
       "statistics": {"in-errors": 0, "out-errors": 0}},
      {"name": "GigabitEthernet4", "enabled": true, "oper-status": "up",
       "ietf-ip:ipv4": {"address": [{"ip": "10.0.2.1", "netmask": "255.255.255.0"}]},
       "statistics": {"in-errors": 4512, "out-errors": 7}}
    ]
  }
}"""

def summarize(interface):
    addresses = interface.get("ietf-ip:ipv4", {}).get("address", [])
    ips = [f"{a['ip']}/{ipaddress.IPv4Network('0.0.0.0/' + a['netmask']).prefixlen}" for a in addresses]
    admin = "up" if interface["enabled"] else "admin-down"
    flags = []
    if interface["enabled"] and interface["oper-status"] == "down":
        flags.append("ADMIN UP / OPER DOWN")
    if interface["statistics"]["in-errors"] > ERROR_LIMIT:
        flags.append(f"{interface['statistics']['in-errors']} input errors")
    return f"{interface['name']:<17} {admin:<10} {interface['oper-status']:<5} {', '.join(ips) or '-':<16} {'; '.join(flags)}"

data = json.loads(response)
for interface in data["ietf-interfaces:interfaces"]["interface"]:
    print(summarize(interface).rstrip())

@@exercise Detect configuration drift
Compare an intended configuration (source of truth) with the running configuration parsed from a device, both as nested dicts. Report missing items, unexpected items and changed values with their paths, then generate the RESTCONF calls that would fix them (PATCH for changes and additions, DELETE for unexpected items).
@@lang python
@@starter
intended = {
    "hostname": "edge-rtr-01",
    "ntp": {"servers": ["10.0.0.10", "10.0.0.11"]},
    "interfaces": {
        "GigabitEthernet1": {"description": "uplink ISP-A", "enabled": True, "mtu": 1500},
        "GigabitEthernet2": {"description": "LAN", "enabled": True, "mtu": 9000},
    },
    "snmp": {"version": "v3", "user": "monitor"},
}
running = {
    "hostname": "edge-rtr-01",
    "ntp": {"servers": ["10.0.0.10"]},
    "interfaces": {
        "GigabitEthernet1": {"description": "uplink ISP-A", "enabled": True, "mtu": 1500},
        "GigabitEthernet2": {"description": "LAN", "enabled": False, "mtu": 1500},
        "GigabitEthernet9": {"description": "temp test", "enabled": True, "mtu": 1500},
    },
    "snmp": {"version": "v2c", "community": "public"},
}

def diff(want, have, path=""):
    raise NotImplementedError
@@expected
CHANGED    /interfaces/GigabitEthernet2/enabled: False -> True
CHANGED    /interfaces/GigabitEthernet2/mtu: 1500 -> 9000
UNEXPECTED /interfaces/GigabitEthernet9: {'description': 'temp test', 'enabled': True, 'mtu': 1500}
CHANGED    /ntp/servers: ['10.0.0.10'] -> ['10.0.0.10', '10.0.0.11']
UNEXPECTED /snmp/community: 'public'
MISSING    /snmp/user: 'monitor'
CHANGED    /snmp/version: 'v2c' -> 'v3'
Remediation:
  PATCH  /restconf/data/interfaces/GigabitEthernet2/enabled True
  PATCH  /restconf/data/interfaces/GigabitEthernet2/mtu 9000
  DELETE /restconf/data/interfaces/GigabitEthernet9
  PATCH  /restconf/data/ntp/servers ['10.0.0.10', '10.0.0.11']
  DELETE /restconf/data/snmp/community
  PATCH  /restconf/data/snmp/user 'monitor'
  PATCH  /restconf/data/snmp/version 'v3'
@@requires
def diff
PATCH
DELETE
@@hint
Recurse when both values are dicts. Yield tuples like `("missing", path, value)`, `("unexpected", path, value)` and `("changed", path, (have, want))`. Lists compare as whole values.
@@solution
intended = {
    "hostname": "edge-rtr-01",
    "ntp": {"servers": ["10.0.0.10", "10.0.0.11"]},
    "interfaces": {
        "GigabitEthernet1": {"description": "uplink ISP-A", "enabled": True, "mtu": 1500},
        "GigabitEthernet2": {"description": "LAN", "enabled": True, "mtu": 9000},
    },
    "snmp": {"version": "v3", "user": "monitor"},
}
running = {
    "hostname": "edge-rtr-01",
    "ntp": {"servers": ["10.0.0.10"]},
    "interfaces": {
        "GigabitEthernet1": {"description": "uplink ISP-A", "enabled": True, "mtu": 1500},
        "GigabitEthernet2": {"description": "LAN", "enabled": False, "mtu": 1500},
        "GigabitEthernet9": {"description": "temp test", "enabled": True, "mtu": 1500},
    },
    "snmp": {"version": "v2c", "community": "public"},
}

def diff(want, have, path=""):
    for key in sorted(set(want) | set(have)):
        p = f"{path}/{key}"
        if key not in have:
            yield "missing", p, want[key]
        elif key not in want:
            yield "unexpected", p, have[key]
        elif isinstance(want[key], dict) and isinstance(have[key], dict):
            yield from diff(want[key], have[key], p)
        elif want[key] != have[key]:
            yield "changed", p, (have[key], want[key])

changes = list(diff(intended, running))
for kind, path, value in changes:
    if kind == "changed":
        print(f"CHANGED    {path}: {value[0]!r} -> {value[1]!r}")
    else:
        print(f"{kind.upper():<10} {path}: {value!r}")
print("Remediation:")
for kind, path, value in changes:
    url = "/restconf/data" + path
    if kind == "unexpected":
        print(f"  DELETE {url}")
    else:
        new = value[1] if kind == "changed" else value
        print(f"  PATCH  {url} {new!r}")
