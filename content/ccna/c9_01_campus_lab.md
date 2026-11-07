@@lesson c9-campus-lab Campus Network Lab and Automation
@@text
Two halves, like a real network job:

1. **Build** a small campus network in **Packet Tracer**: VLANs, trunks, Rapid PVST+, router-on-a-stick, DHCP, OSPF, NAT, SSH with an ACL, NTP and syslog. `LAB.md` has the topology and 12 tasks; `configs/` has reference configurations to check yourself against *after* you've typed your own.
2. **Automate the checking**: network engineers don't read `show` output from 200 switches by eye. You'll write a parser that turns it into structured data (the CCNA automation domain), and a checker that flags problems:

```text
python main.py samples/r1_ip_int_brief.txt samples/sw1_vlan_brief.txt --vlans 10,20,30,99
8 interfaces, 9 VLANs checked
  x GigabitEthernet0/1 (10.1.12.1): up/down, check layer 2 (encapsulation, keepalives, the other end)
  x GigabitEthernet0/2 (203.0.113.2): shut down
  x VLAN 30 is missing or not active
3 problem(s)
```

With `--json` it prints the parsed data, ready for another tool, a dashboard or a REST API. With `--host` it logs into a real device over SSH using **Netmiko** (`pip install netmiko`), the most popular Python library for network automation.

Real show output is messy: a prompt line, headers, a two-word status (`administratively down`), and port lists that wrap onto indented lines. Handling that is the whole skill. (Production tools use TextFSM or Genie templates for the same job.)
@@project campus_lab Campus Network Lab
Do the Packet Tracer lab in `LAB.md`. Then write the four functions in `showparse.py` until `python -m unittest` passes, and run the checker on output saved from your own lab.
@@file showparse.py
"""Turn Cisco IOS "show" output into structured data, and check it."""


def parse_ip_int_brief(text):
    """Parse `show ip interface brief` into [{"interface", "ip", "status", "protocol"}, ...].
    ip is None for "unassigned". Status may be two words ("administratively down")."""
    raise NotImplementedError("TODO: parse_ip_int_brief")


def parse_vlan_brief(text):
    """Parse `show vlan brief` into {vlan_id: {"name", "status", "ports": [...]}}.
    Long port lists continue on the next line(s), indented."""
    raise NotImplementedError("TODO: parse_vlan_brief")


def interface_problems(interfaces):
    """Messages for interfaces that have an IP address but aren't up/up:
      "Gi0/1 (10.1.1.1): shut down"                      status "administratively down"
      "Gi0/1 (10.1.1.1): down/down, check the cable and the other device"
      "Gi0/1 (10.1.1.1): up/down, check layer 2 (encapsulation, keepalives, the other end)"
    Interfaces without an IP are ignored (unused ports are often shut on purpose)."""
    raise NotImplementedError("TODO: interface_problems")


def missing_vlans(vlans, required):
    """The required VLAN ids that don't exist or aren't active, sorted."""
    raise NotImplementedError("TODO: missing_vlans")
@@file LAB.md
# Lab: a small campus network in Packet Tracer

Build this in Cisco Packet Tracer (free from Cisco Networking Academy). Type every command
yourself; the reference configs in `configs/` are for checking afterwards, not copying.

```
            Internet (ISP router, 203.0.113.1)
                     |
                  Gi0/2  203.0.113.2/30
     +------------- R1 ----------------+
     | Gi0/0 (trunk: .10 .20 .99)      | Gi0/1 10.1.12.1/30
     |                                 |
    SW1 --- Gi0/24 trunk --- SW2       R2 (Loopback0 10.2.2.2/24 as the "branch" network)
     |                        |
   PC1 (VLAN 10)        PC2 (VLAN 10), IP phone (VLAN 20)
```

| VLAN | Name   | Subnet          | Gateway     |
|------|--------|-----------------|-------------|
| 10   | STAFF  | 10.10.10.0/24   | 10.10.10.1  |
| 20   | VOICE  | 10.10.20.0/24   | 10.10.20.1  |
| 99   | MGMT   | 10.10.99.0/24   | 10.10.99.1  |

## Tasks
1. Basic setup on every device: hostname, enable secret, banner, `no ip domain-lookup`, console `logging synchronous`.
2. SW1/SW2: VLANs 10, 20, 99 with names. Access ports in VLAN 10 with voice VLAN 20, portfast and BPDU guard.
3. Trunk between SW1 and SW2 and from SW1 to R1. Native VLAN 999, allowed VLANs 10,20,99 only.
4. Make SW1 the root bridge for all VLANs.
5. R1: router-on-a-stick sub-interfaces for VLANs 10, 20, 99.
6. R1: DHCP pools for VLANs 10 and 20 (exclude .1-.20). Check PCs get addresses.
7. R1-R2: OSPF area 0 on the /30 link and R2's loopback; passive-interface on the LAN sub-interfaces. PC1 must ping 10.2.2.2.
8. R1: a default route to the ISP and `default-information originate` in OSPF.
9. R1: PAT for 10.10.0.0/16 out of Gi0/2.
10. SSH on all devices (domain name, RSA 2048, local user, `transport input ssh`), with an ACL so only VLAN 99 can SSH in.
11. NTP from R1, timezone JST, syslog to 10.10.99.30.
12. Save every device's configuration.

## Check
- `show vlan brief`, `show interfaces trunk`, `show spanning-tree`
- `show ip interface brief`, `show ip route`, `show ip ospf neighbor` (FULL)
- `show ip dhcp binding`, `show ip nat translations`, `show access-lists`
- Copy the output of `show ip interface brief` and `show vlan brief` into files and run this
  project's checker on them: `python main.py r1.txt sw1.txt --vlans 10,20,99`

## Break and fix
- Shut R1's Gi0/1: what does the checker report? What happens to OSPF?
- Remove VLAN 20 from the trunk's allowed list: which symptoms do phones show?
- Set a different native VLAN on SW2's trunk end: what does CDP log?
@@file configs/R1.txt
! Reference configuration for R1 (check your own against it)
hostname R1
enable secret Lab!Secret1
no ip domain-lookup
ip domain-name lab.local
banner motd # Authorized access only #
clock timezone JST 9
!
ip dhcp excluded-address 10.10.10.1 10.10.10.20
ip dhcp excluded-address 10.10.20.1 10.10.20.20
ip dhcp pool STAFF
 network 10.10.10.0 255.255.255.0
 default-router 10.10.10.1
 dns-server 8.8.8.8
ip dhcp pool VOICE
 network 10.10.20.0 255.255.255.0
 default-router 10.10.20.1
!
username admin secret Lab!Admin1
crypto key generate rsa modulus 2048
ip ssh version 2
!
interface GigabitEthernet0/0
 no shutdown
interface GigabitEthernet0/0.10
 encapsulation dot1Q 10
 ip address 10.10.10.1 255.255.255.0
 ip nat inside
interface GigabitEthernet0/0.20
 encapsulation dot1Q 20
 ip address 10.10.20.1 255.255.255.0
 ip nat inside
interface GigabitEthernet0/0.99
 encapsulation dot1Q 99
 ip address 10.10.99.1 255.255.255.0
interface GigabitEthernet0/0.999
 encapsulation dot1Q 999 native
interface GigabitEthernet0/1
 ip address 10.1.12.1 255.255.255.252
 no shutdown
interface GigabitEthernet0/2
 ip address 203.0.113.2 255.255.255.252
 ip nat outside
 no shutdown
interface Loopback0
 ip address 1.1.1.1 255.255.255.255
!
router ospf 1
 router-id 1.1.1.1
 network 10.10.0.0 0.0.255.255 area 0
 network 10.1.12.0 0.0.0.3 area 0
 passive-interface GigabitEthernet0/0.10
 passive-interface GigabitEthernet0/0.20
 passive-interface GigabitEthernet0/0.99
 default-information originate
!
ip route 0.0.0.0 0.0.0.0 203.0.113.1
access-list 1 permit 10.10.0.0 0.0.255.255
ip nat inside source list 1 interface GigabitEthernet0/2 overload
!
ip access-list standard MGMT
 permit 10.10.99.0 0.0.0.255
line console 0
 logging synchronous
line vty 0 15
 login local
 transport input ssh
 access-class MGMT in
!
ntp master 3
logging host 10.10.99.30
end
@@file configs/SW1.txt
! Reference configuration for SW1 (check your own against it)
hostname SW1
enable secret Lab!Secret1
no ip domain-lookup
ip domain-name lab.local
banner motd # Authorized access only #
!
vlan 10
 name STAFF
vlan 20
 name VOICE
vlan 99
 name MGMT
vlan 999
 name NATIVE-UNUSED
!
spanning-tree mode rapid-pvst
spanning-tree vlan 10,20,99 root primary
!
interface range GigabitEthernet0/1 - 8
 switchport mode access
 switchport access vlan 10
 switchport voice vlan 20
 spanning-tree portfast
 spanning-tree bpduguard enable
!
interface GigabitEthernet0/23
 description to R1
 switchport mode trunk
 switchport trunk native vlan 999
 switchport trunk allowed vlan 10,20,99,999
 switchport nonegotiate
interface GigabitEthernet0/24
 description to SW2
 switchport mode trunk
 switchport trunk native vlan 999
 switchport trunk allowed vlan 10,20,99
 switchport nonegotiate
!
interface Vlan99
 ip address 10.10.99.2 255.255.255.0
 no shutdown
ip default-gateway 10.10.99.1
!
username admin secret Lab!Admin1
crypto key generate rsa modulus 2048
ip ssh version 2
line vty 0 15
 login local
 transport input ssh
!
ntp server 10.10.99.1
logging host 10.10.99.30
end
@@file samples/r1_ip_int_brief.txt
R1#show ip interface brief
Interface              IP-Address      OK? Method Status                Protocol
GigabitEthernet0/0     unassigned      YES unset  up                    up
GigabitEthernet0/0.10  10.10.10.1      YES manual up                    up
GigabitEthernet0/0.20  10.10.20.1      YES manual up                    up
GigabitEthernet0/0.99  10.10.99.1      YES manual up                    up
GigabitEthernet0/1     10.1.12.1       YES manual up                    down
GigabitEthernet0/2     203.0.113.2     YES manual administratively down down
GigabitEthernet0/3     unassigned      YES unset  administratively down down
Loopback0              1.1.1.1         YES manual up                    up
R1#
@@file samples/sw1_vlan_brief.txt
SW1#show vlan brief

VLAN Name                             Status    Ports
---- -------------------------------- --------- -------------------------------
1    default                          active    Gi0/23, Gi0/24
10   STAFF                            active    Gi0/1, Gi0/2, Gi0/3, Gi0/4
                                                Gi0/5, Gi0/6, Gi0/7, Gi0/8
20   VOICE                            active    Gi0/1, Gi0/2, Gi0/3, Gi0/4
30   PRINTERS                         act/lshut Gi0/9
99   MGMT                             active
1002 fddi-default                     act/unsup
1003 token-ring-default               act/unsup
1004 fddinet-default                  act/unsup
1005 trnet-default                    act/unsup
SW1#
@@file test_showparse.py
import unittest
from pathlib import Path

from showparse import interface_problems, missing_vlans, parse_ip_int_brief, parse_vlan_brief

SAMPLES = Path(__file__).parent / "samples"


class IpIntBriefTests(unittest.TestCase):
    def setUp(self):
        self.rows = parse_ip_int_brief((SAMPLES / "r1_ip_int_brief.txt").read_text())

    def test_every_interface_and_no_header_or_prompt(self):
        self.assertEqual(len(self.rows), 8)
        self.assertEqual(self.rows[0]["interface"], "GigabitEthernet0/0")
        self.assertEqual(self.rows[-1]["interface"], "Loopback0")

    def test_fields(self):
        self.assertEqual(self.rows[1], {"interface": "GigabitEthernet0/0.10", "ip": "10.10.10.1", "status": "up", "protocol": "up"})

    def test_unassigned_is_none(self):
        self.assertIsNone(self.rows[0]["ip"])

    def test_two_word_status(self):
        self.assertEqual(self.rows[5]["status"], "administratively down")
        self.assertEqual(self.rows[5]["protocol"], "down")

    def test_problems(self):
        self.assertEqual(interface_problems(self.rows), [
            "GigabitEthernet0/1 (10.1.12.1): up/down, check layer 2 (encapsulation, keepalives, the other end)",
            "GigabitEthernet0/2 (203.0.113.2): shut down",
        ])

    def test_down_down(self):
        rows = parse_ip_int_brief("Serial0/0/0  10.9.9.1  YES manual down  down")
        self.assertEqual(interface_problems(rows), ["Serial0/0/0 (10.9.9.1): down/down, check the cable and the other device"])


class VlanBriefTests(unittest.TestCase):
    def setUp(self):
        self.vlans = parse_vlan_brief((SAMPLES / "sw1_vlan_brief.txt").read_text())

    def test_ids(self):
        self.assertEqual(sorted(self.vlans), [1, 10, 20, 30, 99, 1002, 1003, 1004, 1005])

    def test_continuation_lines(self):
        self.assertEqual(self.vlans[10]["ports"], [f"Gi0/{n}" for n in range(1, 9)])
        self.assertEqual(self.vlans[10]["name"], "STAFF")

    def test_no_ports(self):
        self.assertEqual(self.vlans[99], {"name": "MGMT", "status": "active", "ports": []})
        self.assertEqual(self.vlans[1005]["ports"], [], "the SW1# prompt is not a port")

    def test_missing_vlans(self):
        self.assertEqual(missing_vlans(self.vlans, [10, 20, 30, 40, 99]), [30, 40])


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Check routers and switches from their "show" output.

python main.py samples/r1_ip_int_brief.txt samples/sw1_vlan_brief.txt --vlans 10,20,30,99
python main.py samples/r1_ip_int_brief.txt --json
python main.py --host 10.10.99.1 --user admin --vlans 10,20    (needs: pip install netmiko)
"""

import argparse
import getpass
import json
import sys
from pathlib import Path

from showparse import interface_problems, missing_vlans, parse_ip_int_brief, parse_vlan_brief


def from_device(host, user):
    """Run the two show commands over SSH with Netmiko."""
    from netmiko import ConnectHandler  # optional dependency

    conn = ConnectHandler(device_type="cisco_ios", host=host, username=user, password=getpass.getpass())
    try:
        return conn.send_command("show ip interface brief"), conn.send_command("show vlan brief")
    finally:
        conn.disconnect()


def main(argv=None):
    parser = argparse.ArgumentParser(description="Check Cisco show output")
    parser.add_argument("files", nargs="*", help="saved show ip interface brief / show vlan brief output")
    parser.add_argument("--host")
    parser.add_argument("--user", default="admin")
    parser.add_argument("--vlans", default="", help="VLAN ids that must exist and be active, e.g. 10,20,99")
    parser.add_argument("--json", action="store_true", help="print the parsed data as JSON")
    args = parser.parse_args(argv)

    texts = list(from_device(args.host, args.user)) if args.host else [Path(f).read_text() for f in args.files]
    interfaces, vlans = [], {}
    for text in texts:
        if "IP-Address" in text:
            interfaces += parse_ip_int_brief(text)
        elif "VLAN Name" in text:
            vlans.update(parse_vlan_brief(text))

    if args.json:
        print(json.dumps({"interfaces": interfaces, "vlans": vlans}, indent=2))
        return 0

    problems = interface_problems(interfaces)
    required = [int(v) for v in args.vlans.split(",") if v.strip()]
    if vlans or required:
        problems += [f"VLAN {v} is missing or not active" for v in missing_vlans(vlans, required)]
    print(f"{len(interfaces)} interfaces, {len(vlans)} VLANs checked")
    for p in problems:
        print("  x", p)
    print("OK" if not problems else f"{len(problems)} problem(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
@@text
> Tips: `line.split()` turns a row into words. For `show ip interface brief` the first four words are interface, IP, OK? and method, the **last** word is the protocol, and the status is everything in between: `" ".join(parts[4:-1])`. For `show vlan brief`, a line starting with a digit is a new VLAN (`line.split(None, 3)` keeps the port list in one piece), and an **indented** line continues the previous VLAN's ports.
@@project campus_lab_finished Campus Network Lab: the finished version
The complete checker. Ideas: parse `show interfaces trunk` and warn about native VLAN mismatches, check `show ip ospf neighbor` for neighbours that aren't FULL, or run the check against every device in an inventory file and write an HTML report.
@@file showparse.py
"""Turn Cisco IOS "show" output into structured data, and check it."""


def parse_ip_int_brief(text):
    """Parse `show ip interface brief` into [{"interface", "ip", "status", "protocol"}, ...].
    ip is None for "unassigned". Status may be two words ("administratively down")."""
    result = []
    for line in text.splitlines():
        parts = line.split()
        if len(parts) < 6 or parts[0] == "Interface":
            continue
        result.append({
            "interface": parts[0],
            "ip": None if parts[1] == "unassigned" else parts[1],
            "status": " ".join(parts[4:-1]),
            "protocol": parts[-1],
        })
    return result


def parse_vlan_brief(text):
    """Parse `show vlan brief` into {vlan_id: {"name", "status", "ports": [...]}}.
    Long port lists continue on the next line(s), indented."""
    vlans = {}
    last = None
    for line in text.splitlines():
        if not line.strip() or line.startswith(("VLAN", "----")):
            continue
        if line[0].isdigit():
            parts = line.split(None, 3)
            last = int(parts[0])
            vlans[last] = {"name": parts[1], "status": parts[2], "ports": []}
            ports = parts[3] if len(parts) > 3 else ""
        elif last is not None and line[0].isspace():
            ports = line
        else:
            continue
        vlans[last]["ports"] += [p.strip() for p in ports.split(",") if p.strip()]
    return vlans


def interface_problems(interfaces):
    """Messages for interfaces that have an IP address but aren't up/up:
      "Gi0/1 (10.1.1.1): shut down"                      status "administratively down"
      "Gi0/1 (10.1.1.1): down/down, check the cable and the other device"
      "Gi0/1 (10.1.1.1): up/down, check layer 2 (encapsulation, keepalives, the other end)"
    Interfaces without an IP are ignored (unused ports are often shut on purpose)."""
    problems = []
    for i in interfaces:
        if not i["ip"] or (i["status"] == "up" and i["protocol"] == "up"):
            continue
        name = f"{i['interface']} ({i['ip']})"
        if i["status"] == "administratively down":
            problems.append(f"{name}: shut down")
        elif i["status"] == "down":
            problems.append(f"{name}: down/down, check the cable and the other device")
        else:
            problems.append(f"{name}: up/down, check layer 2 (encapsulation, keepalives, the other end)")
    return problems


def missing_vlans(vlans, required):
    """The required VLAN ids that don't exist or aren't active, sorted."""
    return sorted(v for v in required if v not in vlans or vlans[v]["status"] != "active")
@@file LAB.md
# Lab: a small campus network in Packet Tracer

Build this in Cisco Packet Tracer (free from Cisco Networking Academy). Type every command
yourself; the reference configs in `configs/` are for checking afterwards, not copying.

```
            Internet (ISP router, 203.0.113.1)
                     |
                  Gi0/2  203.0.113.2/30
     +------------- R1 ----------------+
     | Gi0/0 (trunk: .10 .20 .99)      | Gi0/1 10.1.12.1/30
     |                                 |
    SW1 --- Gi0/24 trunk --- SW2       R2 (Loopback0 10.2.2.2/24 as the "branch" network)
     |                        |
   PC1 (VLAN 10)        PC2 (VLAN 10), IP phone (VLAN 20)
```

| VLAN | Name   | Subnet          | Gateway     |
|------|--------|-----------------|-------------|
| 10   | STAFF  | 10.10.10.0/24   | 10.10.10.1  |
| 20   | VOICE  | 10.10.20.0/24   | 10.10.20.1  |
| 99   | MGMT   | 10.10.99.0/24   | 10.10.99.1  |

## Tasks
1. Basic setup on every device: hostname, enable secret, banner, `no ip domain-lookup`, console `logging synchronous`.
2. SW1/SW2: VLANs 10, 20, 99 with names. Access ports in VLAN 10 with voice VLAN 20, portfast and BPDU guard.
3. Trunk between SW1 and SW2 and from SW1 to R1. Native VLAN 999, allowed VLANs 10,20,99 only.
4. Make SW1 the root bridge for all VLANs.
5. R1: router-on-a-stick sub-interfaces for VLANs 10, 20, 99.
6. R1: DHCP pools for VLANs 10 and 20 (exclude .1-.20). Check PCs get addresses.
7. R1-R2: OSPF area 0 on the /30 link and R2's loopback; passive-interface on the LAN sub-interfaces. PC1 must ping 10.2.2.2.
8. R1: a default route to the ISP and `default-information originate` in OSPF.
9. R1: PAT for 10.10.0.0/16 out of Gi0/2.
10. SSH on all devices (domain name, RSA 2048, local user, `transport input ssh`), with an ACL so only VLAN 99 can SSH in.
11. NTP from R1, timezone JST, syslog to 10.10.99.30.
12. Save every device's configuration.

## Check
- `show vlan brief`, `show interfaces trunk`, `show spanning-tree`
- `show ip interface brief`, `show ip route`, `show ip ospf neighbor` (FULL)
- `show ip dhcp binding`, `show ip nat translations`, `show access-lists`
- Copy the output of `show ip interface brief` and `show vlan brief` into files and run this
  project's checker on them: `python main.py r1.txt sw1.txt --vlans 10,20,99`

## Break and fix
- Shut R1's Gi0/1: what does the checker report? What happens to OSPF?
- Remove VLAN 20 from the trunk's allowed list: which symptoms do phones show?
- Set a different native VLAN on SW2's trunk end: what does CDP log?
@@file configs/R1.txt
! Reference configuration for R1 (check your own against it)
hostname R1
enable secret Lab!Secret1
no ip domain-lookup
ip domain-name lab.local
banner motd # Authorized access only #
clock timezone JST 9
!
ip dhcp excluded-address 10.10.10.1 10.10.10.20
ip dhcp excluded-address 10.10.20.1 10.10.20.20
ip dhcp pool STAFF
 network 10.10.10.0 255.255.255.0
 default-router 10.10.10.1
 dns-server 8.8.8.8
ip dhcp pool VOICE
 network 10.10.20.0 255.255.255.0
 default-router 10.10.20.1
!
username admin secret Lab!Admin1
crypto key generate rsa modulus 2048
ip ssh version 2
!
interface GigabitEthernet0/0
 no shutdown
interface GigabitEthernet0/0.10
 encapsulation dot1Q 10
 ip address 10.10.10.1 255.255.255.0
 ip nat inside
interface GigabitEthernet0/0.20
 encapsulation dot1Q 20
 ip address 10.10.20.1 255.255.255.0
 ip nat inside
interface GigabitEthernet0/0.99
 encapsulation dot1Q 99
 ip address 10.10.99.1 255.255.255.0
interface GigabitEthernet0/0.999
 encapsulation dot1Q 999 native
interface GigabitEthernet0/1
 ip address 10.1.12.1 255.255.255.252
 no shutdown
interface GigabitEthernet0/2
 ip address 203.0.113.2 255.255.255.252
 ip nat outside
 no shutdown
interface Loopback0
 ip address 1.1.1.1 255.255.255.255
!
router ospf 1
 router-id 1.1.1.1
 network 10.10.0.0 0.0.255.255 area 0
 network 10.1.12.0 0.0.0.3 area 0
 passive-interface GigabitEthernet0/0.10
 passive-interface GigabitEthernet0/0.20
 passive-interface GigabitEthernet0/0.99
 default-information originate
!
ip route 0.0.0.0 0.0.0.0 203.0.113.1
access-list 1 permit 10.10.0.0 0.0.255.255
ip nat inside source list 1 interface GigabitEthernet0/2 overload
!
ip access-list standard MGMT
 permit 10.10.99.0 0.0.0.255
line console 0
 logging synchronous
line vty 0 15
 login local
 transport input ssh
 access-class MGMT in
!
ntp master 3
logging host 10.10.99.30
end
@@file configs/SW1.txt
! Reference configuration for SW1 (check your own against it)
hostname SW1
enable secret Lab!Secret1
no ip domain-lookup
ip domain-name lab.local
banner motd # Authorized access only #
!
vlan 10
 name STAFF
vlan 20
 name VOICE
vlan 99
 name MGMT
vlan 999
 name NATIVE-UNUSED
!
spanning-tree mode rapid-pvst
spanning-tree vlan 10,20,99 root primary
!
interface range GigabitEthernet0/1 - 8
 switchport mode access
 switchport access vlan 10
 switchport voice vlan 20
 spanning-tree portfast
 spanning-tree bpduguard enable
!
interface GigabitEthernet0/23
 description to R1
 switchport mode trunk
 switchport trunk native vlan 999
 switchport trunk allowed vlan 10,20,99,999
 switchport nonegotiate
interface GigabitEthernet0/24
 description to SW2
 switchport mode trunk
 switchport trunk native vlan 999
 switchport trunk allowed vlan 10,20,99
 switchport nonegotiate
!
interface Vlan99
 ip address 10.10.99.2 255.255.255.0
 no shutdown
ip default-gateway 10.10.99.1
!
username admin secret Lab!Admin1
crypto key generate rsa modulus 2048
ip ssh version 2
line vty 0 15
 login local
 transport input ssh
!
ntp server 10.10.99.1
logging host 10.10.99.30
end
@@file samples/r1_ip_int_brief.txt
R1#show ip interface brief
Interface              IP-Address      OK? Method Status                Protocol
GigabitEthernet0/0     unassigned      YES unset  up                    up
GigabitEthernet0/0.10  10.10.10.1      YES manual up                    up
GigabitEthernet0/0.20  10.10.20.1      YES manual up                    up
GigabitEthernet0/0.99  10.10.99.1      YES manual up                    up
GigabitEthernet0/1     10.1.12.1       YES manual up                    down
GigabitEthernet0/2     203.0.113.2     YES manual administratively down down
GigabitEthernet0/3     unassigned      YES unset  administratively down down
Loopback0              1.1.1.1         YES manual up                    up
R1#
@@file samples/sw1_vlan_brief.txt
SW1#show vlan brief

VLAN Name                             Status    Ports
---- -------------------------------- --------- -------------------------------
1    default                          active    Gi0/23, Gi0/24
10   STAFF                            active    Gi0/1, Gi0/2, Gi0/3, Gi0/4
                                                Gi0/5, Gi0/6, Gi0/7, Gi0/8
20   VOICE                            active    Gi0/1, Gi0/2, Gi0/3, Gi0/4
30   PRINTERS                         act/lshut Gi0/9
99   MGMT                             active
1002 fddi-default                     act/unsup
1003 token-ring-default               act/unsup
1004 fddinet-default                  act/unsup
1005 trnet-default                    act/unsup
SW1#
@@file test_showparse.py
import unittest
from pathlib import Path

from showparse import interface_problems, missing_vlans, parse_ip_int_brief, parse_vlan_brief

SAMPLES = Path(__file__).parent / "samples"


class IpIntBriefTests(unittest.TestCase):
    def setUp(self):
        self.rows = parse_ip_int_brief((SAMPLES / "r1_ip_int_brief.txt").read_text())

    def test_every_interface_and_no_header_or_prompt(self):
        self.assertEqual(len(self.rows), 8)
        self.assertEqual(self.rows[0]["interface"], "GigabitEthernet0/0")
        self.assertEqual(self.rows[-1]["interface"], "Loopback0")

    def test_fields(self):
        self.assertEqual(self.rows[1], {"interface": "GigabitEthernet0/0.10", "ip": "10.10.10.1", "status": "up", "protocol": "up"})

    def test_unassigned_is_none(self):
        self.assertIsNone(self.rows[0]["ip"])

    def test_two_word_status(self):
        self.assertEqual(self.rows[5]["status"], "administratively down")
        self.assertEqual(self.rows[5]["protocol"], "down")

    def test_problems(self):
        self.assertEqual(interface_problems(self.rows), [
            "GigabitEthernet0/1 (10.1.12.1): up/down, check layer 2 (encapsulation, keepalives, the other end)",
            "GigabitEthernet0/2 (203.0.113.2): shut down",
        ])

    def test_down_down(self):
        rows = parse_ip_int_brief("Serial0/0/0  10.9.9.1  YES manual down  down")
        self.assertEqual(interface_problems(rows), ["Serial0/0/0 (10.9.9.1): down/down, check the cable and the other device"])


class VlanBriefTests(unittest.TestCase):
    def setUp(self):
        self.vlans = parse_vlan_brief((SAMPLES / "sw1_vlan_brief.txt").read_text())

    def test_ids(self):
        self.assertEqual(sorted(self.vlans), [1, 10, 20, 30, 99, 1002, 1003, 1004, 1005])

    def test_continuation_lines(self):
        self.assertEqual(self.vlans[10]["ports"], [f"Gi0/{n}" for n in range(1, 9)])
        self.assertEqual(self.vlans[10]["name"], "STAFF")

    def test_no_ports(self):
        self.assertEqual(self.vlans[99], {"name": "MGMT", "status": "active", "ports": []})
        self.assertEqual(self.vlans[1005]["ports"], [], "the SW1# prompt is not a port")

    def test_missing_vlans(self):
        self.assertEqual(missing_vlans(self.vlans, [10, 20, 30, 40, 99]), [30, 40])


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""Check routers and switches from their "show" output.

python main.py samples/r1_ip_int_brief.txt samples/sw1_vlan_brief.txt --vlans 10,20,30,99
python main.py samples/r1_ip_int_brief.txt --json
python main.py --host 10.10.99.1 --user admin --vlans 10,20    (needs: pip install netmiko)
"""

import argparse
import getpass
import json
import sys
from pathlib import Path

from showparse import interface_problems, missing_vlans, parse_ip_int_brief, parse_vlan_brief


def from_device(host, user):
    """Run the two show commands over SSH with Netmiko."""
    from netmiko import ConnectHandler  # optional dependency

    conn = ConnectHandler(device_type="cisco_ios", host=host, username=user, password=getpass.getpass())
    try:
        return conn.send_command("show ip interface brief"), conn.send_command("show vlan brief")
    finally:
        conn.disconnect()


def main(argv=None):
    parser = argparse.ArgumentParser(description="Check Cisco show output")
    parser.add_argument("files", nargs="*", help="saved show ip interface brief / show vlan brief output")
    parser.add_argument("--host")
    parser.add_argument("--user", default="admin")
    parser.add_argument("--vlans", default="", help="VLAN ids that must exist and be active, e.g. 10,20,99")
    parser.add_argument("--json", action="store_true", help="print the parsed data as JSON")
    args = parser.parse_args(argv)

    texts = list(from_device(args.host, args.user)) if args.host else [Path(f).read_text() for f in args.files]
    interfaces, vlans = [], {}
    for text in texts:
        if "IP-Address" in text:
            interfaces += parse_ip_int_brief(text)
        elif "VLAN Name" in text:
            vlans.update(parse_vlan_brief(text))

    if args.json:
        print(json.dumps({"interfaces": interfaces, "vlans": vlans}, indent=2))
        return 0

    problems = interface_problems(interfaces)
    required = [int(v) for v in args.vlans.split(",") if v.strip()]
    if vlans or required:
        problems += [f"VLAN {v} is missing or not active" for v in missing_vlans(vlans, required)]
    print(f"{len(interfaces)} interfaces, {len(vlans)} VLANs checked")
    for p in problems:
        print("  x", p)
    print("OK" if not problems else f"{len(problems)} problem(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())

@@drill
Which IOS command shows a summary of interface IP addresses and status?
= show ip interface brief
= sh ip int br
= show ip int brief
? The first command to run when checking a device.

@@text
🎉 You configured a campus network by hand *and* wrote the automation that checks it. That combination, CCNA knowledge plus Python, is exactly what "network automation engineer" job ads ask for. Put the lab (with your own configs) and the checker on GitHub.
