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
