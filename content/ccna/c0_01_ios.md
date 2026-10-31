@@lesson c0-ios CCNA and the Cisco IOS Command Line
@@text
**Cisco CCNA** (exam **200-301**) is the most respected entry-level networking certification. Cisco gear runs a huge share of company and carrier networks, including in Japan, where CCNA/CCNP holders are in steady demand at SIers and network integrators. Do **Network+** (or at least its subnetting lesson) first.

| Domain | Weight |
|---|---|
| Network fundamentals | 20% |
| Network access (switching, VLANs, wireless) | 20% |
| IP connectivity (routing) | 25% |
| IP services (NAT, DHCP, NTP, QoS...) | 10% |
| Security fundamentals | 15% |
| Automation and programmability | 10% |

120 minutes, around 100 questions, with **simulations** where you type real commands. Practise in **Cisco Packet Tracer** (free with a Cisco Networking Academy account), GNS3, or Cisco's CML.

## IOS modes

```text
Switch>                         user EXEC: look only
Switch> enable
Switch#                         privileged EXEC: show, debug, copy, reload
Switch# configure terminal
Switch(config)#                 global configuration
Switch(config)# interface g0/1
Switch(config-if)#              interface configuration
Switch(config-if)# end          back to privileged EXEC (exit goes up one level)
```

`?` shows what you can type next; **Tab** completes; commands can be shortened (`conf t`, `sh ip int br`). `do show ...` runs a show command from config mode. `no <command>` removes a setting.

## Basic device setup

```text
hostname SW1
enable secret Str0ng!Secret            ! hashed; never use "enable password"
service password-encryption            ! weakly hides other passwords in the config
banner motd # Authorized access only #
no ip domain-lookup                    ! typos don't hang trying DNS
line console 0
 logging synchronous
 exec-timeout 10 0
!
interface vlan 1                       ! a switch's management IP lives on an SVI
 ip address 192.168.1.2 255.255.255.0
 no shutdown
ip default-gateway 192.168.1.1
```

## Saving and showing

| Command | Shows / does |
|---|---|
| `show running-config` | the active config in RAM |
| `show startup-config` | the config saved in NVRAM, used at boot |
| `copy running-config startup-config` (or `write memory`) | **save**: forget it and a reboot loses your work |
| `show ip interface brief` | every interface, its IP and up/down status |
| `show interfaces g0/1` | details, errors, duplex, speed |
| `show version` | IOS version, uptime, config register |
| `show cdp neighbors` / `show lldp neighbors` | directly connected devices |
| `show mac address-table` | switch MAC table |
| `show ip route` | router's routing table |

Interface status: **up/up** fine; **down/down** cable or other side; **administratively down** = `shutdown` configured; **up/down** = a layer 2 problem (encapsulation, keepalives).

@@quiz
You configured a router, then it rebooted after a power cut and all your changes are gone. What was missed?
* copy running-config startup-config
- enable secret
- no shutdown
- show running-config
? The running config lives in RAM; only the startup config in NVRAM survives a reboot.

@@quiz
Which prompt means you're in global configuration mode?
* Router(config)#
- Router#
- Router>
- Router(config-if)#
? Get there with `configure terminal` from privileged EXEC.

@@quiz
`show ip interface brief` shows an interface as "administratively down". What's the fix?
* Enter the interface and type no shutdown
- Replace the cable
- Change the duplex
- Save the configuration
? Router interfaces are shut down by default.

@@quiz
Why use `enable secret` instead of `enable password`?
* enable secret is stored as a strong hash; enable password is plain text (or weakly encoded)
- enable password is longer
- enable secret works only on switches
- They are identical
? If both are set, enable secret takes precedence.

@@quiz
How does a layer 2 switch get a management IP address?
* On an SVI such as interface vlan 1, with ip default-gateway for remote access
- On a physical access port
- It can't have an IP
- Through ip route 0.0.0.0
? Layer 2 switches don't route, so they also need a default gateway.

@@flashcards
user EXEC (>) :: View only
privileged EXEC (#) :: enable: show, copy, reload
global config :: configure terminal
running-config :: Active config in RAM
startup-config :: Saved config in NVRAM
copy run start / write memory :: Save the config
show ip interface brief :: Interface IPs and status summary
enable secret :: Hashed privileged password
no shutdown :: Enable an interface
administratively down :: Interface has "shutdown" configured
do :: Run EXEC commands from config mode
show cdp neighbors :: Directly connected Cisco devices
ip default-gateway :: Gateway for a layer 2 switch's management

@@drill
What's the short command to save the running configuration? (the classic one, starting with "copy")
= copy run start
= copy running-config startup-config
= copy run startup-config
? `write memory` (wr) also works.
