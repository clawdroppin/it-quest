@@lesson f9-home-lab Build Your Home Lab
@@text
Employers ask "what have you built?" more than "what have you read?". A **home lab** is where you practise everything in this app on real systems, break things without consequences, and collect stories for interviews. It doesn't need to cost anything: one PC with **16 GB of RAM** and a free hypervisor is enough to start.

## What you'll set up

| VM | Software (free) | You'll practise |
|---|---|---|
| **DC01** | Windows Server evaluation (180 days) | Active Directory, DNS, DHCP, Group Policy |
| **WS01** | Windows 11 Enterprise evaluation | joining a domain, help desk tasks, PowerShell |
| **LNX01** | Ubuntu Server LTS | Linux admin, SSH, a web server, cron, Docker |
| *(later)* **FW01** | pfSense or OPNsense | firewall rules, VLANs, VPN |

Use **Hyper-V** (built into Windows 10/11 Pro), **VirtualBox** or **VMware Workstation**. Put the VMs on an *internal* virtual network so your lab's DHCP server never fights your home router.

## Document like a professional

Keep a lab notebook (the project below gives you templates). Write down every IP, password location, and every "I broke it and here's how I fixed it". That notebook becomes a **portfolio**: put the clean version on GitHub with screenshots, and link it on your CV.

@@project home_lab Home Lab Notebook
Create the folder, open it in VS Code, and fill in the templates as you build. Tick off the checklist one line at a time; each line is a skill you can put on your CV.
@@file README.md
# My IT Home Lab

A small virtual company network for practising Windows Server, Linux, networking and security.

## Hardware
- Host: (CPU, RAM, disk)
- Hypervisor: (Hyper-V / VirtualBox / VMware Workstation)

## Network
| Name  | Role                   | OS                  | IP           |
|-------|------------------------|---------------------|--------------|
| DC01  | Domain controller, DNS, DHCP | Windows Server | 10.10.0.10 |
| WS01  | Client                 | Windows 11          | DHCP         |
| LNX01 | Web / Docker host      | Ubuntu Server LTS   | 10.10.0.20   |

Lab network: 10.10.0.0/24, gateway 10.10.0.1, domain `lab.local`

## What I've done
See [checklist.md](checklist.md) and the notes in [log.md](log.md).
@@file checklist.md
# Lab checklist

## Week 1: the basics
- [ ] Install the hypervisor and create an internal network
- [ ] Install Windows Server, set a static IP, rename to DC01
- [ ] Install Ubuntu Server, enable SSH, log in from your PC with an SSH key
- [ ] Take a snapshot of each VM ("clean install")

## Week 2: Active Directory
- [ ] Promote DC01 to a domain controller for lab.local
- [ ] Configure DHCP on DC01 for 10.10.0.100-200
- [ ] Join WS01 to the domain
- [ ] Create OUs (Staff, IT, Computers), users and groups, with PowerShell
- [ ] A Group Policy that maps a drive and sets a wallpaper

## Week 3: help desk scenarios
- [ ] Reset a password and unlock an account (GUI and PowerShell)
- [ ] Share a folder with NTFS permissions for one group only
- [ ] Break DNS on WS01 on purpose, troubleshoot it with ipconfig/nslookup, fix it
- [ ] Restore a VM from a snapshot

## Week 4: Linux and services
- [ ] Install nginx on LNX01 and serve a page
- [ ] Set up a firewall (ufw) allowing only SSH and HTTP
- [ ] Schedule a backup script with cron
- [ ] Run a container with Docker

## Next
- [ ] Firewall VM with VLANs
- [ ] Central logging (send logs to one place)
- [ ] Put the cleaned-up notes on GitHub
@@file log.md
# Lab log

Write an entry every time you work on the lab, especially when something breaks.

## YYYY-MM-DD
**Goal:**
**What I did:**
**What went wrong:**
**How I fixed it:**
**What I learned:**

@@drill
What's the name of software that runs virtual machines on a computer?
= hypervisor
= a hypervisor
? VirtualBox, Hyper-V and Proxmox are examples.

@@text
💡 In interviews, a story from this log ("DNS broke after I changed the DC's IP, and here's how I found it") is worth more than a certificate on its own. Do both.
