@@lesson xp-config_diff Router Config Backup and Diff
@@text
Most outages start with a change. Save `show running-config` output from your lab routers (Packet Tracer, GNS3, CML or real gear) into dated files, and write a tool that compares two backups section by section: which interfaces, ACLs, routing statements and VLANs were added, removed or changed.

**You'll practise:** IOS configuration structure · parent/child sections · difflib · change management

## Milestones

1. **Collect**: save two configs of the same device before and after a lab change.
2. **Parse**: group lines into sections (a line without indentation starts a new one).
3. **Compare**: added, removed and changed sections, ignoring timestamps and `!` lines.
4. **Report**: a readable summary, then the line-level diff of changed sections.
5. **Process**: write a CHANGE.md template (what, why, rollback plan) like a real change request.

## You're done when

- Comparing the two sample configs reports exactly the three changes made.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Which lines in a running-config change on their own and should be ignored?"*
- *"How do I group IOS config lines into sections?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project config_diff Router Config Backup and Diff
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Router Config Backup and Diff

Most outages start with a change. Save `show running-config` output from your lab routers (Packet Tracer, GNS3, CML or real gear) into dated files, and write a tool that compares two backups section by section: which interfaces, ACLs, routing statements and VLANs were added, removed or changed.

## Milestones

- [ ] 1. Collect: save two configs of the same device before and after a lab change.
- [ ] 2. Parse: group lines into sections (a line without indentation starts a new one).
- [ ] 3. Compare: added, removed and changed sections, ignoring timestamps and `!` lines.
- [ ] 4. Report: a readable summary, then the line-level diff of changed sections.
- [ ] 5. Process: write a CHANGE.md template (what, why, rollback plan) like a real change request.

## Done when

- [ ] Comparing the two sample configs reports exactly the three changes made.
@@file config_diff.py
"""Compare two IOS config backups: python config_diff.py before.cfg after.cfg"""
import sys


def sections(text: str) -> dict[str, list[str]]:
    """Top-level line → its indented child lines."""
    out: dict[str, list[str]] = {}
    current = None
    for line in text.splitlines():
        if not line.strip() or line.startswith("!"):
            continue
        if not line.startswith(" "):
            current = line.strip()
            out[current] = []
        elif current:
            out[current].append(line.strip())
    return out


if __name__ == "__main__":
    a = sections(open(sys.argv[1], encoding="utf-8").read())
    b = sections(open(sys.argv[2], encoding="utf-8").read())
    # TODO: added, removed, changed
    print(len(a), "sections before,", len(b), "after")
@@file before.cfg
hostname R1
!
interface GigabitEthernet0/0
 ip address 10.0.0.1 255.255.255.0
 no shutdown
!
interface GigabitEthernet0/1
 shutdown
!
router ospf 1
 network 10.0.0.0 0.0.0.255 area 0
!
line vty 0 4
 transport input ssh
@@file after.cfg
hostname R1
!
interface GigabitEthernet0/0
 ip address 10.0.0.1 255.255.255.0
 ip access-group MGMT in
 no shutdown
!
interface GigabitEthernet0/1
 ip address 10.1.0.1 255.255.255.0
 no shutdown
!
router ospf 1
 network 10.0.0.0 0.0.0.255 area 0
 network 10.1.0.0 0.0.0.255 area 0
!
ip access-list standard MGMT
 permit 10.0.0.0 0.0.0.255
!
line vty 0 4
 transport input ssh
