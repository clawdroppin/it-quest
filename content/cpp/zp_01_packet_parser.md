@@lesson xp-packet_parser Network Packet Header Parser
@@text
Read a captured packet as bytes and decode its headers by hand: Ethernet (MACs, EtherType), IPv4 (version, header length, TTL, protocol, addresses, checksum) and TCP (ports, flags). You'll learn byte order, bit masks and safe bounds checking: the foundations of every network tool.

**You'll practise:** std::vector<uint8_t> and std::span · big-endian byte order · bit masks and shifts · bounds checking · std::format or iostreams

## Milestones

1. **Input**: the sample packet as a hex string in packet.txt; convert to bytes.
2. **Ethernet**: destination and source MAC, EtherType 0x0800.
3. **IPv4**: IHL × 4 = header length; protocol 6 = TCP; dotted addresses; verify the checksum.
4. **TCP**: ports and flags (SYN, ACK, FIN, RST, PSH) from the flags byte.
5. **Safety**: reject truncated packets instead of reading past the end.

## You're done when

- The sample decodes to the values in the comments and a truncated packet is rejected cleanly.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I read a 16-bit big-endian number from two bytes?"*
- *"Explain the IPv4 header checksum algorithm."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project packet_parser Network Packet Header Parser
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Network Packet Header Parser

Read a captured packet as bytes and decode its headers by hand: Ethernet (MACs, EtherType), IPv4 (version, header length, TTL, protocol, addresses, checksum) and TCP (ports, flags). You'll learn byte order, bit masks and safe bounds checking: the foundations of every network tool.

## Milestones

- [ ] 1. Input: the sample packet as a hex string in packet.txt; convert to bytes.
- [ ] 2. Ethernet: destination and source MAC, EtherType 0x0800.
- [ ] 3. IPv4: IHL × 4 = header length; protocol 6 = TCP; dotted addresses; verify the checksum.
- [ ] 4. TCP: ports and flags (SYN, ACK, FIN, RST, PSH) from the flags byte.
- [ ] 5. Safety: reject truncated packets instead of reading past the end.

## Done when

- [ ] The sample decodes to the values in the comments and a truncated packet is rejected cleanly.
@@file main.cpp
// Decode Ethernet / IPv4 / TCP headers from a hex dump.
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::uint8_t> fromHex(const std::string& hex) {
    std::vector<std::uint8_t> out;
    for (std::size_t i = 0; i + 1 < hex.size(); i += 2) out.push_back(static_cast<std::uint8_t>(std::stoi(hex.substr(i, 2), nullptr, 16)));
    return out;
}

std::uint16_t be16(const std::vector<std::uint8_t>& b, std::size_t at) { return static_cast<std::uint16_t>(b.at(at) << 8 | b.at(at + 1)); }

int main() {
    std::ifstream in("packet.txt");
    std::string hex;
    in >> hex;
    auto bytes = fromHex(hex);
    std::cout << bytes.size() << " bytes, EtherType 0x" << std::hex << be16(bytes, 12) << "\n";
    // TODO: MAC addresses, IPv4 header, TCP ports and flags, checksum
}
@@file packet.txt
00155d0a0b0c3c22fb112233080045000034a6f240004006b1e6c0a80164c0a8010ad3a401bbe1c4b0a700000000802272102c8d0000020405b40103030801010402
