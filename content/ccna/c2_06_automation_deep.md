@@lesson c2-automation-deep Automation and Programmability: Controllers, REST APIs, JSON and Config Management
@@text
Automation and programmability is 10% of CCNA. You need to compare traditional and controller-based networking, describe SDN planes and APIs, read JSON, understand REST and HTTP status codes, and know what Ansible and Terraform do. You won't write big programs on the exam, but this is how modern networks are run.

## Planes

- **Data plane**: forwards packets (switching ASICs, NAT, ACL lookups, encapsulation).
- **Control plane**: decides how to forward (OSPF, STP, ARP, MAC learning).
- **Management plane**: how admins and tools manage the device (SSH, SNMP, syslog, APIs).

Traditional networking: each device has its own control plane and is configured box by box. **Controller-based / SDN**: a **controller** centralises (part of) the control and management planes.

- **Northbound API** (NBI): between the controller and apps/scripts, usually **REST** over HTTPS with JSON.
- **Southbound API** (SBI): between the controller and devices: **NETCONF**, **RESTCONF**, OpenFlow, SSH/CLI, SNMP, gNMI.
- **Underlay** (physical network and routing) vs **overlay** (virtual tunnels such as VXLAN on top) vs **fabric** (the whole).

Cisco examples: **Catalyst Center** (formerly DNA Center) for campus (SD-Access), **SD-WAN** (vManage/Manager), **Meraki** dashboard, **ACI** for data centres. Benefits: consistency, faster changes, intent-based policies, assurance/telemetry.

## REST APIs

REST uses HTTP methods on resources (URIs):

| CRUD | HTTP method |
|---|---|
| Create | **POST** |
| Read | **GET** |
| Update | **PUT** (replace) / **PATCH** (modify) |
| Delete | **DELETE** |

Status codes: **2xx** success (200 OK, 201 Created, 204 No Content), **3xx** redirect, **4xx** client error (400 Bad Request, **401 Unauthorized** (not authenticated), **403 Forbidden** (no permission), **404 Not Found**), **5xx** server error (500, 503).

Characteristics: client–server, **stateless** (each request carries its auth, often a **token** in a header such as `X-Auth-Token` or `Authorization: Bearer`), cacheable, uniform interface.

```text
GET https://catalyst-center.example/dna/intent/api/v1/network-device
X-Auth-Token: eyJhbGciOi...
Accept: application/json

HTTP/1.1 200 OK
Content-Type: application/json
{"response": [{"hostname": "edge-sw1", "managementIpAddress": "10.0.99.11", "platformId": "C9300-48P", "softwareVersion": "17.9.4", "reachabilityStatus": "Reachable"}]}
```

## JSON

Data types: **object** `{}` (key/value pairs, keys are strings in double quotes), **array** `[]`, string, number, `true`/`false`, `null`. No comments, no trailing commas.

```json,norun
{
  "interface": {
    "name": "GigabitEthernet1/0/1",
    "enabled": true,
    "vlan": 10,
    "ipv4": null,
    "tags": ["user", "floor2"]
  }
}
```

Compare with **XML** (`<vlan>10</vlan>`, used by NETCONF) and **YAML** (indentation-based, used by Ansible). **YANG** is the data *model* that defines what the JSON/XML must look like.

## Configuration management

| Tool | Style | Agent | Language | Notes |
|---|---|---|---|---|
| **Ansible** | push, procedural playbooks | **agentless** (SSH, NETCONF, APIs) | YAML playbooks, Jinja2 templates | inventory of hosts, idempotent modules (`ios_config`, `ios_vlans`) |
| **Terraform** | declarative infrastructure as code | agentless (provider APIs) | HCL | `plan` then `apply`, keeps **state** |
| **Puppet** | pull, declarative | agent (or proxy for network devices) | Puppet DSL | manifests |
| **Chef** | pull | agent | Ruby | recipes, cookbooks |

Benefits: consistent configs, version control (Git), templates, fewer manual mistakes, **configuration drift** detection.

```yaml,norun
- name: Ensure VLANs exist
  hosts: access_switches
  gather_facts: false
  tasks:
    - name: VLANs
      cisco.ios.ios_vlans:
        config:
          - vlan_id: 10
            name: USERS
          - vlan_id: 20
            name: VOICE
        state: merged
```

## AI in network operations (blueprint 1.1)

The current blueprint also mentions AI/ML: predictive analytics and anomaly detection in controllers (assurance), generative AI assistants for configuration and troubleshooting. Always review AI-suggested configs before applying.

@@quiz
Which plane runs OSPF and builds the routing table?
- Data plane
* Control plane
- Management plane
- Overlay
? The data plane forwards using the result.

@@quiz
In SDN, what is the API between the controller and applications called?
* Northbound API
- Southbound API
- Eastbound API
- Underlay API
? Southbound faces the devices.

@@quiz
Which HTTP method creates a new resource in a REST API?
- GET
* POST
- PUT
- DELETE
? PUT replaces an existing resource.

@@quiz
A REST call returns 401. What does it mean?
- The resource doesn't exist
* The request isn't authenticated (missing or invalid token)
- The server crashed
- Success with no content
? 403 means authenticated but not allowed.

@@quiz
Which statement about Ansible is true?
- It requires an agent on every switch
* It is agentless and uses YAML playbooks
- It uses Ruby recipes
- It only works with Cisco devices
? It connects over SSH or APIs.

@@quiz
Which is valid JSON?
- `{name: "sw1", vlan: 10}`
* `{"name": "sw1", "vlan": 10}`
- `{"name": 'sw1', "vlan": 10,}`
- `<name>sw1</name>`
? Keys and strings use double quotes; no trailing commas.

@@flashcards
data plane :: forwards packets
control plane :: routing, STP, ARP decisions
management plane :: SSH, SNMP, APIs
northbound API :: controller to apps (REST)
southbound API :: controller to devices
NETCONF :: XML over SSH (port 830)
RESTCONF :: REST with YANG data
YANG :: data modelling language
underlay / overlay :: physical / virtual tunnels
Catalyst Center :: campus controller (DNA Center)
POST / GET :: create / read
PUT / PATCH :: replace / modify
200 / 201 / 204 :: OK / Created / No Content
401 / 403 / 404 :: unauthenticated / forbidden / not found
REST stateless :: auth with every request
JSON object / array :: {} / []
Ansible :: agentless, YAML, push
Terraform :: declarative IaC, HCL, state
Puppet / Chef :: agent-based, pull
configuration drift :: device differs from intended config

@@drill
Which HTTP status code means "Created"?
= 201
? Returned after a successful POST.

@@drill
Which data format do Ansible playbooks use?
= YAML
= yaml
? Templates use Jinja2.

@@exercise Parse a controller's device inventory
`response` is JSON text from a controller's network-device API. Parse it with the `json` module and print a table of hostname, management IP, platform and version sorted by hostname. Then report unreachable devices, devices not on the `TARGET` version, and the count per platform.
@@lang python
@@starter
import json
from collections import Counter

TARGET = "17.9.4"
response = '''{"response": [
 {"hostname": "edge-sw2", "managementIpAddress": "10.0.99.12", "platformId": "C9300-48P", "softwareVersion": "17.6.5", "reachabilityStatus": "Reachable"},
 {"hostname": "core-sw1", "managementIpAddress": "10.0.99.2", "platformId": "C9500-24Y4C", "softwareVersion": "17.9.4", "reachabilityStatus": "Reachable"},
 {"hostname": "edge-sw1", "managementIpAddress": "10.0.99.11", "platformId": "C9300-48P", "softwareVersion": "17.9.4", "reachabilityStatus": "Reachable"},
 {"hostname": "wan-r1", "managementIpAddress": "10.0.99.1", "platformId": "C8300-1N1S", "softwareVersion": "17.9.4", "reachabilityStatus": "Unreachable"},
 {"hostname": "edge-sw3", "managementIpAddress": "10.0.99.13", "platformId": "C9200L-24P", "softwareVersion": "17.6.5", "reachabilityStatus": "Reachable"}
], "version": "1.0"}'''
@@expected
Hostname  Mgmt IP     Platform     Version
core-sw1  10.0.99.2   C9500-24Y4C  17.9.4
edge-sw1  10.0.99.11  C9300-48P    17.9.4
edge-sw2  10.0.99.12  C9300-48P    17.6.5
edge-sw3  10.0.99.13  C9200L-24P   17.6.5
wan-r1    10.0.99.1   C8300-1N1S   17.9.4
Unreachable: wan-r1
Not on 17.9.4: edge-sw2, edge-sw3
  C8300-1N1S: 1
  C9200L-24P: 1
  C9300-48P: 2
  C9500-24Y4C: 1
@@requires
json.loads(
["response"]
@@hint
`devices = json.loads(response)["response"]`, then `sorted(devices, key=lambda d: d["hostname"])`.
@@solution
import json
from collections import Counter

TARGET = "17.9.4"
response = '''{"response": [
 {"hostname": "edge-sw2", "managementIpAddress": "10.0.99.12", "platformId": "C9300-48P", "softwareVersion": "17.6.5", "reachabilityStatus": "Reachable"},
 {"hostname": "core-sw1", "managementIpAddress": "10.0.99.2", "platformId": "C9500-24Y4C", "softwareVersion": "17.9.4", "reachabilityStatus": "Reachable"},
 {"hostname": "edge-sw1", "managementIpAddress": "10.0.99.11", "platformId": "C9300-48P", "softwareVersion": "17.9.4", "reachabilityStatus": "Reachable"},
 {"hostname": "wan-r1", "managementIpAddress": "10.0.99.1", "platformId": "C8300-1N1S", "softwareVersion": "17.9.4", "reachabilityStatus": "Unreachable"},
 {"hostname": "edge-sw3", "managementIpAddress": "10.0.99.13", "platformId": "C9200L-24P", "softwareVersion": "17.6.5", "reachabilityStatus": "Reachable"}
], "version": "1.0"}'''

devices = json.loads(response)["response"]
print(f"{'Hostname':<10}{'Mgmt IP':<12}{'Platform':<13}Version")
for d in sorted(devices, key=lambda d: d["hostname"]):
    print(f"{d['hostname']:<10}{d['managementIpAddress']:<12}{d['platformId']:<13}{d['softwareVersion']}")

down = [d["hostname"] for d in devices if d["reachabilityStatus"] != "Reachable"]
old = sorted(d["hostname"] for d in devices if d["softwareVersion"] != TARGET)
print("Unreachable:", ", ".join(down) or "none")
print(f"Not on {TARGET}:", ", ".join(old) or "none")
for platform, count in sorted(Counter(d["platformId"] for d in devices).items()):
    print(f"  {platform}: {count}")

@@exercise Desired state vs actual: VLAN drift
Like an idempotent Ansible module with `state: merged` or `replaced`, compare the desired VLANs (from YAML-like data already parsed into a dict) with each switch's actual VLANs (JSON text). For each switch print the actions needed: `create` missing VLANs, `rename` VLANs with a different name, and, only when `STATE == "replaced"`, `delete` VLANs that aren't desired (never VLAN 1). Print `no changes (idempotent)` when nothing differs. Run for both states.
@@lang python
@@starter
import json

desired = {10: "USERS", 20: "VOICE", 30: "GUEST", 99: "MGMT"}
actual_json = '''{
 "edge-sw1": [{"id": 1, "name": "default"}, {"id": 10, "name": "USERS"}, {"id": 20, "name": "VOICE"}, {"id": 30, "name": "GUEST"}, {"id": 99, "name": "MGMT"}],
 "edge-sw2": [{"id": 1, "name": "default"}, {"id": 10, "name": "Users"}, {"id": 20, "name": "VOICE"}, {"id": 50, "name": "OLD-LAB"}],
 "edge-sw3": [{"id": 1, "name": "default"}, {"id": 10, "name": "USERS"}]
}'''

def plan(actual, state):
    raise NotImplementedError
@@expected
state: merged
  edge-sw1: no changes (idempotent)
  edge-sw2: rename 10 Users -> USERS; create 30 GUEST; create 99 MGMT
  edge-sw3: create 20 VOICE; create 30 GUEST; create 99 MGMT
state: replaced
  edge-sw1: no changes (idempotent)
  edge-sw2: rename 10 Users -> USERS; create 30 GUEST; create 99 MGMT; delete 50 OLD-LAB
  edge-sw3: create 20 VOICE; create 30 GUEST; create 99 MGMT
@@requires
def plan
json.loads(
@@hint
Turn each switch's list into `{v["id"]: v["name"] for v in vlans}`. Loop over `sorted(desired)` for create/rename, then over `sorted(actual)` for delete.
@@solution
import json

desired = {10: "USERS", 20: "VOICE", 30: "GUEST", 99: "MGMT"}
actual_json = '''{
 "edge-sw1": [{"id": 1, "name": "default"}, {"id": 10, "name": "USERS"}, {"id": 20, "name": "VOICE"}, {"id": 30, "name": "GUEST"}, {"id": 99, "name": "MGMT"}],
 "edge-sw2": [{"id": 1, "name": "default"}, {"id": 10, "name": "Users"}, {"id": 20, "name": "VOICE"}, {"id": 50, "name": "OLD-LAB"}],
 "edge-sw3": [{"id": 1, "name": "default"}, {"id": 10, "name": "USERS"}]
}'''

def plan(actual, state):
    actions = []
    for vid in sorted(desired):
        if vid not in actual:
            actions.append(f"create {vid} {desired[vid]}")
        elif actual[vid] != desired[vid]:
            actions.append(f"rename {vid} {actual[vid]} -> {desired[vid]}")
    if state == "replaced":
        for vid in sorted(actual):
            if vid != 1 and vid not in desired:
                actions.append(f"delete {vid} {actual[vid]}")
    return actions

switches = json.loads(actual_json)
for state in ("merged", "replaced"):
    print(f"state: {state}")
    for name, vlans in switches.items():
        actual = {v["id"]: v["name"] for v in vlans}
        actions = plan(actual, state)
        print(f"  {name}: " + ("; ".join(actions) if actions else "no changes (idempotent)"))
