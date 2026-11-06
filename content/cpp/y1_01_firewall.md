@@lesson x9-firewall Firewall Rule Engine
@@text
Every router ACL, Linux `iptables`/`nftables` chain, and AWS or Azure security group works the same way: **a list of rules, checked from the top, where the first match wins** and anything unmatched is denied. In this build you write that engine in C++, the language real packet filters are written in.

```text
8 rules loaded from rules.txt
  warning: line 8 is shadowed by line 3

allow tcp  10.10.3.41      -> 192.168.1.10    443   line 3
allow tcp  10.10.50.7      -> 192.168.1.20    22    line 4
deny  tcp  10.10.3.41      -> 192.168.1.20    22    line 5
deny  tcp  10.10.3.41      -> 142.250.4.100   8080  default deny
deny  icmp 10.10.3.41      -> 8.8.8.8         0     default deny
...
allowed 5, denied 5
  line 8 was never used
```

The **shadowed rule** check is what network auditors look for: a rule that can never match because an earlier, broader rule always matches first. Line 8 of the sample rules was meant to let guests reach the intranet, but line 3 already decides every packet it could match.

What you practise: parsing with `std::string` and `std::optional`, bit masks for IPv4 and CIDR (the same maths as subnetting for the CCNA), structs, exceptions with useful messages, and `std::vector` algorithms. Everything lives in one header, `firewall.hpp`, with a test program next to it.

Building, from the project folder (or use the buttons on the card):

```bash,norun
g++ -std=c++20 -Wall -Wextra -o tests tests.cpp && ./tests       # Linux / MinGW
cl /EHsc /std:c++20 /W4 tests.cpp && tests.exe                    # Windows, Developer Command Prompt
```

Write the functions in `firewall.hpp` in order. Each has a comment saying exactly what it must do. When all 20 tests pass, build and run `main.cpp`.
@@project firewall Firewall Rule Engine
Write the functions in `firewall.hpp` (each starts with `throw std::logic_error("TODO...")`) until all tests pass. Then run `main` on the sample `rules.txt` and `traffic.txt`, and try your own rules.
@@file firewall.hpp
// firewall.hpp: a first-match packet filter, like an ACL on a router or a cloud security group.
#pragma once

#include <cstdint>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace fw {

/// "192.168.1.10" -> 3232235786. Exactly four parts, each 0-255 and digits only.
/// Anything else ("1.2.3", "256.1.1.1", "a.b.c.d", "1.2.3.4.5", "") -> std::nullopt.
inline std::optional<std::uint32_t> parseIpv4(const std::string& text) {
    throw std::logic_error("TODO: not done yet");
}

/// The mask for a prefix length: /24 -> 0xFFFFFF00, /0 -> 0.
inline std::uint32_t maskOf(int prefix) {
    return prefix == 0 ? 0u : 0xFFFFFFFFu << (32 - prefix);
}

struct Cidr {
    std::uint32_t network = 0;
    int prefix = 0;

    /// True if ip is inside this network.
    bool contains(std::uint32_t ip) const {
        throw std::logic_error("TODO: not done yet");
    }

    /// True if every address in `other` is also in this network.
    bool covers(const Cidr& other) const {
        throw std::logic_error("TODO: not done yet");
    }
};

/// "10.0.0.0/8" -> {0x0A000000, 8}. A plain address means /32, "any" means 0.0.0.0/0.
/// Host bits are cleared: "10.1.2.3/8" -> 10.0.0.0/8. Bad text or a prefix outside 0-32 -> nullopt.
inline std::optional<Cidr> parseCidr(const std::string& text) {
    throw std::logic_error("TODO: not done yet");
}

struct Rule {
    std::string action;   // "allow" or "deny"
    std::string proto;    // "tcp", "udp", "icmp" or "any"
    Cidr src, dst;
    int portLow = 0, portHigh = 65535;
    int line = 0;         // where it came from, for messages
};

/// Parse "allow tcp 10.0.0.0/8 any 443". The port is a number, a range "1000-2000" or "any".
/// Errors throw std::invalid_argument("line <n>: <what>") with these whats:
/// "expected 5 fields", "bad action <x>", "bad protocol <x>", "bad address <x>", "bad port <x>".
inline Rule parseRule(const std::string& text, int line) {
    throw std::logic_error("TODO: not done yet");
}

struct Packet {
    std::string proto;
    std::uint32_t src = 0, dst = 0;
    int port = 0;   // destination port (0 for icmp)
};

/// True if the rule matches the packet: same protocol (or rule proto "any"), both addresses
/// inside the rule's networks, and the port in range. For icmp packets the port is ignored.
inline bool matches(const Rule& r, const Packet& p) {
    throw std::logic_error("TODO: not done yet");
}

struct Verdict {
    std::string action;
    int ruleLine = 0;   // 0 = no rule matched: the implicit "deny all" at the end
};

/// First match wins, like a router ACL. No match -> {"deny", 0}.
inline Verdict evaluate(const std::vector<Rule>& rules, const Packet& p) {
    throw std::logic_error("TODO: not done yet");
}

/// Rules that can never match because an earlier rule matches everything they would:
/// the earlier rule has the same protocol or "any", covers both networks, and covers the port range.
/// One message per shadowed rule, in order: "line 7 is shadowed by line 3".
inline std::vector<std::string> shadowedRules(const std::vector<Rule>& rules) {
    throw std::logic_error("TODO: not done yet");
}

}  // namespace fw
@@file tests.cpp
// tests.cpp: a tiny test runner, so the project needs nothing installed.
#include <functional>
#include <iostream>
#include <string>

#include "firewall.hpp"

using namespace fw;

static int passed = 0, failed = 0;

static void check(const std::string& name, const std::function<bool()>& test) {
    try {
        if (test()) {
            ++passed;
            std::cout << "  ok    " << name << '\n';
            return;
        }
        std::cout << "  FAIL  " << name << '\n';
    } catch (const std::exception& e) {
        if (std::string(e.what()).starts_with("TODO")) {
            std::cout << "  TODO  " << name << '\n';
            ++failed;
            return;
        }
        std::cout << "  FAIL  " << name << ": exception: " << e.what() << '\n';
    }
    ++failed;
}

static bool throwsWith(const std::string& rule, const std::string& message) {
    try {
        parseRule(rule, 4);
    } catch (const std::invalid_argument& e) {
        return e.what() == message;
    }
    return false;
}

static Packet pkt(const std::string& proto, const std::string& src, const std::string& dst, int port) {
    return {proto, *parseIpv4(src), *parseIpv4(dst), port};
}

int main() {
    std::cout << "parseIpv4\n";
    check("parses a dotted quad", [] { return parseIpv4("192.168.1.10") == 3232235786u; });
    check("0.0.0.0 and 255.255.255.255", [] { return parseIpv4("0.0.0.0") == 0u && parseIpv4("255.255.255.255") == 0xFFFFFFFFu; });
    check("rejects bad addresses", [] {
        for (std::string bad : {"1.2.3", "256.1.1.1", "a.b.c.d", "1.2.3.4.5", "", "1..2.3", "1.2.3.-4"})
            if (parseIpv4(bad)) return false;
        return true;
    });

    std::cout << "Cidr\n";
    check("parses a network", [] { auto c = parseCidr("10.0.0.0/8"); return c && c->network == 0x0A000000u && c->prefix == 8; });
    check("plain address is /32, any is /0", [] { return parseCidr("10.1.1.1")->prefix == 32 && parseCidr("any")->prefix == 0; });
    check("clears host bits", [] { return parseCidr("10.1.2.3/8")->network == 0x0A000000u; });
    check("rejects bad prefixes", [] { return !parseCidr("10.0.0.0/33") && !parseCidr("10.0.0.0/") && !parseCidr("10.0.0.0/x"); });
    check("contains", [] {
        auto net = *parseCidr("192.168.1.0/26");
        return net.contains(*parseIpv4("192.168.1.63")) && !net.contains(*parseIpv4("192.168.1.64"));
    });
    check("any contains everything", [] { return parseCidr("any")->contains(*parseIpv4("8.8.8.8")); });
    check("covers", [] { return parseCidr("10.0.0.0/8")->covers(*parseCidr("10.20.0.0/16")) && !parseCidr("10.20.0.0/16")->covers(*parseCidr("10.0.0.0/8")); });

    std::cout << "parseRule\n";
    check("parses a rule with one port", [] {
        Rule r = parseRule("allow tcp 10.0.0.0/8 any 443", 3);
        return r.action == "allow" && r.proto == "tcp" && r.src.prefix == 8 && r.dst.prefix == 0 && r.portLow == 443 && r.portHigh == 443 && r.line == 3;
    });
    check("port ranges and any", [] {
        Rule a = parseRule("deny udp any any 1000-2000", 1), b = parseRule("deny icmp any any any", 2);
        return a.portLow == 1000 && a.portHigh == 2000 && b.portLow == 0 && b.portHigh == 65535;
    });
    check("error messages", [] {
        return throwsWith("allow tcp any any", "line 4: expected 5 fields") &&
               throwsWith("permit tcp any any 80", "line 4: bad action permit") &&
               throwsWith("allow sctp any any 80", "line 4: bad protocol sctp") &&
               throwsWith("allow tcp 10.0.0.0/40 any 80", "line 4: bad address 10.0.0.0/40") &&
               throwsWith("allow tcp any any 70000", "line 4: bad port 70000") &&
               throwsWith("allow tcp any any 90-80", "line 4: bad port 90-80");
    });

    std::cout << "evaluate\n";
    std::vector<Rule> rules = {
        parseRule("allow tcp 10.0.0.0/8 192.168.1.10 443", 1),
        parseRule("deny tcp 10.66.0.0/16 any any", 2),
        parseRule("allow udp any 192.168.1.53 53", 3),
        parseRule("allow icmp 10.0.0.0/8 any any", 4),
    };
    check("first match wins", [&] { auto v = evaluate(rules, pkt("tcp", "10.66.1.1", "192.168.1.10", 443)); return v.action == "allow" && v.ruleLine == 1; });
    check("later rule matches", [&] { auto v = evaluate(rules, pkt("tcp", "10.66.1.1", "192.168.1.11", 22)); return v.action == "deny" && v.ruleLine == 2; });
    check("protocol must match", [&] { return evaluate(rules, pkt("udp", "10.1.1.1", "192.168.1.10", 443)).ruleLine == 0; });
    check("icmp ignores ports", [&] { return evaluate(rules, pkt("icmp", "10.9.9.9", "8.8.8.8", 0)).ruleLine == 4; });
    check("implicit deny", [&] { auto v = evaluate(rules, pkt("tcp", "172.16.0.1", "192.168.1.10", 443)); return v.action == "deny" && v.ruleLine == 0; });

    std::cout << "shadowedRules\n";
    check("finds shadowed rules", [] {
        std::vector<Rule> r = {
            parseRule("allow tcp 10.0.0.0/8 any 1-1024", 1),
            parseRule("deny tcp 10.20.0.0/16 any 22", 2),        // shadowed by 1
            parseRule("deny any any any any", 3),
            parseRule("allow udp 10.0.0.0/8 any 53", 4),         // shadowed by 3
        };
        auto s = shadowedRules(r);
        return s.size() == 2 && s[0] == "line 2 is shadowed by line 1" && s[1] == "line 4 is shadowed by line 3";
    });
    check("partial overlap isn't shadowing", [] {
        std::vector<Rule> r = {parseRule("allow tcp 10.0.0.0/16 any 80", 1), parseRule("deny tcp 10.0.0.0/8 any 80", 2)};
        return shadowedRules(r).empty();
    });

    std::cout << '\n' << passed << " passed, " << failed << " failed\n";
    return failed == 0 ? 0 : 1;
}
@@file main.cpp
// main.cpp: check traffic against a rule file.  Usage: main [rules.txt] [traffic.txt]
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

#include "firewall.hpp"

static std::vector<fw::Rule> loadRules(const std::string& path) {
    std::ifstream in(path);
    if (!in) throw std::runtime_error("can't open " + path);
    std::vector<fw::Rule> rules;
    std::string line;
    for (int n = 1; std::getline(in, line); ++n) {
        auto hash = line.find('#');
        if (hash != std::string::npos) line.erase(hash);          // comments
        if (line.find_first_not_of(" \t\r") == std::string::npos) continue;
        rules.push_back(fw::parseRule(line, n));
    }
    return rules;
}

int main(int argc, char** argv) {
    std::string rulesPath = argc > 1 ? argv[1] : "rules.txt";
    std::string trafficPath = argc > 2 ? argv[2] : "traffic.txt";
    try {
        auto rules = loadRules(rulesPath);
        std::cout << rules.size() << " rules loaded from " << rulesPath << '\n';
        for (const auto& warning : fw::shadowedRules(rules)) std::cout << "  warning: " << warning << '\n';

        std::ifstream traffic(trafficPath);
        if (!traffic) throw std::runtime_error("can't open " + trafficPath);
        std::map<std::string, int> totals;
        std::map<int, int> hitsPerRule;
        std::string proto, src, dst;
        int port = 0;
        std::cout << '\n';
        while (traffic >> proto >> src >> dst >> port) {
            auto s = fw::parseIpv4(src), d = fw::parseIpv4(dst);
            if (!s || !d) {
                std::cout << "skipping bad packet " << src << " -> " << dst << '\n';
                continue;
            }
            auto v = fw::evaluate(rules, {proto, *s, *d, port});
            totals[v.action]++;
            hitsPerRule[v.ruleLine]++;
            std::cout << std::left << std::setw(6) << v.action << std::setw(5) << proto << std::setw(16) << src << "-> "
                      << std::setw(16) << dst << std::setw(6) << port
                      << (v.ruleLine ? "line " + std::to_string(v.ruleLine) : std::string("default deny")) << '\n';
        }
        std::cout << "\nallowed " << totals["allow"] << ", denied " << totals["deny"] << '\n';
        for (const auto& r : rules)
            if (!hitsPerRule.contains(r.line)) std::cout << "  line " << r.line << " was never used\n";
    } catch (const std::exception& e) {
        std::cerr << "error: " << e.what() << '\n';
        return 1;
    }
}
@@file rules.txt
# Office firewall: first match wins, everything else is denied.
# action  proto  source          destination      port
allow     tcp    10.10.0.0/16    192.168.1.10     443          # staff -> intranet (HTTPS)
allow     tcp    10.10.50.0/24   192.168.1.20     22           # IT team -> jump host (SSH)
deny      tcp    10.10.0.0/16    192.168.1.20     22           # everyone else: no SSH
allow     udp    any             192.168.1.53     53           # DNS
allow     tcp    10.10.0.0/16    any              80-443       # staff -> internet
allow     tcp    10.10.7.0/24    192.168.1.10     443          # guests -> intranet?? (shadowed)
allow     icmp   10.10.50.0/24   any              any          # IT may ping anything
deny      tcp    any             192.168.1.30     3389         # no RDP to the file server
@@file traffic.txt
tcp  10.10.3.41   192.168.1.10   443
tcp  10.10.50.7   192.168.1.20   22
tcp  10.10.3.41   192.168.1.20   22
udp  10.10.3.41   192.168.1.53   53
tcp  10.10.3.41   142.250.4.100  443
tcp  10.10.3.41   142.250.4.100  8080
icmp 10.10.50.7   8.8.8.8        0
icmp 10.10.3.41   8.8.8.8        0
tcp  172.16.9.9   192.168.1.10   443
tcp  10.10.3.41   192.168.1.30   3389
@@text
> Tips: in `parseIpv4`, split on `'.'` with `find`/`substr` and check each part is 1–3 digits and at most 255 before shifting it in: `ip = (ip << 8) | part`. A network contains an address when `(ip & mask) == network`. Network A covers B when A's prefix is shorter or equal *and* A contains B's network address. In `parseRule`, an `std::istringstream` splits the line into words.
@@project firewall_finished Firewall Rule Engine: the finished version
The complete engine. Ideas: support IPv6 (`std::array<std::uint8_t, 16>`), stateful rules ("allow established"), a rule that's *partly* shadowed, hit counters written to a file, or reading real `iptables-save` output.
@@file firewall.hpp
// firewall.hpp: a first-match packet filter, like an ACL on a router or a cloud security group.
#pragma once

#include <cstdint>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace fw {

/// "192.168.1.10" -> 3232235786. Exactly four parts, each 0-255 and digits only.
/// Anything else ("1.2.3", "256.1.1.1", "a.b.c.d", "1.2.3.4.5", "") -> std::nullopt.
inline std::optional<std::uint32_t> parseIpv4(const std::string& text) {
    std::uint32_t ip = 0;
    int parts = 0;
    std::size_t start = 0;
    while (true) {
        std::size_t dot = text.find('.', start);
        std::string part = text.substr(start, dot == std::string::npos ? std::string::npos : dot - start);
        if (part.empty() || part.size() > 3 || part.find_first_not_of("0123456789") != std::string::npos) return std::nullopt;
        int value = std::stoi(part);
        if (value > 255) return std::nullopt;
        ip = (ip << 8) | static_cast<std::uint32_t>(value);
        ++parts;
        if (dot == std::string::npos) break;
        start = dot + 1;
    }
    if (parts != 4) return std::nullopt;
    return ip;
}

/// The mask for a prefix length: /24 -> 0xFFFFFF00, /0 -> 0.
inline std::uint32_t maskOf(int prefix) {
    return prefix == 0 ? 0u : 0xFFFFFFFFu << (32 - prefix);
}

struct Cidr {
    std::uint32_t network = 0;
    int prefix = 0;

    /// True if ip is inside this network.
    bool contains(std::uint32_t ip) const {
        return (ip & maskOf(prefix)) == network;
    }

    /// True if every address in `other` is also in this network.
    bool covers(const Cidr& other) const {
        return prefix <= other.prefix && contains(other.network);
    }
};

/// "10.0.0.0/8" -> {0x0A000000, 8}. A plain address means /32, "any" means 0.0.0.0/0.
/// Host bits are cleared: "10.1.2.3/8" -> 10.0.0.0/8. Bad text or a prefix outside 0-32 -> nullopt.
inline std::optional<Cidr> parseCidr(const std::string& text) {
    if (text == "any") return Cidr{0, 0};
    std::size_t slash = text.find('/');
    auto ip = parseIpv4(text.substr(0, slash));
    if (!ip) return std::nullopt;
    int prefix = 32;
    if (slash != std::string::npos) {
        std::string p = text.substr(slash + 1);
        if (p.empty() || p.size() > 2 || p.find_first_not_of("0123456789") != std::string::npos) return std::nullopt;
        prefix = std::stoi(p);
        if (prefix > 32) return std::nullopt;
    }
    return Cidr{*ip & maskOf(prefix), prefix};
}

struct Rule {
    std::string action;   // "allow" or "deny"
    std::string proto;    // "tcp", "udp", "icmp" or "any"
    Cidr src, dst;
    int portLow = 0, portHigh = 65535;
    int line = 0;         // where it came from, for messages
};

/// Parse "allow tcp 10.0.0.0/8 any 443". The port is a number, a range "1000-2000" or "any".
/// Errors throw std::invalid_argument("line <n>: <what>") with these whats:
/// "expected 5 fields", "bad action <x>", "bad protocol <x>", "bad address <x>", "bad port <x>".
inline Rule parseRule(const std::string& text, int line) {
    std::istringstream in(text);
    std::vector<std::string> f;
    for (std::string w; in >> w;) f.push_back(w);
    auto fail = [line](const std::string& what) { return std::invalid_argument("line " + std::to_string(line) + ": " + what); };
    if (f.size() != 5) throw fail("expected 5 fields");

    Rule r;
    r.line = line;
    r.action = f[0];
    if (r.action != "allow" && r.action != "deny") throw fail("bad action " + f[0]);
    r.proto = f[1];
    if (r.proto != "tcp" && r.proto != "udp" && r.proto != "icmp" && r.proto != "any") throw fail("bad protocol " + f[1]);
    auto src = parseCidr(f[2]);
    if (!src) throw fail("bad address " + f[2]);
    auto dst = parseCidr(f[3]);
    if (!dst) throw fail("bad address " + f[3]);
    r.src = *src;
    r.dst = *dst;

    if (f[4] != "any") {
        auto number = [&](const std::string& s) {
            if (s.empty() || s.size() > 5 || s.find_first_not_of("0123456789") != std::string::npos) throw fail("bad port " + f[4]);
            int n = std::stoi(s);
            if (n < 1 || n > 65535) throw fail("bad port " + f[4]);
            return n;
        };
        std::size_t dash = f[4].find('-');
        r.portLow = number(f[4].substr(0, dash));
        r.portHigh = dash == std::string::npos ? r.portLow : number(f[4].substr(dash + 1));
        if (r.portLow > r.portHigh) throw fail("bad port " + f[4]);
    }
    return r;
}

struct Packet {
    std::string proto;
    std::uint32_t src = 0, dst = 0;
    int port = 0;   // destination port (0 for icmp)
};

/// True if the rule matches the packet: same protocol (or rule proto "any"), both addresses
/// inside the rule's networks, and the port in range. For icmp packets the port is ignored.
inline bool matches(const Rule& r, const Packet& p) {
    if (r.proto != "any" && r.proto != p.proto) return false;
    if (!r.src.contains(p.src) || !r.dst.contains(p.dst)) return false;
    return p.proto == "icmp" || (p.port >= r.portLow && p.port <= r.portHigh);
}

struct Verdict {
    std::string action;
    int ruleLine = 0;   // 0 = no rule matched: the implicit "deny all" at the end
};

/// First match wins, like a router ACL. No match -> {"deny", 0}.
inline Verdict evaluate(const std::vector<Rule>& rules, const Packet& p) {
    for (const auto& r : rules)
        if (matches(r, p)) return {r.action, r.line};
    return {"deny", 0};
}

/// Rules that can never match because an earlier rule matches everything they would:
/// the earlier rule has the same protocol or "any", covers both networks, and covers the port range.
/// One message per shadowed rule, in order: "line 7 is shadowed by line 3".
inline std::vector<std::string> shadowedRules(const std::vector<Rule>& rules) {
    std::vector<std::string> out;
    for (std::size_t j = 0; j < rules.size(); ++j) {
        const Rule& later = rules[j];
        for (std::size_t i = 0; i < j; ++i) {
            const Rule& earlier = rules[i];
            bool proto = earlier.proto == "any" || earlier.proto == later.proto;
            bool ports = earlier.portLow <= later.portLow && earlier.portHigh >= later.portHigh;
            if (proto && ports && earlier.src.covers(later.src) && earlier.dst.covers(later.dst)) {
                out.push_back("line " + std::to_string(later.line) + " is shadowed by line " + std::to_string(earlier.line));
                break;
            }
        }
    }
    return out;
}

}  // namespace fw
@@file tests.cpp
// tests.cpp: a tiny test runner, so the project needs nothing installed.
#include <functional>
#include <iostream>
#include <string>

#include "firewall.hpp"

using namespace fw;

static int passed = 0, failed = 0;

static void check(const std::string& name, const std::function<bool()>& test) {
    try {
        if (test()) {
            ++passed;
            std::cout << "  ok    " << name << '\n';
            return;
        }
        std::cout << "  FAIL  " << name << '\n';
    } catch (const std::exception& e) {
        if (std::string(e.what()).starts_with("TODO")) {
            std::cout << "  TODO  " << name << '\n';
            ++failed;
            return;
        }
        std::cout << "  FAIL  " << name << ": exception: " << e.what() << '\n';
    }
    ++failed;
}

static bool throwsWith(const std::string& rule, const std::string& message) {
    try {
        parseRule(rule, 4);
    } catch (const std::invalid_argument& e) {
        return e.what() == message;
    }
    return false;
}

static Packet pkt(const std::string& proto, const std::string& src, const std::string& dst, int port) {
    return {proto, *parseIpv4(src), *parseIpv4(dst), port};
}

int main() {
    std::cout << "parseIpv4\n";
    check("parses a dotted quad", [] { return parseIpv4("192.168.1.10") == 3232235786u; });
    check("0.0.0.0 and 255.255.255.255", [] { return parseIpv4("0.0.0.0") == 0u && parseIpv4("255.255.255.255") == 0xFFFFFFFFu; });
    check("rejects bad addresses", [] {
        for (std::string bad : {"1.2.3", "256.1.1.1", "a.b.c.d", "1.2.3.4.5", "", "1..2.3", "1.2.3.-4"})
            if (parseIpv4(bad)) return false;
        return true;
    });

    std::cout << "Cidr\n";
    check("parses a network", [] { auto c = parseCidr("10.0.0.0/8"); return c && c->network == 0x0A000000u && c->prefix == 8; });
    check("plain address is /32, any is /0", [] { return parseCidr("10.1.1.1")->prefix == 32 && parseCidr("any")->prefix == 0; });
    check("clears host bits", [] { return parseCidr("10.1.2.3/8")->network == 0x0A000000u; });
    check("rejects bad prefixes", [] { return !parseCidr("10.0.0.0/33") && !parseCidr("10.0.0.0/") && !parseCidr("10.0.0.0/x"); });
    check("contains", [] {
        auto net = *parseCidr("192.168.1.0/26");
        return net.contains(*parseIpv4("192.168.1.63")) && !net.contains(*parseIpv4("192.168.1.64"));
    });
    check("any contains everything", [] { return parseCidr("any")->contains(*parseIpv4("8.8.8.8")); });
    check("covers", [] { return parseCidr("10.0.0.0/8")->covers(*parseCidr("10.20.0.0/16")) && !parseCidr("10.20.0.0/16")->covers(*parseCidr("10.0.0.0/8")); });

    std::cout << "parseRule\n";
    check("parses a rule with one port", [] {
        Rule r = parseRule("allow tcp 10.0.0.0/8 any 443", 3);
        return r.action == "allow" && r.proto == "tcp" && r.src.prefix == 8 && r.dst.prefix == 0 && r.portLow == 443 && r.portHigh == 443 && r.line == 3;
    });
    check("port ranges and any", [] {
        Rule a = parseRule("deny udp any any 1000-2000", 1), b = parseRule("deny icmp any any any", 2);
        return a.portLow == 1000 && a.portHigh == 2000 && b.portLow == 0 && b.portHigh == 65535;
    });
    check("error messages", [] {
        return throwsWith("allow tcp any any", "line 4: expected 5 fields") &&
               throwsWith("permit tcp any any 80", "line 4: bad action permit") &&
               throwsWith("allow sctp any any 80", "line 4: bad protocol sctp") &&
               throwsWith("allow tcp 10.0.0.0/40 any 80", "line 4: bad address 10.0.0.0/40") &&
               throwsWith("allow tcp any any 70000", "line 4: bad port 70000") &&
               throwsWith("allow tcp any any 90-80", "line 4: bad port 90-80");
    });

    std::cout << "evaluate\n";
    std::vector<Rule> rules = {
        parseRule("allow tcp 10.0.0.0/8 192.168.1.10 443", 1),
        parseRule("deny tcp 10.66.0.0/16 any any", 2),
        parseRule("allow udp any 192.168.1.53 53", 3),
        parseRule("allow icmp 10.0.0.0/8 any any", 4),
    };
    check("first match wins", [&] { auto v = evaluate(rules, pkt("tcp", "10.66.1.1", "192.168.1.10", 443)); return v.action == "allow" && v.ruleLine == 1; });
    check("later rule matches", [&] { auto v = evaluate(rules, pkt("tcp", "10.66.1.1", "192.168.1.11", 22)); return v.action == "deny" && v.ruleLine == 2; });
    check("protocol must match", [&] { return evaluate(rules, pkt("udp", "10.1.1.1", "192.168.1.10", 443)).ruleLine == 0; });
    check("icmp ignores ports", [&] { return evaluate(rules, pkt("icmp", "10.9.9.9", "8.8.8.8", 0)).ruleLine == 4; });
    check("implicit deny", [&] { auto v = evaluate(rules, pkt("tcp", "172.16.0.1", "192.168.1.10", 443)); return v.action == "deny" && v.ruleLine == 0; });

    std::cout << "shadowedRules\n";
    check("finds shadowed rules", [] {
        std::vector<Rule> r = {
            parseRule("allow tcp 10.0.0.0/8 any 1-1024", 1),
            parseRule("deny tcp 10.20.0.0/16 any 22", 2),        // shadowed by 1
            parseRule("deny any any any any", 3),
            parseRule("allow udp 10.0.0.0/8 any 53", 4),         // shadowed by 3
        };
        auto s = shadowedRules(r);
        return s.size() == 2 && s[0] == "line 2 is shadowed by line 1" && s[1] == "line 4 is shadowed by line 3";
    });
    check("partial overlap isn't shadowing", [] {
        std::vector<Rule> r = {parseRule("allow tcp 10.0.0.0/16 any 80", 1), parseRule("deny tcp 10.0.0.0/8 any 80", 2)};
        return shadowedRules(r).empty();
    });

    std::cout << '\n' << passed << " passed, " << failed << " failed\n";
    return failed == 0 ? 0 : 1;
}
@@file main.cpp
// main.cpp: check traffic against a rule file.  Usage: main [rules.txt] [traffic.txt]
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

#include "firewall.hpp"

static std::vector<fw::Rule> loadRules(const std::string& path) {
    std::ifstream in(path);
    if (!in) throw std::runtime_error("can't open " + path);
    std::vector<fw::Rule> rules;
    std::string line;
    for (int n = 1; std::getline(in, line); ++n) {
        auto hash = line.find('#');
        if (hash != std::string::npos) line.erase(hash);          // comments
        if (line.find_first_not_of(" \t\r") == std::string::npos) continue;
        rules.push_back(fw::parseRule(line, n));
    }
    return rules;
}

int main(int argc, char** argv) {
    std::string rulesPath = argc > 1 ? argv[1] : "rules.txt";
    std::string trafficPath = argc > 2 ? argv[2] : "traffic.txt";
    try {
        auto rules = loadRules(rulesPath);
        std::cout << rules.size() << " rules loaded from " << rulesPath << '\n';
        for (const auto& warning : fw::shadowedRules(rules)) std::cout << "  warning: " << warning << '\n';

        std::ifstream traffic(trafficPath);
        if (!traffic) throw std::runtime_error("can't open " + trafficPath);
        std::map<std::string, int> totals;
        std::map<int, int> hitsPerRule;
        std::string proto, src, dst;
        int port = 0;
        std::cout << '\n';
        while (traffic >> proto >> src >> dst >> port) {
            auto s = fw::parseIpv4(src), d = fw::parseIpv4(dst);
            if (!s || !d) {
                std::cout << "skipping bad packet " << src << " -> " << dst << '\n';
                continue;
            }
            auto v = fw::evaluate(rules, {proto, *s, *d, port});
            totals[v.action]++;
            hitsPerRule[v.ruleLine]++;
            std::cout << std::left << std::setw(6) << v.action << std::setw(5) << proto << std::setw(16) << src << "-> "
                      << std::setw(16) << dst << std::setw(6) << port
                      << (v.ruleLine ? "line " + std::to_string(v.ruleLine) : std::string("default deny")) << '\n';
        }
        std::cout << "\nallowed " << totals["allow"] << ", denied " << totals["deny"] << '\n';
        for (const auto& r : rules)
            if (!hitsPerRule.contains(r.line)) std::cout << "  line " << r.line << " was never used\n";
    } catch (const std::exception& e) {
        std::cerr << "error: " << e.what() << '\n';
        return 1;
    }
}
@@file rules.txt
# Office firewall: first match wins, everything else is denied.
# action  proto  source          destination      port
allow     tcp    10.10.0.0/16    192.168.1.10     443          # staff -> intranet (HTTPS)
allow     tcp    10.10.50.0/24   192.168.1.20     22           # IT team -> jump host (SSH)
deny      tcp    10.10.0.0/16    192.168.1.20     22           # everyone else: no SSH
allow     udp    any             192.168.1.53     53           # DNS
allow     tcp    10.10.0.0/16    any              80-443       # staff -> internet
allow     tcp    10.10.7.0/24    192.168.1.10     443          # guests -> intranet?? (shadowed)
allow     icmp   10.10.50.0/24   any              any          # IT may ping anything
deny      tcp    any             192.168.1.30     3389         # no RDP to the file server
@@file traffic.txt
tcp  10.10.3.41   192.168.1.10   443
tcp  10.10.50.7   192.168.1.20   22
tcp  10.10.3.41   192.168.1.20   22
udp  10.10.3.41   192.168.1.53   53
tcp  10.10.3.41   142.250.4.100  443
tcp  10.10.3.41   142.250.4.100  8080
icmp 10.10.50.7   8.8.8.8        0
icmp 10.10.3.41   8.8.8.8        0
tcp  172.16.9.9   192.168.1.10   443
tcp  10.10.3.41   192.168.1.30   3389

@@drill
In a first-match firewall, which rule decides a packet's fate?
= the first matching rule
= first match
= first matching rule
= the first match
? Order rules from specific to general.

@@text
🎉 You've built the core of a packet filter. In an interview, "I wrote a first-match ACL engine with shadowed-rule detection in C++" shows both programming and networking skills, which is exactly the combination infrastructure teams look for.
