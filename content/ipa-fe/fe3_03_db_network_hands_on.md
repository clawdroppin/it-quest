@@lesson fe3-db-network-hands-on テクノロジ系 Hands-on: 正規化, SQL Results and Subnetting
@@lang python
@@text
Two technology topics produce reliable 科目A points once you can compute them instead of guessing: **database normalisation** (正規化) together with predicting SQL results, and **IPv4 subnetting** (サブネット). This lesson gives you the method and code that checks it.

## Functional dependencies and keys (関数従属・候補キー)

- `A → B`: knowing A determines B (社員番号 → 氏名).
- The **closure** (閉包) of a set of attributes is everything those attributes determine through the dependencies, applied repeatedly.
- A **candidate key** (候補キー) is a minimal set whose closure is every attribute. The primary key (主キー) is the one you choose.

## Normal forms (正規形)

| Form | Rule | Violation looks like |
|---|---|---|
| 1NF (第1正規形) | no repeating groups, atomic values | 商品1, 商品2, 商品3 columns |
| 2NF (第2正規形) | 1NF + no **partial dependency** (部分関数従属) of a non-key attribute on part of a composite key | (注文番号, 商品番号) → 数量, but 商品番号 → 商品名 |
| 3NF (第3正規形) | 2NF + no **transitive dependency** (推移的関数従属) through a non-key attribute | 社員番号 → 部署番号 → 部署名 |

The fix is always to split off the dependency into its own table, with the determinant as that table's key.

## SQL results the exam asks about

- `GROUP BY ... HAVING`: HAVING filters **groups**, WHERE filters **rows** before grouping.
- `COUNT(*)` counts rows, while `COUNT(col)` skips NULLs. `AVG` ignores NULLs too.
- `INNER JOIN` drops unmatched rows. `LEFT OUTER JOIN` keeps all left rows and fills the rest with NULL.

## Subnetting (サブネット)

For `192.168.10.77/26`:
- The mask is 26 ones: `255.255.255.192`. The block size is 2^(32−26) = 64.
- Network address: 77 rounded down to a multiple of 64 gives `192.168.10.64`. Broadcast: `192.168.10.127`.
- Usable hosts: 2^6 − 2 = **62** (the network and broadcast addresses can't be assigned).
- Two hosts are in the same subnet when `IP AND mask` is equal for both.

@@quiz
Table 受注明細(受注番号, 商品番号, 数量, 商品名) has key (受注番号, 商品番号). 商品番号 → 商品名. Which normal form does it violate?
- 1NF
* 2NF
- 3NF
- None
? 商品名 depends on part of the key: a partial dependency.

@@quiz
社員(社員番号, 氏名, 部署番号, 部署名) with 社員番号 → 部署番号 → 部署名. What is the problem?
- A partial dependency
* A transitive dependency (violates 3NF)
- A repeating group
- No primary key
? Split 部署 into its own table.

@@quiz
How many hosts can be assigned in a /27 subnet?
- 32
* 30
- 62
- 14
? 2⁵ − 2.

@@quiz
Which hosts share a subnet with 10.1.1.100/25?
* 10.1.1.20
- 10.1.1.130
- 10.1.2.100
- 10.1.0.100
? /25 splits .0–.127 and .128–.255.

@@quiz
A column has values 10, NULL, 20. What do COUNT(*) and AVG(col) return?
- 3 and 10
* 3 and 15
- 2 and 10
- 2 and 15
? AVG ignores NULL: (10 + 20) / 2.

@@flashcards
関数従属 A → B :: A determines B
閉包 :: everything a set of attributes determines
候補キー :: minimal set determining every attribute
第1正規形 :: atomic values, no repeating groups
第2正規形 :: no partial dependency on part of the key
第3正規形 :: no transitive dependency
部分関数従属 :: depends on part of a composite key
推移的関数従属 :: depends via a non-key attribute
HAVING :: filters groups
COUNT(col) :: skips NULL
/26 mask :: 255.255.255.192
usable hosts :: 2^(host bits) − 2
same subnet :: IP AND mask equal

@@drill
How many usable host addresses does a /28 subnet have?
= 14
? 2⁴ − 2.

@@drill
What is the network address of 172.16.5.200/24?
= 172.16.5.0
? The last octet is all host bits.

@@exercise Closures, candidate keys and normal forms
Compute attribute closures under a set of functional dependencies, find the candidate keys of a relation, then report partial dependencies (2NF violations) and transitive dependencies (3NF violations).
@@starter
from itertools import combinations

attrs = ["受注番号", "商品番号", "数量", "商品名", "単価", "顧客番号", "顧客名"]
fds = [
    ({"受注番号", "商品番号"}, {"数量"}),
    ({"商品番号"}, {"商品名", "単価"}),
    ({"受注番号"}, {"顧客番号"}),
    ({"顧客番号"}, {"顧客名"}),
]

def closure(start):
    raise NotImplementedError
@@expected
closure {商品番号}: ['商品番号', '商品名', '単価']
closure {受注番号}: ['受注番号', '顧客番号', '顧客名']
candidate keys: [['受注番号', '商品番号']]
non-key attributes: ['数量', '商品名', '単価', '顧客番号', '顧客名']
2NF violation: {'受注番号'} -> ['顧客番号', '顧客名'] (partial dependency)
2NF violation: {'商品番号'} -> ['商品名', '単価'] (partial dependency)
3NF violation: key -> 顧客番号 -> ['顧客名'] (transitive dependency)
@@requires
def closure
combinations
while 
@@hint
Closure: start with the set, and keep applying every dependency whose left side is included until nothing changes. Candidate keys: try subsets by size and keep those whose closure is every attribute, skipping supersets of keys you've already found. A partial dependency is a proper subset of a key that determines a non-key attribute. A transitive dependency is a key → non-key X → non-key Y.
@@solution
from itertools import combinations

attrs = ["受注番号", "商品番号", "数量", "商品名", "単価", "顧客番号", "顧客名"]
fds = [
    ({"受注番号", "商品番号"}, {"数量"}),
    ({"商品番号"}, {"商品名", "単価"}),
    ({"受注番号"}, {"顧客番号"}),
    ({"顧客番号"}, {"顧客名"}),
]

def closure(start):
    result = set(start)
    changed = True
    while changed:
        changed = False
        for left, right in fds:
            if left <= result and not right <= result:
                result |= right
                changed = True
    return result

print("closure {商品番号}:", sorted(closure({"商品番号"}), key=attrs.index))
print("closure {受注番号}:", sorted(closure({"受注番号"}), key=attrs.index))

keys = []
for size in range(1, len(attrs) + 1):
    for combo in combinations(attrs, size):
        s = set(combo)
        if closure(s) == set(attrs) and not any(k <= s for k in keys):
            keys.append(s)
print("candidate keys:", [sorted(k, key=attrs.index) for k in keys])

key = keys[0]
non_key = [a for a in attrs if a not in set().union(*keys)]
print("non-key attributes:", non_key)
for size in range(1, len(key)):
    for part in combinations(sorted(key, key=attrs.index), size):
        partial = [a for a in non_key if a in closure(set(part))]
        if partial:
            print(f"2NF violation: {set(part)} -> {partial} (partial dependency)")
for x in non_key:
    determined = [y for y in closure({x}) if y != x and y in non_key]
    if determined:
        print(f"3NF violation: key -> {x} -> {determined} (transitive dependency)")

@@exercise Subnet calculator
Implement the subnet maths with plain integer bit operations (no `ipaddress` module): parse dotted addresses, build the mask from the prefix, then compute the network address, broadcast address, usable host range and host count, and check whether pairs of hosts share a subnet.
@@starter
def to_int(dotted):
    raise NotImplementedError

def to_dotted(n):
    raise NotImplementedError

def subnet(cidr):
    raise NotImplementedError
@@expected
192.168.10.77/26   mask 255.255.255.192 network 192.168.10.64 broadcast 192.168.10.127 hosts 192.168.10.65-192.168.10.126 (62)
10.20.30.40/20     mask 255.255.240.0   network 10.20.16.0    broadcast 10.20.31.255   hosts 10.20.16.1-10.20.31.254 (4094)
172.16.5.200/24    mask 255.255.255.0   network 172.16.5.0    broadcast 172.16.5.255   hosts 172.16.5.1-172.16.5.254 (254)
192.168.1.9/30     mask 255.255.255.252 network 192.168.1.8   broadcast 192.168.1.11   hosts 192.168.1.9-192.168.1.10 (2)
10.1.1.100 and 10.1.1.20 /25: same subnet
10.1.1.100 and 10.1.1.130 /25: different subnets
192.168.0.10 and 192.168.3.250 /22: same subnet
@@requires
def to_int
def subnet
& mask
0xFFFFFFFF
@@hint
`to_int`: fold the octets with `n = n * 256 + int(octet)`. Mask: `(0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF`. Network = `ip & mask`, broadcast = `network | (~mask & 0xFFFFFFFF)`, and the usable range is network + 1 through broadcast − 1, giving 2^(32 − prefix) − 2 hosts.
@@solution
def to_int(dotted):
    n = 0
    for octet in dotted.split("."):
        n = n * 256 + int(octet)
    return n

def to_dotted(n):
    return ".".join(str((n >> shift) & 255) for shift in (24, 16, 8, 0))

def subnet(cidr):
    ip, prefix = cidr.split("/")
    prefix = int(prefix)
    mask = (0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF
    network = to_int(ip) & mask
    broadcast = network | (~mask & 0xFFFFFFFF)
    return mask, network, broadcast

for cidr in ("192.168.10.77/26", "10.20.30.40/20", "172.16.5.200/24", "192.168.1.9/30"):
    mask, network, broadcast = subnet(cidr)
    hosts = broadcast - network - 1
    print(f"{cidr:<18} mask {to_dotted(mask):<15} network {to_dotted(network):<13} broadcast {to_dotted(broadcast):<14}"
          f" hosts {to_dotted(network + 1)}-{to_dotted(broadcast - 1)} ({hosts})")

for a, b, prefix in (("10.1.1.100", "10.1.1.20", 25), ("10.1.1.100", "10.1.1.130", 25), ("192.168.0.10", "192.168.3.250", 22)):
    mask = (0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF
    same = to_int(a) & mask == to_int(b) & mask
    print(f"{a} and {b} /{prefix}: {'same subnet' if same else 'different subnets'}")
