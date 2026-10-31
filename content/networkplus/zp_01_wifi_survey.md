@@lesson xp-wifi_survey Wi-Fi Channel Survey
@@text
Use your laptop to survey Wi-Fi: on Windows `netsh wlan show networks mode=bssid`, on Linux `nmcli -f SSID,CHAN,SIGNAL dev wifi`. Parse the output, count networks per channel weighted by signal strength, account for 2.4 GHz channel overlap, and recommend the best channels for 2.4 and 5 GHz.

**You'll practise:** 2.4 vs 5 GHz · channel overlap (1, 6, 11) · signal strength (dBm / %) · parsing command output

## Milestones

1. **Capture**: save the command's output to survey.txt (a sample is included).
2. **Parse**: SSID, BSSID, channel and signal for each access point.
3. **Weight**: stronger neighbours count more; 2.4 GHz channels within 4 of each other overlap.
4. **Recommend**: the best of 1/6/11, and the best 5 GHz channel.
5. **Walk test**: survey from three rooms and compare.

## You're done when

- The recommendation for the sample is 11 (2.4 GHz), and you understand why.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why are only channels 1, 6 and 11 recommended in 2.4 GHz?"*
- *"How do signal percentage and dBm relate?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project wifi_survey Wi-Fi Channel Survey
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Wi-Fi Channel Survey

Use your laptop to survey Wi-Fi: on Windows `netsh wlan show networks mode=bssid`, on Linux `nmcli -f SSID,CHAN,SIGNAL dev wifi`. Parse the output, count networks per channel weighted by signal strength, account for 2.4 GHz channel overlap, and recommend the best channels for 2.4 and 5 GHz.

## Milestones

- [ ] 1. Capture: save the command's output to survey.txt (a sample is included).
- [ ] 2. Parse: SSID, BSSID, channel and signal for each access point.
- [ ] 3. Weight: stronger neighbours count more; 2.4 GHz channels within 4 of each other overlap.
- [ ] 4. Recommend: the best of 1/6/11, and the best 5 GHz channel.
- [ ] 5. Walk test: survey from three rooms and compare.

## Done when

- [ ] The recommendation for the sample is 11 (2.4 GHz), and you understand why.
@@file survey.py
"""Recommend Wi-Fi channels from netsh output: python survey.py survey.txt"""
import re
import sys

text = open(sys.argv[1] if len(sys.argv) > 1 else "survey.txt", encoding="utf-8").read()
aps = []
for m in re.finditer(r"Signal\s*:\s*(\d+)%.*?Channel\s*:\s*(\d+)", text, re.S):
    aps.append({"signal": int(m.group(1)), "channel": int(m.group(2))})
print(len(aps), "access points")
# TODO: weight per channel with overlap, recommend 2.4 and 5 GHz channels
@@file survey.txt
SSID 1 : HomeNet
    BSSID 1                 : aa:bb:cc:00:00:01
         Signal             : 82%
         Channel            : 1
SSID 2 : Neighbour-2G
    BSSID 1                 : aa:bb:cc:00:00:02
         Signal             : 60%
         Channel            : 6
SSID 3 : Cafe
    BSSID 1                 : aa:bb:cc:00:00:03
         Signal             : 40%
         Channel            : 3
SSID 4 : Office-5G
    BSSID 1                 : aa:bb:cc:00:00:04
         Signal             : 70%
         Channel            : 36
