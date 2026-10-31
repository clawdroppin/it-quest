@@lesson c1-switching Configuring Switches: VLANs, Trunks, STP, EtherChannel
@@text
## VLANs and access ports

```text
vlan 10
 name STAFF
vlan 20
 name VOICE
!
interface range g0/1 - 24
 switchport mode access
 switchport access vlan 10
 switchport voice vlan 20              ! IP phone + PC on one port
 spanning-tree portfast                ! end devices: skip the STP wait
 spanning-tree bpduguard enable        ! shut the port if a switch is plugged in
```

Check with `show vlan brief`, `show interfaces switchport`.

## Trunks

```text
interface g0/48
 switchport trunk encapsulation dot1q  ! only needed on switches that also support ISL
 switchport mode trunk
 switchport trunk native vlan 999      ! unused VLAN, not 1
 switchport trunk allowed vlan 10,20,30,99
 switchport nonegotiate                ! no DTP
```

`show interfaces trunk` shows trunk ports, the native VLAN and which VLANs are allowed and active. A **native VLAN mismatch** at the two ends causes CDP warnings and traffic leaking between VLANs. **DTP** modes: `dynamic auto` + `dynamic auto` = no trunk; `dynamic desirable` with auto or desirable = trunk. Best practice: set `mode trunk` or `mode access` explicitly.

## Spanning Tree

Cisco runs **PVST+** (one STP per VLAN) or **Rapid PVST+** (`spanning-tree mode rapid-pvst`). The **root bridge** is the switch with the lowest **bridge ID** (priority, default 32768 + VLAN number, then lowest MAC). Make your core switch root on purpose:

```text
spanning-tree vlan 10,20 root primary      ! or: spanning-tree vlan 10 priority 4096
```

Port roles: **root port** (best path to the root, one per non-root switch), **designated** (forwarding on each segment), **alternate/backup** (blocking). `show spanning-tree vlan 10`.

## EtherChannel

```text
interface range g0/47 - 48
 channel-group 1 mode active           ! LACP; "passive" on the other side also works
! (mode desirable/auto = Cisco's PAgP; mode on = no negotiation)
interface port-channel 1
 switchport mode trunk
```

Member ports must match: speed, duplex, VLANs, trunk settings. `show etherchannel summary` (look for `SU` and `P` flags).

## Port security and neighbours

```text
interface g0/5
 switchport port-security
 switchport port-security maximum 2
 switchport port-security mac-address sticky
 switchport port-security violation restrict   ! shutdown (default) | restrict | protect
```

A port in **err-disabled** state (after a violation or BPDU guard) comes back with `shutdown` then `no shutdown` (or errdisable recovery). **CDP** (Cisco) and **LLDP** (standard; `lldp run`) discover neighbours; turn them off on ports facing untrusted devices.

@@quiz
Two switches are connected, but VLAN 20 traffic doesn't pass between them, while VLAN 10 does. `show interfaces trunk` shows "10,30" as allowed. What's the fix?
* Add VLAN 20 to the allowed list: switchport trunk allowed vlan add 20
- Change the native VLAN to 20
- Enable portfast on the trunk
- Shut down VLAN 10
? Careful: without "add", the command replaces the whole list.

@@quiz
Which switch becomes the STP root bridge?
* The one with the lowest bridge ID (priority, then MAC address)
- The one with the highest priority number
- The newest switch
- The one with the most ports
? Lower is better. Set the priority deliberately on your core switch.

@@quiz
A user plugged a small switch into their desk port and the port went err-disabled. Which feature did that?
* BPDU guard
- Portfast
- DTP
- CDP
? BPDU guard shuts a portfast port that receives BPDUs (switches send them).

@@quiz
Which EtherChannel mode actively negotiates with LACP?
* active
- desirable
- auto
- on
? desirable/auto are PAgP; on has no negotiation at all.

@@quiz
Both ends of a link are set to switchport mode dynamic auto. What happens?
* The link stays an access port: neither side initiates a trunk
- It becomes a trunk
- The ports err-disable
- STP blocks it
? At least one side must be desirable (or trunk).

@@quiz
Which port security violation mode drops offending traffic and logs it, but keeps the port up?
* restrict
- shutdown
- protect
- sticky
? protect drops without logging; shutdown err-disables the port.

@@flashcards
switchport mode access / trunk :: Set the port type explicitly
switchport access vlan 10 :: Put an access port in VLAN 10
switchport voice vlan :: Separate VLAN for an IP phone
spanning-tree portfast :: Skip listening/learning on edge ports
bpduguard :: err-disable an edge port that receives BPDUs
allowed vlan add :: Add VLANs to a trunk without replacing the list
native VLAN mismatch :: Different untagged VLANs on the trunk ends
DTP :: Dynamic Trunking Protocol; disable with nonegotiate
PVST+ / Rapid PVST+ :: Cisco per-VLAN STP / RSTP
root bridge :: Lowest bridge ID
root port :: Best path to the root on a non-root switch
show interfaces trunk :: Trunks, native and allowed VLANs
channel-group mode active :: EtherChannel with LACP
port-security sticky :: Learn and save the connected MAC
err-disabled :: Port shut by a protection feature

@@drill
Which command shows VLANs and their access ports in a short table?
= show vlan brief
= sh vlan br
= sh vlan brief
? `show interfaces trunk` covers trunks.
