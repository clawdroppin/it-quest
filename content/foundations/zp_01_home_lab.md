@@lesson xp-home_lab Home Lab Inventory and Network Map
@@text
Treat your home as a tiny company: record every device (PC, phone, router, TV), its IP and MAC address, OS and owner, draw the network, and write a script that collects your own PC's details automatically. Documentation is the habit that separates good IT staff from the rest.

**You'll practise:** IP and MAC addresses · DHCP and the router · documentation · Python platform and socket modules

## Milestones

1. **Collect**: find each device's IP and MAC (router admin page, `ipconfig /all`, phone settings).
2. **Record**: devices.csv with name, type, IP, MAC, OS, owner, notes.
3. **Script**: collect.py prints this PC's hostname, OS, CPU, RAM and IP addresses.
4. **Map**: draw the network (router, Wi-Fi, devices) in NETWORK.md with a simple text diagram.
5. **Review**: which devices haven't had updates in a year? Note them.

## You're done when

- Someone else could understand your home network from your files alone.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I find which device has an unknown IP on my network?"*
- *"What's the difference between a private and a public IP address?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project home_lab Home Lab Inventory and Network Map
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Home Lab Inventory and Network Map

Treat your home as a tiny company: record every device (PC, phone, router, TV), its IP and MAC address, OS and owner, draw the network, and write a script that collects your own PC's details automatically. Documentation is the habit that separates good IT staff from the rest.

## Milestones

- [ ] 1. Collect: find each device's IP and MAC (router admin page, `ipconfig /all`, phone settings).
- [ ] 2. Record: devices.csv with name, type, IP, MAC, OS, owner, notes.
- [ ] 3. Script: collect.py prints this PC's hostname, OS, CPU, RAM and IP addresses.
- [ ] 4. Map: draw the network (router, Wi-Fi, devices) in NETWORK.md with a simple text diagram.
- [ ] 5. Review: which devices haven't had updates in a year? Note them.

## Done when

- [ ] Someone else could understand your home network from your files alone.
@@file collect.py
"""Print this computer's basic details for the inventory."""
import platform
import socket

print("hostname:", socket.gethostname())
print("os:", platform.system(), platform.release())
print("cpu:", platform.processor() or platform.machine())
# TODO: RAM (hint: on Windows use the wmic or PowerShell output; on Linux read /proc/meminfo), IP addresses
@@file devices.csv
name,type,ip,mac,os,owner,notes
router,router,192.168.1.1,,,family,ISP-provided
