@@lesson xp-vlsm_planner VLSM Subnet Planner
@@text
Given a block like 10.20.0.0/22 and a list of departments with host counts, allocate the smallest subnet that fits each (biggest first, so nothing overlaps), and print a plan with network, mask, first and last usable host, broadcast and spare capacity. Exactly the subnetting the Network+ and CCNA exams test.

**You'll practise:** VLSM · binary and masks · Python ipaddress · allocation order

## Milestones

1. **Sizes**: for each department, the smallest prefix whose usable hosts ≥ needed hosts.
2. **Allocate**: sort largest first, hand out subnets in order from the block.
3. **Report**: a table with network, mask, usable range, broadcast and % used.
4. **Errors**: say clearly when the block is too small.
5. **Check by hand**: do two plans on paper first and compare with the program.

## You're done when

- The plan matches your hand calculations and never overlaps.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why allocate the biggest subnets first?"*
- *"Walk me through finding the broadcast address of 10.20.1.64/27 in binary."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project vlsm_planner VLSM Subnet Planner
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# VLSM Subnet Planner

Given a block like 10.20.0.0/22 and a list of departments with host counts, allocate the smallest subnet that fits each (biggest first, so nothing overlaps), and print a plan with network, mask, first and last usable host, broadcast and spare capacity. Exactly the subnetting the Network+ and CCNA exams test.

## Milestones

- [ ] 1. Sizes: for each department, the smallest prefix whose usable hosts ≥ needed hosts.
- [ ] 2. Allocate: sort largest first, hand out subnets in order from the block.
- [ ] 3. Report: a table with network, mask, usable range, broadcast and % used.
- [ ] 4. Errors: say clearly when the block is too small.
- [ ] 5. Check by hand: do two plans on paper first and compare with the program.

## Done when

- [ ] The plan matches your hand calculations and never overlaps.
@@file vlsm.py
"""VLSM planner: python vlsm.py 10.20.0.0/22 sales=120 dev=60 hr=25 mgmt=10 p2p=2"""
import ipaddress
import math
import sys


def prefix_for(hosts: int) -> int:
    """The longest prefix (smallest subnet) with at least `hosts` usable addresses."""
    # usable = 2 ** (32 - prefix) - 2
    return 32 - math.ceil(math.log2(hosts + 2))


def plan(block: str, needs: dict[str, int]) -> list[tuple[str, ipaddress.IPv4Network]]:
    """(department, subnet) pairs, biggest first."""
    # TODO
    return []


if __name__ == "__main__":
    needs = dict((k, int(v)) for k, v in (a.split("=") for a in sys.argv[2:]))
    for name, net in plan(sys.argv[1], needs):
        print(f"{name:<8} {net}")
