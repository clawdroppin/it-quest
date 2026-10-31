@@lesson c2-services IP Services and Security on Cisco Devices
@@text
## DHCP

```text
ip dhcp excluded-address 10.10.10.1 10.10.10.20
ip dhcp pool STAFF
 network 10.10.10.0 255.255.255.0
 default-router 10.10.10.1
 dns-server 10.10.99.10
 lease 1
! relay on the gateway of another subnet:
interface g0/1
 ip helper-address 10.10.99.10
```

`show ip dhcp binding`, `show ip dhcp pool`. Security: **DHCP snooping** (`ip dhcp snooping`, `ip dhcp snooping vlan 10`, `ip dhcp snooping trust` on the uplink) blocks rogue DHCP servers; **Dynamic ARP Inspection** (`ip arp inspection vlan 10`) uses that table to block ARP spoofing.

## NAT and PAT

```text
interface g0/0
 ip nat inside
interface g0/1
 ip nat outside
access-list 1 permit 10.10.0.0 0.0.255.255
ip nat inside source list 1 interface g0/1 overload      ! PAT: everyone shares the outside IP
ip nat inside source static 10.10.99.20 203.0.113.20     ! static NAT for a server
```

Terms: **inside local** (the private address), **inside global** (the public address it's translated to), **outside global** (the internet host). `show ip nat translations`.

## NTP, syslog, SNMP, SSH

```text
ntp server 10.10.99.1
clock timezone JST 9
logging host 10.10.99.30
logging trap informational             ! severities 0 emergency ... 7 debugging
snmp-server group MONITOR v3 priv
!
ip domain-name lab.local
crypto key generate rsa modulus 2048
ip ssh version 2
username admin secret Str0ng!Pass
line vty 0 15
 login local
 transport input ssh                   ! no Telnet
 access-class MGMT in                  ! only from management hosts
```

Syslog severity: **0 Emergency, 1 Alert, 2 Critical, 3 Error, 4 Warning, 5 Notice, 6 Informational, 7 Debugging** ("Every Awesome Cisco Engineer Will Need Ice cream Daily").

## Access control lists

- **Standard** ACLs (1–99, or named) match the **source** only: place them **close to the destination**.
- **Extended** ACLs (100–199, or named) match protocol, source, destination and port: place them **close to the source**.
- Read top-down, first match wins, **implicit deny any** at the end. One ACL per interface, per direction, per protocol.

```text
ip access-list extended WEB-ONLY
 permit tcp 10.10.10.0 0.0.0.255 host 10.10.99.20 eq 443
 permit icmp 10.10.10.0 0.0.0.255 any echo
 deny ip any any log
interface g0/0.10
 ip access-group WEB-ONLY in
```

`show access-lists` shows hit counters per line.

## Other services and security

- **QoS**: classification and marking (**DSCP** EF for voice), queuing, policing vs shaping.
- **AAA** with **RADIUS** (UDP, encrypts only the password, common for network access) or **TACACS+** (TCP 49, encrypts the whole payload, separates authorization: preferred for device admin).
- **IPsec** site-to-site VPNs; **802.1X** port authentication; **WPA3**.

@@quiz
A standard ACL should stop network 10.1.5.0/24 from reaching one server. Where should it be applied?
* As close to the destination (the server) as possible
- As close to the source as possible
- On every interface
- On the internet router only
? Standard ACLs only see the source; near the source they'd block that network from everything.

@@quiz
An ACL contains only `permit tcp any host 10.1.1.10 eq 80`. Which traffic passes?
* Only HTTP to 10.1.1.10; everything else hits the implicit deny
- Everything
- Everything except HTTP
- Only traffic from 10.1.1.10
? Remember the invisible deny any at the end.

@@quiz
Which NAT command lets all inside hosts share the outside interface's single public IP?
* ip nat inside source list 1 interface g0/1 overload
- ip nat inside source static 10.1.1.10 203.0.113.10
- ip nat outside
- ip helper-address
? overload = PAT (port address translation).

@@quiz
Which protects against rogue DHCP servers on access ports?
* DHCP snooping, with the uplink port trusted
- Port security sticky
- portfast
- NTP authentication
? Untrusted ports may not send DHCP offers.

@@quiz
Which line configuration allows only SSH for remote management?
* transport input ssh
- login local only
- transport input telnet ssh
- exec-timeout 0
? Also needs a domain name, RSA keys and local users (or AAA).

@@quiz
Which AAA protocol encrypts the entire payload and is preferred for device administration?
* TACACS+
- RADIUS
- SNMPv2c
- Syslog
? RADIUS encrypts only the password.

@@quiz
What syslog severity level is "Warning"?
* 4
- 2
- 6
- 7
? 0 Emergency, 1 Alert, 2 Critical, 3 Error, 4 Warning, 5 Notice, 6 Informational, 7 Debugging.

@@flashcards
ip dhcp excluded-address :: Keep addresses out of a DHCP pool
ip helper-address :: DHCP relay on an interface
DHCP snooping :: Blocks rogue DHCP; trust uplinks
Dynamic ARP Inspection :: Blocks ARP spoofing using the snooping table
ip nat inside / outside :: Mark NAT interfaces
overload :: PAT: many hosts share one IP
inside local / inside global :: Private address / its public translation
standard ACL :: Source only; place near the destination
extended ACL :: Protocol, source, destination, ports; place near the source
implicit deny :: End of every ACL
ip access-group NAME in :: Apply an ACL to an interface
transport input ssh :: Only SSH on vty lines
crypto key generate rsa :: Create keys for SSH
syslog levels :: 0 Emergency ... 4 Warning ... 7 Debugging
RADIUS vs TACACS+ :: UDP, password-only encryption vs TCP 49, full encryption
DSCP EF :: QoS marking for voice

@@drill
Which TCP port does TACACS+ use?
= 49
= tcp 49
? RADIUS uses UDP 1812/1813.
