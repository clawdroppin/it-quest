@@lesson f2-troubleshooting Troubleshooting and the Help Desk
@@text
Employers hire IT support staff for two things: **fixing problems methodically** and **communicating well** while doing it. Both can be learned.

## The troubleshooting methodology

This is CompTIA's version (it's on the A+ exam), and it's what experienced engineers do without thinking:

1. **Identify the problem.** Ask the user, reproduce it, check logs. *What changed?* Back up data before you change anything.
2. **Establish a theory of probable cause.** Question the obvious first (is it plugged in? is the caps lock on?).
3. **Test the theory.** Confirmed → plan the fix. Not confirmed → new theory, or **escalate**.
4. **Establish a plan of action** and think about side effects (does a reboot interrupt a meeting? does a change need approval?).
5. **Implement** the solution, or escalate.
6. **Verify full system functionality**, and add preventive measures if you can.
7. **Document** findings, actions and outcomes.

The question that solves the most problems: **"What changed?"** A new update, a moved desk, a changed password, a new monitor.

## Good questions

- *When did it start? Did it ever work?*
- *Does it happen to anyone else?* (one user → their machine; everyone → the network or a server)
- *Does it happen every time, or sometimes?*
- *What exactly does the error say?* Ask for a screenshot.
- *What were you doing when it happened?*

**Divide and conquer**: if Outlook can't connect, does the web browser work? Does Outlook on the web work? Each answer halves the possibilities.

## Tickets

Work is tracked in **tickets** in an ITSM tool (ServiceNow, Jira Service Management, Zendesk, Freshservice). A good ticket note lets anyone continue the work:

```text
Issue: User cannot print to 3F-PRINTER-02 since this morning. Others on 3F can print.
Checked: printer online, queue empty. User's PC showed driver error 0x0000007e.
Action: removed and re-added the printer via \\print01\3F-PRINTER-02; test page OK.
Result: resolved, user confirmed. Cause: corrupted driver after last night's update.
```

**Priority** usually = **impact** (how many people) × **urgency** (how soon it hurts). The CEO's printer is not automatically priority 1; the whole sales floor offline is.

## Communication

- Use plain words. "Your laptop couldn't reach the server that checks passwords", not "the Kerberos ticket failed".
- **Set expectations**: "I'm going to restart the service. It takes about two minutes; I'll call you back by 10:30."
- Never blame the user, never argue, don't promise what you can't control.
- Respect privacy: don't read their documents, ask before taking remote control, and log off when done.
- **Escalate** when it's beyond your access or skill, with clear notes, not when it's merely boring.

@@quiz
A user says the internet is down. What's the best first question?
* "Is it affecting anyone else near you?"
- "Have you tried a different browser?"
- "Which ISP does the company use?"
- "Can you send me your password so I can test?"
? Scope first: one person points at their device; many people point at the network. (And never ask for passwords.)

@@quiz
According to the CompTIA methodology, what comes right after implementing the solution?
* Verify full system functionality and, if applicable, implement preventive measures
- Document findings
- Establish a theory of probable cause
- Close the ticket immediately
? Verify, then document. Documentation is the final step.

@@quiz
Before replacing a failing hard drive or reinstalling Windows, what should you do first?
* Make sure the user's data is backed up
- Update the BIOS
- Tell the user it's their fault
- Order a new computer
? Step 1 includes "back up data before making changes".

@@quiz
Your theory doesn't hold up after testing, and you're out of ideas. What next?
* Establish a new theory or escalate
- Close the ticket as "could not reproduce"
- Reinstall the operating system
- Ask the user to wait until it fixes itself
? Escalating with good notes is a professional move, not a failure.

@@quiz
How is ticket priority usually decided?
* By impact and urgency
- By the requester's job title
- By the order tickets arrive in
- By how easy the fix is
? Many organizations use an impact × urgency matrix.

@@flashcards
1. Identify the problem :: Question the user, reproduce, ask what changed, back up first
2. Theory of probable cause :: Start with the obvious
3. Test the theory :: Confirm, or new theory / escalate
4. Plan of action :: Consider side effects and approvals
5. Implement :: Fix it, or escalate
6. Verify :: Full functionality; preventive measures
7. Document :: Findings, actions, outcomes
priority :: Impact × urgency
escalate :: Hand over to a higher level of support, with notes
ITSM :: IT service management tools: ServiceNow, Jira Service Management...

@@drill
What is the very last step of the CompTIA troubleshooting methodology? (one word)
= document
= documentation
? Document findings, actions and outcomes, so the next person (or future you) benefits.

@@exercise Did they follow the methodology?
A ticket's work log tags each action with the step of the lesson's methodology it belongs to: `identify`, `theory`, `test`, `plan`, `implement`, `verify`, `document`, or `escalate`. Write `audit(log)` that returns the problems:

- steps may repeat (a failed test sends you back to `theory`) but must never jump **forward** past a step that hasn't happened yet (implementing before testing a theory);
- `verify` and `document` must both happen before the ticket closes, unless it was escalated;
- the log should mention a backup before the first `implement` when the fix touches data (`touches_data`).

Print each ticket with `followed the methodology` or its problems.
@@lang python
@@starter
ORDER = ["identify", "theory", "test", "plan", "implement", "verify", "document"]
tickets = {
    "INC-301 printer offline": {"touches_data": False, "log": [
        ("identify", "user can't print, others can"), ("theory", "wrong default printer"),
        ("test", "default is the old printer: confirmed"), ("plan", "set the new default"),
        ("implement", "changed default"), ("verify", "test page printed"), ("document", "noted in ticket")]},
    "INC-302 Outlook crash": {"touches_data": True, "log": [
        ("identify", "crashes on start"), ("implement", "rebuilt the profile"), ("verify", "opens now")]},
    "INC-303 slow laptop": {"touches_data": False, "log": [
        ("identify", "slow since Monday"), ("theory", "update running"), ("test", "no update: wrong"),
        ("theory", "full disk"), ("test", "disk 99% full: confirmed"), ("plan", "clear temp files"),
        ("implement", "cleared 40 GB"), ("verify", "fast again")]},
    "INC-304 VPN for one user": {"touches_data": False, "log": [
        ("identify", "VPN fails for one user"), ("theory", "certificate expired"),
        ("test", "not confirmed"), ("escalate", "sent to network team")]},
    "INC-305 file share": {"touches_data": True, "log": [
        ("identify", "folder missing"), ("theory", "permissions changed"), ("test", "confirmed"),
        ("plan", "backup taken, then restore permissions"), ("implement", "restored ACL"),
        ("verify", "user sees folder"), ("document", "recorded the change")]},
}

def audit(ticket):
    raise NotImplementedError


for name, t in tickets.items():
    problems = audit(t)
    print(f"{name}: " + ("followed the methodology" if not problems else "; ".join(problems)))
@@expected
INC-301 printer offline: followed the methodology
INC-302 Outlook crash: jumped to implement before theory; no document step; no backup before changing data
INC-303 slow laptop: no document step
INC-304 VPN for one user: followed the methodology
INC-305 file share: followed the methodology
@@requires
def audit
ORDER.index
@@hint
Track the furthest step reached so far. A step is fine if it is at most one further than that; going back to an earlier step is allowed.
@@solution
ORDER = ["identify", "theory", "test", "plan", "implement", "verify", "document"]
tickets = {
    "INC-301 printer offline": {"touches_data": False, "log": [
        ("identify", "user can't print, others can"), ("theory", "wrong default printer"),
        ("test", "default is the old printer: confirmed"), ("plan", "set the new default"),
        ("implement", "changed default"), ("verify", "test page printed"), ("document", "noted in ticket")]},
    "INC-302 Outlook crash": {"touches_data": True, "log": [
        ("identify", "crashes on start"), ("implement", "rebuilt the profile"), ("verify", "opens now")]},
    "INC-303 slow laptop": {"touches_data": False, "log": [
        ("identify", "slow since Monday"), ("theory", "update running"), ("test", "no update: wrong"),
        ("theory", "full disk"), ("test", "disk 99% full: confirmed"), ("plan", "clear temp files"),
        ("implement", "cleared 40 GB"), ("verify", "fast again")]},
    "INC-304 VPN for one user": {"touches_data": False, "log": [
        ("identify", "VPN fails for one user"), ("theory", "certificate expired"),
        ("test", "not confirmed"), ("escalate", "sent to network team")]},
    "INC-305 file share": {"touches_data": True, "log": [
        ("identify", "folder missing"), ("theory", "permissions changed"), ("test", "confirmed"),
        ("plan", "backup taken, then restore permissions"), ("implement", "restored ACL"),
        ("verify", "user sees folder"), ("document", "recorded the change")]},
}

def audit(ticket):
    problems, reached = [], -1
    for step, note in ticket["log"]:
        if step == "escalate":
            continue
        pos = ORDER.index(step)
        if pos > reached + 1:
            problems.append(f"jumped to {step} before {ORDER[reached + 1]}")
        reached = max(reached, pos)
    steps = [s for s, _ in ticket["log"]]
    if "escalate" not in steps:
        for needed in ("verify", "document"):
            if needed not in steps:
                problems.append(f"no {needed} step")
    if ticket["touches_data"] and "implement" in steps:
        before = ticket["log"][:steps.index("implement")]
        if not any("backup" in note for _, note in before):
            problems.append("no backup before changing data")
    return problems

for name, t in tickets.items():
    problems = audit(t)
    print(f"{name}: " + ("followed the methodology" if not problems else "; ".join(problems)))
