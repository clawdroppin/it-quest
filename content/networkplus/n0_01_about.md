@@lesson n0-about About Network+
@@text
**CompTIA Network+** proves you can design, run and troubleshoot networks. It's the usual next step after A+ for NOC technicians, junior network and systems admins, and it's a solid base before Cisco's CCNA. The current exam is **N10-009**.

| Domain | Roughly |
|---|---|
| Networking concepts | OSI, ports and protocols, addressing, cloud, topologies |
| Network implementation | routing, switching, wireless, physical installation |
| Network operations | documentation, monitoring, disaster recovery, IPv4/IPv6 services, access |
| Network security | concepts, attacks, hardening |
| Network troubleshooting | methodology, cabling, services, performance, tools |

Up to **90 questions in 90 minutes**, including performance-based simulations; passing score **720** (100–900). Check comptia.org for the current objectives before you book.

**Subnetting** is the skill that decides this exam and many job interviews. Practise until you can do a /27 in your head in 20 seconds; the subnetting lesson and the Network Toolkit build in the Python track will get you there.
## How this track is organized

The lessons follow the five exam domains. The first modules cover concepts (OSI model, ports, addressing, subnetting), and the later ones cover implementation, operations, security and troubleshooting. Every quiz question goes into the Network+ practice exam bank in Review, and the flashcards feed your daily spaced-repetition reviews.

## Study plan

A realistic plan is six to eight weeks at about an hour a day:

1. **Weeks 1-2:** the OSI model, common ports and protocols, IPv4 and IPv6 addressing. Do subnetting drills every day from day one.
2. **Weeks 3-4:** switching (VLANs, trunking, spanning tree), routing (static routes, OSPF, BGP basics) and wireless standards.
3. **Weeks 5-6:** operations and security: monitoring, SNMP, syslog, backups and disaster recovery, attacks and hardening.
4. **Weeks 7-8:** troubleshooting methodology and tools, then timed practice exams. Book the real exam once you're scoring 85% or more.

## Hands-on practice

Network+ has performance-based questions, so practise on something real:

- Use `ipconfig /all`, `ping`, `tracert`, `nslookup` and `netstat` (or `ip`, `ss` and `dig` on Linux) on your own machine until you can read their output quickly.
- Log in to your home router and find its DHCP range, its DNS servers and its port-forwarding settings.
- Install a packet analyser such as Wireshark and watch a DHCP exchange (Discover, Offer, Request, Acknowledge) and a DNS lookup.
- Free network simulators let you build VLANs and routes without buying hardware.

## The troubleshooting methodology

CompTIA's seven-step method appears on every exam. Learn its order:

1. Identify the problem.
2. Establish a theory of probable cause.
3. Test the theory to determine the cause.
4. Establish a plan of action and identify potential effects.
5. Implement the solution or escalate as necessary.
6. Verify full system functionality and implement preventive measures.
7. Document findings, actions, outcomes and lessons learned.

Many questions ask what to do NEXT in a scenario. Work out which step has just finished and choose the following one.

## After Network+

Network+ is valid for three years. It pairs naturally with **Security+**, and it prepares you for **CCNA**, which goes deeper into Cisco configuration.

@@quiz
Which skill is most worth drilling until it's automatic for Network+ and network job interviews?
* Subnetting
- Memorizing cable colours
- Wi-Fi channel numbers above 100
- Printer protocols
? Subnetting questions appear throughout the exam and are a classic interview test.

@@quiz
What is the first step of the CompTIA troubleshooting methodology?
* Identify the problem
- Establish a theory of probable cause
- Implement the solution
- Document findings
? Gather information, question users, identify symptoms and recent changes before forming a theory.

@@quiz
A technician has implemented a fix. According to the methodology, what comes next?
* Verify full system functionality and implement preventive measures
- Document findings
- Establish a new theory
- Escalate the problem
? Documentation is the final step, after verification.

@@quiz
What is the passing score for Network+ N10-009?
* 720 on a scale of 100-900
- 675
- 700
- 800
? A+ Core 1 is 675 and Core 2 is 700; Network+ is 720.

@@flashcards
Network+ exam :: N10-009; 90 questions, 90 minutes, pass 720/900
Network+ domains :: Concepts, implementation, operations, security, troubleshooting
Troubleshooting step 1 :: Identify the problem
Troubleshooting step 7 :: Document findings, actions, outcomes and lessons learned
