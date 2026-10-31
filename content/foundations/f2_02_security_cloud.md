@@lesson f2-security-cloud Security, Virtualization and the Cloud in a Nutshell
@@text
## Security basics everyone in IT needs

The **CIA triad** is what security protects:

- **Confidentiality**: only the right people can read it (encryption, permissions).
- **Integrity**: it hasn't been changed by someone who shouldn't (hashes, signatures, change control).
- **Availability**: it's there when needed (backups, redundancy, protection against DoS).

**Authentication** proves *who* you are; **authorization** decides *what* you may do. **MFA** (multi-factor authentication) combines factors of different kinds: something you **know** (password), **have** (phone app, security key), or **are** (fingerprint). MFA stops the vast majority of account takeovers, which is why every company is rolling it out.

The most common attacks you'll see at a help desk:

- **Phishing**: a fake email or text that steals passwords or installs malware. Check the real sender address and hover links before clicking.
- **Ransomware**: encrypts files and demands payment. Defence: patches, backups kept **offline or immutable**, least privilege, EDR.
- **Social engineering**: "Hi, it's IT, I need your MFA code." Real IT never asks for passwords or codes. *You* must verify callers too before resetting passwords.

## Virtualization

A **virtual machine** (VM) is a computer simulated in software. A **hypervisor** runs many VMs on one physical server:

- **Type 1** ("bare metal") runs directly on the hardware: VMware ESXi, Microsoft Hyper-V, Proxmox, KVM. Used in data centres and the cloud.
- **Type 2** runs as an app on your desktop OS: VirtualBox, VMware Workstation. Perfect for a home lab.

**Containers** (Docker) are lighter: they share the host's kernel and package just an app and its libraries. They start in a second, and they're how modern apps are shipped.

## The cloud

"The cloud" is someone else's data centre, rented by the minute, run by software. The big three: **AWS**, **Microsoft Azure**, **Google Cloud**.

| Model | You manage | Provider manages | Example |
|---|---|---|---|
| **IaaS** | OS, apps, data | hardware, network, virtualization | an EC2 / Azure VM |
| **PaaS** | your app and data | everything below it | Azure App Service, AWS Elastic Beanstalk, managed databases |
| **SaaS** | just your data and users | everything | Microsoft 365, Gmail, Salesforce |

This split is the **shared responsibility model**: the provider secures *the cloud*; you secure what you put *in* it. A public storage bucket full of customer data is the customer's fault, not Amazon's.

@@quiz
A hacker changes the prices in an online shop's database. Which part of the CIA triad was attacked?
- Confidentiality
* Integrity
- Availability
- Authentication
? Unauthorized modification is an integrity failure.

@@quiz
Which is multi-factor authentication?
* A password plus a code from an authenticator app
- A password plus a security question
- Two different passwords
- A long password
? A password and a security question are both "something you know": one factor type.

@@quiz
Someone calls the help desk claiming to be a manager who's locked out, and is in a hurry. What should you do?
* Verify their identity using the company's procedure before resetting anything
- Reset the password right away because they're a manager
- Give them the admin password
- Hang up
? Urgency and authority are classic social engineering tricks. Follow the verification process every time.

@@quiz
Microsoft 365 is an example of...
* SaaS
- IaaS
- PaaS
- A hypervisor
? You just use the software; Microsoft runs everything underneath.

@@quiz
Which runs directly on server hardware without a host OS?
* A Type 1 hypervisor such as ESXi or Hyper-V
- VirtualBox
- Docker Desktop
- A web browser
? Type 2 hypervisors like VirtualBox run on top of a desktop OS.

@@flashcards
CIA triad :: Confidentiality, Integrity, Availability
authentication vs authorization :: Who you are vs what you may do
MFA factors :: Something you know, have, are
phishing :: Fake messages that steal credentials or deliver malware
ransomware :: Malware that encrypts data for ransom
social engineering :: Manipulating people instead of hacking systems
hypervisor :: Software that runs virtual machines
Type 1 / Type 2 hypervisor :: Bare metal (ESXi, Hyper-V) / hosted (VirtualBox)
container :: A packaged app sharing the host kernel (Docker)
IaaS / PaaS / SaaS :: Rent machines / rent a platform / rent finished software
shared responsibility :: Provider secures the cloud; customer secures what's in it

@@drill
Name the three letters of the security triad (as a word).
= CIA
? Confidentiality, Integrity, Availability.
