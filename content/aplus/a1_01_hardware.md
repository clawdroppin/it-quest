@@lesson a1-hardware PC and Laptop Hardware
@@text
## Memory (RAM)

| Type | Notes |
|---|---|
| **DIMM** | desktop memory sticks |
| **SO-DIMM** | smaller sticks for laptops and mini PCs |
| **DDR4** / **DDR5** | both 288 pins, but the notch is in a different place: they can't be swapped |
| **ECC** | detects and corrects memory errors; used in servers |

Install sticks in matching pairs in the right slots (see the motherboard manual) to get **dual-channel** speed. A PC that beeps and won't show anything after a RAM upgrade usually has a stick not fully seated.

## Storage interfaces

- **SATA**: 2.5"/3.5" drives, up to 6 Gb/s. SSDs and HDDs.
- **M.2**: a slot form factor. An M.2 drive can be **SATA** or **NVMe** (PCIe): check which the slot supports, and the **key** (M-key for NVMe, B+M for SATA).
- **NVMe**: talks over PCIe lanes, several GB/s.
- HDD speeds: 5,400 and 7,200 RPM (desktop), 10k/15k (old servers).

## CPU, motherboard, power

- CPUs fit a **socket**: Intel uses **LGA** (pins on the board), AMD's AM5 is also LGA, older AMD AM4 is **PGA** (pins on the CPU). Always use **thermal paste** between CPU and cooler.
- Form factors: **ATX** (standard), **microATX** (smaller), **mini-ITX** (tiny).
- Expansion slots: **PCIe** x1, x4, x16 (graphics cards use x16).
- **PSU**: rated in watts; connectors are the 24-pin motherboard, 8-pin (4+4) CPU, PCIe 6/8-pin or 12VHPWR for GPUs, SATA power. **Modular** PSUs let you attach only the cables you need. A PSU's 80 PLUS rating describes its efficiency.
- **Firmware** settings (UEFI): boot order, Secure Boot, TPM, virtualization support (Intel VT-x / AMD-V, needed for VMs), fan curves, a supervisor password.

## Ports and cables

| Connector | Use |
|---|---|
| **USB-C** | reversible; data, video and power (**USB PD**) |
| **Thunderbolt 3/4** | uses USB-C plugs; 40 Gb/s, docks, external GPUs |
| **USB 2.0 / 3.x** | 480 Mb/s / 5–20 Gb/s (blue ports are usually 3.x) |
| **HDMI** | TVs and monitors, carries audio |
| **DisplayPort** | monitors, daisy-chaining; **VGA** (analog, blue, 15 pins) and **DVI** are legacy |
| **RJ45** | Ethernet; **RJ11** is the smaller phone/DSL plug |

## Laptops and mobile devices

- Laptop parts commonly replaced: **battery**, RAM (if not soldered), SSD, keyboard, screen, Wi-Fi card. Disconnect the battery before opening, and use the right screwdrivers and plastic pry tools.
- A **swollen battery** is a fire risk: stop using it, don't puncture it, and recycle it properly.
- Screens: **LCD** (IPS has the best colours and angles, TN is fast and cheap, VA has deep blacks), **OLED** (each pixel lights itself, true blacks). A dim but visible image on an old LCD = backlight/inverter failure.
- Wireless: Wi-Fi, **Bluetooth** (pair a device: enable, discover, enter/confirm a PIN, test), **NFC** (tap to pay), cellular.
- Company phones are managed with **MDM** (Intune, Jamf): push settings and apps, require a PIN, remote wipe a lost device. **BYOD** means staff's own devices.

@@quiz
A user upgraded their desktop from DDR4 to new DDR5 memory, but the DDR5 sticks won't fit. Why?
* DDR4 and DDR5 have different notch positions and the motherboard only supports one type
- They need more force
- DDR5 is only for laptops
- The BIOS must be updated first to make them fit
? Memory generations are keyed differently on purpose. The motherboard decides which type you can use.

@@quiz
A technician wants to install an M.2 NVMe SSD, but the drive isn't detected. What should they check?
* Whether the M.2 slot supports NVMe/PCIe and not only SATA
- Whether the SATA power cable is connected
- Whether the drive is 7,200 RPM
- Whether the drive is formatted FAT32
? M.2 is a shape; the slot might only support SATA drives, and M.2 drives don't use SATA power cables.

@@quiz
Which connector carries data, video and up to 240 W of power over one reversible cable?
* USB-C with USB Power Delivery
- HDMI
- VGA
- RJ45
? USB-C with USB PD (and Thunderbolt) powers laptops and docks.

@@quiz
A laptop's screen is very dim, and you can just see the image with a flashlight. What has most likely failed?
* The backlight
- The GPU
- The OS display driver
- The digitizer
? The LCD panel still draws the image; the light behind it is gone.

@@quiz
Which BIOS/UEFI setting must be enabled to run 64-bit virtual machines?
* Hardware virtualization (Intel VT-x or AMD-V)
- Secure Boot
- Fast Boot
- Legacy USB support
? Hypervisors need CPU virtualization extensions.

@@quiz
A user's phone is lost with company email on it. What lets IT protect the data?
* MDM remote wipe
- Changing the user's desk phone
- Bluetooth pairing
- Disabling NFC
? Mobile Device Management can lock or wipe managed devices.

@@flashcards
SO-DIMM :: Laptop memory module
ECC RAM :: Corrects memory errors; for servers
dual-channel :: Matched sticks in paired slots for more bandwidth
M.2 :: Small SSD slot; SATA or NVMe depending on drive and slot
NVMe :: SSD over PCIe; much faster than SATA
LGA / PGA :: Pins on the motherboard / on the CPU
ATX / microATX / mini-ITX :: Motherboard sizes from large to small
PCIe x16 :: Slot for graphics cards
Thunderbolt :: 40 Gb/s over USB-C connectors
DisplayPort :: Monitor connector; supports daisy-chaining
RJ45 / RJ11 :: Ethernet / phone line connector
MDM :: Mobile device management: policies, apps, remote wipe
OLED :: Each pixel emits its own light; no backlight

@@drill
How many pins does a DDR5 desktop DIMM have?
= 288
? Same count as DDR4, different notch.
