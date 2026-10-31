@@lesson a2-security A+ Security
@@text
## Malware

| Type | What it does |
|---|---|
| **Virus** | attaches to files, spreads when run |
| **Worm** | spreads by itself over the network |
| **Trojan** | pretends to be something useful |
| **Ransomware** | encrypts files, demands payment |
| **Spyware / keylogger** | watches and records what you do |
| **Rootkit** | hides deep in the OS, hard to detect |
| **Botnet** | infected machines controlled remotely |
| **Cryptominer** | steals CPU/GPU to mine cryptocurrency |

Symptoms: pop-ups, browser redirects, a new toolbar or search engine, slow performance, fans busy while idle, disabled antivirus, files renamed with odd extensions, certificate warnings.

## The malware removal process

CompTIA's order, worth memorizing:

1. **Investigate and verify** malware symptoms.
2. **Quarantine** the infected system (disconnect it from the network).
3. **Disable System Restore** (in Windows), so infected restore points don't come back.
4. **Remediate**: update the anti-malware software, then scan and remove (Safe Mode, a boot-time scan or recovery environment if needed).
5. **Schedule scans and run updates.**
6. **Enable System Restore** and create a new restore point.
7. **Educate the end user.**

## Windows security features

- **Microsoft Defender Antivirus** and **Defender Firewall** (separate rules for Domain, Private and Public networks).
- **BitLocker** encrypts whole drives (uses the TPM); **BitLocker To Go** encrypts USB drives. Store the **recovery key** safely (in AD or Entra ID). **EFS** encrypts individual files.
- **NTFS permissions** (Full control, Modify, Read & execute, List, Read, Write) vs **share permissions** (Full, Change, Read). Over the network, the **more restrictive** of the two applies. **Deny** overrides Allow. Copying a file gives it the new folder's permissions; moving it within the same volume keeps them.
- Account security: strong passwords, lockout after failed attempts, disable the **Guest** account, rename/disable the built-in Administrator, screen lock timeouts.

## Wireless and SOHO security

WPA3 (or WPA2-AES), change default router passwords, disable **WPS**, update firmware, guest networks, MAC filtering (weak), content filtering. **RADIUS/TACACS+** and **802.1X** give each user their own Wi-Fi login in companies (WPA2/WPA3-**Enterprise**).

## Social engineering and physical security

**Phishing** (email), **vishing** (voice), **smishing** (SMS), **whaling** (targeting executives), **tailgating** (following someone through a secure door), **shoulder surfing**, **impersonation**, **dumpster diving**. Physical controls: badge readers, **access control vestibules** (mantraps), security guards, cable locks, privacy screens, locked server rooms, video surveillance.

## Data destruction

- **Erasing / wiping** (overwriting) lets a drive be reused.
- **Degaussing** destroys magnetic data (HDDs, tapes), not SSDs.
- **Physical destruction**: shredding, drilling, incineration, with a **certificate of destruction** from third parties.
- A normal **quick format does not remove data.**

@@quiz
A user's browser homepage changed, and pop-ups appear constantly. You confirmed it's malware. What's the NEXT step?
* Quarantine the system by disconnecting it from the network
- Disable System Restore
- Educate the user
- Schedule regular scans
? After investigating and verifying, quarantine before anything else.

@@quiz
Why disable System Restore during malware removal?
* So the malware isn't restored later from infected restore points
- To make the scan faster
- Because antivirus can't run with it on
- To free space for the antivirus
? Re-enable it and create a clean restore point afterwards.

@@quiz
A folder is shared with share permission "Read" for Everyone, and NTFS permission "Modify" for Sales. What can a Sales user do when accessing it over the network?
* Only read
- Modify
- Full control
- Nothing
? Over the network the more restrictive of share and NTFS permissions applies.

@@quiz
A laptop containing customer data is stolen. Which feature would have kept the data unreadable?
* BitLocker full-disk encryption
- Windows Defender Firewall
- A strong Wi-Fi password
- System Restore
? Without the key, an encrypted disk is useless to a thief.

@@quiz
Someone without a badge follows an employee through a secured door. What is this?
* Tailgating
- Phishing
- Shoulder surfing
- Whaling
? An access control vestibule (mantrap) prevents it.

@@quiz
Old SSDs must be disposed of so data can't be recovered. Which method doesn't work for SSDs?
* Degaussing
- Shredding
- Secure erase by the manufacturer's tool
- Incineration
? SSDs store data in flash memory, not magnetically.

@@quiz
Which malware is designed to hide itself deep in the OS and evade detection?
* Rootkit
- Adware
- Worm
- Keylogger
? Rootkits often need offline or boot-time scanning, or a reinstall.

@@flashcards
malware removal 1-7 :: Investigate, quarantine, disable System Restore, remediate, schedule scans/updates, enable System Restore, educate
worm :: Self-spreading malware
trojan :: Malware disguised as legitimate software
rootkit :: Hides deep in the OS
BitLocker :: Full-disk encryption; BitLocker To Go for USB
EFS :: Encrypts individual files on NTFS
share + NTFS permissions :: The more restrictive applies over the network
Deny :: Overrides Allow
copy vs move permissions :: Copy inherits the new folder's; move on the same volume keeps them
WPA2/3-Enterprise :: Individual Wi-Fi logins via 802.1X and RADIUS
tailgating :: Following someone through a secure door
vishing / smishing :: Phishing by voice / SMS
degaussing :: Erases magnetic media (not SSDs)

@@drill
What is the last step of the CompTIA malware removal process? (two words)
= educate user
= educate the user
= educate the end user
= educate end user
? Most infections start with a click; teaching prevents the next one.
