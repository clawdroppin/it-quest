@@lesson fe3-technology テクノロジ系: Systems, Databases, Networks, Security
@@lang python
@@text
The rest of テクノロジ系 overlaps with what you learned in the A+, Network+, Security+ and SQL tracks. Here are the parts the FE exam calculates with, and the Japanese terms.

## Computer systems (コンピュータシステム)

- **CPU performance**: clock frequency (クロック周波数), **CPI** (clocks per instruction), **MIPS** (million instructions per second). MIPS = clock (MHz) ÷ CPI. Pipelining (パイプライン), superscalar, multicore.
- **Cache** (キャッシュメモリ): effective access time = hit rate × cache time + (1 − hit rate) × memory time. With 90% hits, 10 ns cache, 60 ns memory: 0.9×10 + 0.1×60 = **15 ns**.
- **Memory**: DRAM (needs refresh, main memory), SRAM (fast, cache), ROM, flash. Virtual memory (仮想記憶): paging (ページング), page faults, LRU / FIFO page replacement.
- **Availability (稼働率)** = MTBF ÷ (MTBF + MTTR).
  - **Series** (直列): A × B (both must work). 0.9 × 0.9 = 0.81.
  - **Parallel** (並列): 1 − (1 − A)(1 − B) (either works). 1 − 0.1 × 0.1 = 0.99.
- **RAID**: 0 (striping), 1 (mirroring), 5 (striping + parity, loses 1 disk of capacity), 6 (two parities).
- **Fail-safe** (フェールセーフ: fail into a safe state, e.g. traffic lights go red), **fail-soft** (フェールソフト: keep running with reduced function), **fool-proof** (フールプルーフ: prevent user mistakes), **fault tolerance** (フォールトトレランス).

## Databases (データベース)

- **Normalization** (正規化): **1NF** no repeating groups; **2NF** no partial dependency on part of a composite key; **3NF** no **transitive** dependency (non-key → non-key).
- Keys: primary key (主キー), foreign key (外部キー), candidate key.
- **Transactions** and **ACID**: atomicity (原子性), consistency (一貫性), isolation (独立性), durability (永続性).
- **Exclusive control** (排他制御): locks (shared/exclusive: 共有ロック/専有ロック), **deadlock** (デッドロック).
- **Recovery**: roll forward (ロールフォワード: redo from the log after a media failure, using the backup + after-images), roll back (ロールバック: undo uncommitted work using before-images), checkpoints.
- SQL: SELECT / JOIN / GROUP BY / HAVING / subqueries, exactly as in the SQL track.

## Networks (ネットワーク)

- OSI and TCP/IP layers, IP addressing, subnets: the Network+ material.
- **Transfer time** = data size ÷ (line speed × efficiency). 100 MB over 100 Mbps at 80% efficiency: 800 Mbit ÷ 80 Mbps = **10 s**. Watch bytes vs bits (×8)!
- Protocols in Japanese: DNS (名前解決), DHCP, NAT/NAPT, SMTP/POP3/IMAP, HTTP/HTTPS, **NTP** (時刻同期).

## Security (セキュリティ): also the 4 questions in 科目B

- **CIA**: 機密性 (confidentiality), 完全性 (integrity), 可用性 (availability).
- Crypto: **共通鍵暗号** (symmetric: AES), **公開鍵暗号** (asymmetric: RSA), **ハッシュ** (SHA-256), **デジタル署名** (sign with the sender's private key, verify with their public key), **PKI** and **認証局 (CA)**.
- Attacks: SQLインジェクション, クロスサイトスクリプティング (XSS), CSRF, DoS/DDoS, フィッシング, ランサムウェア, 標的型攻撃, ソーシャルエンジニアリング, 水飲み場型攻撃 (watering hole), ゼロデイ攻撃, ブルートフォース, パスワードリスト攻撃 (credential stuffing).
- Defences: ファイアウォール, WAF, IDS/IPS, DMZ, VPN, 多要素認証 (MFA), 最小権限 (least privilege), ログ監視, **CSIRT**, **ISMS** (ISO/IEC 27001).
- 科目B security questions are usually **scenarios**: an employee does X, which response is appropriate? Think: contain, report to the right team, preserve evidence, follow the policy.

@@exercise Availability calculator
Write `series(*rates)` and `parallel(*rates)` that return the availability of components in series (all must work) and in parallel (any one is enough), rounded to 4 decimals. Print: two 0.9 servers in series; two 0.9 servers in parallel; and a 0.99 router in series with a parallel pair of 0.9 servers.
@@starter
def series(*rates):
    # multiply all the rates
    return 0


def parallel(*rates):
    # 1 - (product of all the failure rates (1 - r))
    return 0


print(series(0.9, 0.9))
print(parallel(0.9, 0.9))
print(series(0.99, parallel(0.9, 0.9)))
@@expected
0.81
0.99
0.9801
@@hint
Start with `result = 1` and multiply in a loop. For parallel, multiply the `(1 - r)` values and return `round(1 - product, 4)`.
@@solution
def series(*rates):
    result = 1
    for r in rates:
        result *= r
    return round(result, 4)


def parallel(*rates):
    failure = 1
    for r in rates:
        failure *= 1 - r
    return round(1 - failure, 4)


print(series(0.9, 0.9))
print(parallel(0.9, 0.9))
print(series(0.99, parallel(0.9, 0.9)))

@@quiz
A system has MTBF 450 hours and MTTR 50 hours. What is its availability (稼働率)?
* 0.9
- 0.1
- 0.89
- 9.0
? 450 ÷ (450 + 50).

@@quiz
Cache hit rate 95%, cache access 10 ns, main memory 100 ns. What's the effective access time?
* 14.5 ns
- 10 ns
- 55 ns
- 95 ns
? 0.95 × 10 + 0.05 × 100 = 9.5 + 5.

@@quiz
A table has the non-key column 部署名 (department name) that depends on 部署コード, which itself is a non-key column. Which normal form does it violate?
* Third normal form (transitive dependency)
- First normal form
- Second normal form
- It's fully normalized
? Move 部署コード → 部署名 into its own table.

@@quiz
After a disk failure, the database is restored from the last backup and then brought up to date. What is this called?
* Roll forward, using the log's after-images
- Roll back, using before-images
- Normalization
- A checkpoint only
? Roll back undoes incomplete transactions; roll forward redoes committed ones.

@@quiz
A 600 MB file is sent over a 100 Mbps line with 80% transmission efficiency. About how long does it take?
* 60 seconds
- 6 seconds
- 7.5 seconds
- 48 seconds
? 600 MB × 8 = 4,800 Mbit ÷ 80 Mbps = 60 s.

@@quiz
To create a digital signature, the sender encrypts the hash of the message with...
* The sender's private key
- The receiver's public key
- The sender's public key
- A shared symmetric key
? Anyone can verify it with the sender's public key.

@@quiz
A design where a machine stops in a safe state when it fails (e.g. a train brakes) is called...
* Fail-safe (フェールセーフ)
- Fail-soft (フェールソフト)
- Fool-proof (フールプルーフ)
- Load balancing
? Fail-soft keeps running with reduced function instead.

@@quiz
Which RAID level stripes data with distributed parity and survives one disk failure?
* RAID 5
- RAID 0
- RAID 1
- RAID 0+1 only
? Capacity of one disk goes to parity.

@@flashcards
稼働率 :: Availability = MTBF / (MTBF + MTTR)
直列 / 並列 :: Series (A×B) / parallel (1 − (1−A)(1−B))
MIPS :: Million instructions per second
キャッシュのヒット率 :: Cache hit rate
仮想記憶 / ページング :: Virtual memory / paging
フェールセーフ :: Fail into a safe state
フェールソフト :: Keep running with reduced function
フールプルーフ :: Prevent user mistakes
フォールトトレランス :: Fault tolerance
正規化 (第1/第2/第3正規形) :: Normalization (1NF/2NF/3NF)
主キー / 外部キー :: Primary key / foreign key
排他制御 :: Concurrency control (locking)
デッドロック :: Deadlock
ロールフォワード / ロールバック :: Redo with after-images / undo with before-images
機密性 / 完全性 / 可用性 :: Confidentiality / integrity / availability
共通鍵暗号 / 公開鍵暗号 :: Symmetric / asymmetric encryption
デジタル署名 :: Digital signature
認証局 (CA) :: Certificate authority
標的型攻撃 :: Targeted attack
ISMS :: Information security management system (ISO/IEC 27001)

@@drill
Two components each with availability 0.8 are connected in parallel. What's the total availability? (decimal)
= 0.96
? 1 − 0.2 × 0.2.
