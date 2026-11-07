@@lesson c2-automation Wireless Architectures and Network Automation
@@text
## Cisco wireless

- **Autonomous APs** are configured one by one; **lightweight APs** are managed by a **WLC** (wireless LAN controller) through **CAPWAP** tunnels (UDP 5246 control, 5247 data).
- Deployments: on-premises WLC, **FlexConnect** (APs at branches switch traffic locally if the WLC link fails), **cloud-managed** (Meraki), embedded controllers.
- AP modes: **local** (serves clients), **monitor**, **sniffer**, **rogue detector**, **bridge/mesh**, **FlexConnect**, **SE-Connect** (spectrum analysis).
- WLC ports: service, distribution system (to the switch, usually a trunk / LAG), management interface, dynamic interfaces (one per WLAN/VLAN).
- WLAN setup in the GUI: SSID, VLAN/interface, security (WPA2/WPA3 Personal or Enterprise with a RADIUS server), QoS profile. Know the GUI screens for the exam's simulations.

## Automation and programmability

**Why**: thousands of devices can't be configured by hand consistently. Automation reduces errors, enforces standards, and makes changes repeatable and reviewable.

- **Controller-based networking / SDN**: a controller (**Cisco Catalyst Center**, formerly DNA Center) holds the policy and programs devices. **Northbound APIs** (REST) are for apps talking to the controller; **southbound APIs** (NETCONF, RESTCONF, OpenFlow, CLI/SSH) are for the controller talking to devices.
- **Underlay** (the physical network and routing), **overlay** (virtual tunnels on top, e.g. VXLAN), **fabric**. Cisco **SD-Access** is the campus fabric.
- **REST APIs**: HTTP methods **GET** (read), **POST** (create), **PUT/PATCH** (replace/update), **DELETE**; status codes 200 OK, 201 Created, 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 500 Server Error. Authentication with tokens or basic auth.
- **Data formats**: **JSON** (the CCNA one to know: objects `{}`, arrays `[]`, key/value pairs, strings in double quotes), XML, YAML.
- **Configuration management**: **Ansible** (agentless, SSH, YAML playbooks, push), **Puppet** and **Chef** (agent-based, pull), **Terraform**.
- **AI and machine learning** in networking operations: predictive analytics, anomaly detection, generative AI assistants.

```json
{
  "hostname": "SW1",
  "interfaces": [
    {"name": "GigabitEthernet0/1", "vlan": 10, "enabled": true},
    {"name": "GigabitEthernet0/2", "vlan": 20, "enabled": false}
  ]
}
```

@@quiz
Lightweight APs communicate with their WLC through which protocol?
* CAPWAP
- LWAPP over TCP 80
- RADIUS
- SNMP
? CAPWAP uses UDP 5246 (control) and 5247 (data).

@@quiz
A branch's WAN link to the central WLC fails, but users should keep working on local resources. Which AP mode helps?
* FlexConnect
- Monitor
- Sniffer
- Rogue detector
? FlexConnect APs can switch traffic locally.

@@quiz
In SDN, which API does a network management application use to talk to the controller?
* Northbound API (usually REST)
- Southbound API
- CAPWAP
- OpenFlow only
? Southbound APIs connect the controller to the devices.

@@quiz
Which HTTP method should a script use to read a list of devices from a REST API?
* GET
- POST
- PUT
- DELETE
? GET reads; POST creates.

@@quiz
In this JSON, what type is the value of "interfaces"? `{"hostname": "SW1", "interfaces": [{"name": "Gi0/1"}]}`
* An array of objects
- A string
- An object of arrays
- A boolean
? `[...]` is an array; each `{...}` inside is an object.

@@quiz
Which configuration management tool is agentless and uses SSH and YAML playbooks?
* Ansible
- Puppet
- Chef
- SNMP
? Puppet and Chef use agents installed on the managed nodes.

@@flashcards
WLC :: Wireless LAN controller
CAPWAP :: AP-to-WLC tunnels, UDP 5246/5247
FlexConnect :: AP switches traffic locally at branches
autonomous vs lightweight AP :: Configured alone vs managed by a WLC
SDN :: Central controller programs the network
northbound API :: Apps to controller (REST)
southbound API :: Controller to devices (NETCONF, RESTCONF, OpenFlow)
Catalyst Center :: Cisco's campus controller (formerly DNA Center)
underlay / overlay :: Physical network / virtual tunnels on top
REST methods :: GET read, POST create, PUT/PATCH update, DELETE remove
HTTP 401 vs 403 :: Not authenticated vs not allowed
JSON :: Objects {}, arrays [], "key": value
Ansible :: Agentless, SSH, YAML, push
Puppet / Chef :: Agent-based configuration management

@@drill
Which HTTP status code means "Created"?
= 201
? 200 is OK; 204 No Content.

@@exercise Map CRUD actions to REST calls
A script talks to a controller's REST API. Write `rest_call(action, resource)` that returns the HTTP method and URL path for `read`, `create`, `update` and `delete` (`GET`, `POST`, `PUT`, `DELETE`; the path is `/api/v1/` followed by the resource, which for updates and deletes includes the item's id, like `vlans/20`).

Also write `explain(status)` for the status codes in the lesson (200, 201, 204, 400, 401, 403, 404, 500), saying whether the script should retry: only 500 is worth retrying; 401 means fix the token. Print each planned call with the controller's reply.
@@lang python
@@starter
METHOD = {"read": "GET", "create": "POST", "update": "PUT", "delete": "DELETE"}
MEANING = {
    200: "OK", 201: "Created", 204: "No Content", 400: "Bad Request",
    401: "Unauthorized", 403: "Forbidden", 404: "Not Found", 500: "Server Error",
}
calls = [
    ("read", "devices", 200),
    ("create", "vlans", 201),
    ("update", "vlans/20", 400),
    ("delete", "vlans/30", 204),
    ("read", "interfaces", 401),
    ("delete", "users/admin", 403),
    ("read", "devices/99", 404),
    ("create", "wlans", 500),
]

def rest_call(action, resource):
    raise NotImplementedError
def explain(status):
    raise NotImplementedError


for action, resource, status in calls:
    method, path = rest_call(action, resource)
    print(f"{action:<7}{method:<7}{path:<24}{explain(status)}")
@@expected
read   GET    /api/v1/devices         200 OK
create POST   /api/v1/vlans           201 Created
update PUT    /api/v1/vlans/20        400 Bad Request: fix the request, don't retry
delete DELETE /api/v1/vlans/30        204 No Content
read   GET    /api/v1/interfaces      401 Unauthorized: get a new token
delete DELETE /api/v1/users/admin     403 Forbidden: fix the request, don't retry
read   GET    /api/v1/devices/99      404 Not Found: fix the request, don't retry
create POST   /api/v1/wlans           500 Server Error: retry later
@@requires
def rest_call
def explain
/api/v1/
@@hint
Two dicts do most of the work: action to method, and status code to its name. `explain` then only needs the retry rules.
@@solution
METHOD = {"read": "GET", "create": "POST", "update": "PUT", "delete": "DELETE"}
MEANING = {
    200: "OK", 201: "Created", 204: "No Content", 400: "Bad Request",
    401: "Unauthorized", 403: "Forbidden", 404: "Not Found", 500: "Server Error",
}
calls = [
    ("read", "devices", 200),
    ("create", "vlans", 201),
    ("update", "vlans/20", 400),
    ("delete", "vlans/30", 204),
    ("read", "interfaces", 401),
    ("delete", "users/admin", 403),
    ("read", "devices/99", 404),
    ("create", "wlans", 500),
]

def rest_call(action, resource):
    return METHOD[action], "/api/v1/" + resource

def explain(status):
    text = f"{status} {MEANING[status]}"
    if status == 500:
        return text + ": retry later"
    if status == 401:
        return text + ": get a new token"
    if status >= 400:
        return text + ": fix the request, don't retry"
    return text

for action, resource, status in calls:
    method, path = rest_call(action, resource)
    print(f"{action:<7}{method:<7}{path:<24}{explain(status)}")

@@exercise Choose the AP mode for each job
Pick an access point mode from the lesson's list for each requirement. Write `ap_mode(need)`:

- serve clients at a **branch** where traffic should stay local if the WAN fails → `FlexConnect`;
- serve clients at the main site → `local`;
- only scan for rogue APs and intrusions, no clients → `monitor`;
- capture packets for Wireshark → `sniffer`;
- find non-Wi-Fi interference (microwaves, Bluetooth) → `SE-Connect`;
- link two buildings without a cable → `bridge/mesh`.

Then count how many APs of each mode the plan needs and warn if any site has no AP serving clients.
@@lang python
@@starter
plan = [
    ("Tokyo HQ", "serve clients", False),
    ("Tokyo HQ", "serve clients", False),
    ("Tokyo HQ", "scan for rogue APs", False),
    ("Osaka branch", "serve clients", True),
    ("Osaka branch", "serve clients", True),
    ("Warehouse", "link two buildings", False),
    ("Warehouse", "find interference", False),
    ("Lab", "capture packets", False),
]
MODES = {
    "scan for rogue APs": "monitor",
    "capture packets": "sniffer",
    "find interference": "SE-Connect",
    "link two buildings": "bridge/mesh",
}

def ap_mode(need, branch):
    raise NotImplementedError


counts, serving = {}, {}
for site, need, branch in plan:
    mode = ap_mode(need, branch)
    counts[mode] = counts.get(mode, 0) + 1
    serving.setdefault(site, False)
    serving[site] |= mode in ("local", "FlexConnect")
    print(f"{site}: {need} -> {mode}")
print("APs per mode: " + ", ".join(f"{m} {n}" for m, n in sorted(counts.items())))
for site, ok in serving.items():
    if not ok:
        print(f"warning: no AP serves clients at {site}")
@@expected
Tokyo HQ: serve clients -> local
Tokyo HQ: serve clients -> local
Tokyo HQ: scan for rogue APs -> monitor
Osaka branch: serve clients -> FlexConnect
Osaka branch: serve clients -> FlexConnect
Warehouse: link two buildings -> bridge/mesh
Warehouse: find interference -> SE-Connect
Lab: capture packets -> sniffer
APs per mode: FlexConnect 2, SE-Connect 1, bridge/mesh 1, local 2, monitor 1, sniffer 1
warning: no AP serves clients at Warehouse
warning: no AP serves clients at Lab
@@requires
def ap_mode
setdefault(
@@hint
Client-serving APs depend on the site: local at the main site, FlexConnect at a branch. The other needs map straight to a mode with a dict.
@@solution
plan = [
    ("Tokyo HQ", "serve clients", False),
    ("Tokyo HQ", "serve clients", False),
    ("Tokyo HQ", "scan for rogue APs", False),
    ("Osaka branch", "serve clients", True),
    ("Osaka branch", "serve clients", True),
    ("Warehouse", "link two buildings", False),
    ("Warehouse", "find interference", False),
    ("Lab", "capture packets", False),
]
MODES = {
    "scan for rogue APs": "monitor",
    "capture packets": "sniffer",
    "find interference": "SE-Connect",
    "link two buildings": "bridge/mesh",
}

def ap_mode(need, branch):
    if need == "serve clients":
        return "FlexConnect" if branch else "local"
    return MODES[need]

counts, serving = {}, {}
for site, need, branch in plan:
    mode = ap_mode(need, branch)
    counts[mode] = counts.get(mode, 0) + 1
    serving.setdefault(site, False)
    serving[site] |= mode in ("local", "FlexConnect")
    print(f"{site}: {need} -> {mode}")
print("APs per mode: " + ", ".join(f"{m} {n}" for m, n in sorted(counts.items())))
for site, ok in serving.items():
    if not ok:
        print(f"warning: no AP serves clients at {site}")
