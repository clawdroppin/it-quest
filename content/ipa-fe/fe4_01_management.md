@@lesson fe4-management マネジメント系・ストラテジ系: Projects, Business and Law
@@text
About a quarter of 科目A is management and strategy. The questions are easier than they look once you know the vocabulary, and the laws matter for your actual job in Japan.

## Project management (プロジェクトマネジメント)

- **WBS** (Work Breakdown Structure): the work split into manageable pieces.
- **Arrow diagrams** (アローダイアグラム) and the **critical path** (クリティカルパス): the longest path through the network; any delay on it delays the whole project. **Float** (余裕日数) is how much a non-critical task can slip.
- **EVM** (Earned Value Management): PV (planned value), EV (earned value), AC (actual cost). **SV** = EV − PV (schedule), **CV** = EV − AC (cost); negative means behind schedule / over budget.
- **Crashing** (クラッシング: add resources to critical tasks) vs **fast tracking** (ファストトラッキング: do tasks in parallel).
- Function points (ファンクションポイント法) estimate size from the functions users see.
- Development models: **waterfall** (ウォーターフォール), **agile** (アジャイル: Scrum sprints, XP: pair programming, TDD), prototyping, spiral, DevOps. Tests: unit (単体テスト) → integration (結合テスト) → system (システムテスト) → acceptance (受入れテスト). White-box vs black-box; **boundary value analysis** (限界値分析), **equivalence partitioning** (同値分割).

Japanese SIers often still run **waterfall** with detailed design documents (基本設計書, 詳細設計書) and test specifications. Know the phases: 要件定義 → 基本設計 → 詳細設計 → 製造 (coding) → 単体テスト → 結合テスト → 総合テスト → 運用.

## Service management (サービスマネジメント)

ITIL in Japanese: インシデント管理, 問題管理, 変更管理, SLA, サービスデスク. Also **system audit** (システム監査): an independent auditor checks controls and reports to management.

## Strategy (ストラテジ系)

- Analysis: **SWOT** (strengths, weaknesses, opportunities, threats), **PPM** (product portfolio: star 花形, cash cow 金のなる木, question mark 問題児, dog 負け犬), **3C** (customer, competitor, company), **5 forces**, **BSC** (balanced scorecard: financial, customer, internal process, learning & growth), **KGI/KPI**.
- Marketing: **4P** (product, price, place, promotion), segmentation, positioning.
- Business: **ERP**, **CRM**, **SCM**, **BPR**, **RPA**, **SaaS/PaaS/IaaS**, **DX** (デジタルトランスフォーメーション), **IoT**, **AI**.
- Accounting (会計): **break-even point** (損益分岐点) = fixed costs ÷ (1 − variable cost ratio). Fixed costs ¥2,000,000, variable cost ratio 0.6 → break-even sales **¥5,000,000**. Balance sheet (貸借対照表), P&L (損益計算書), depreciation (減価償却).

## Laws you'll actually meet at work (法務)

| Law | What it covers |
|---|---|
| **著作権法** (Copyright Act) | software is protected automatically; programs made at work usually belong to the employer (職務著作) |
| **産業財産権** | patents (特許権), utility models, designs, trademarks (商標権) |
| **不正競争防止法** | trade secrets (営業秘密), domain-name squatting |
| **不正アクセス禁止法** | unauthorized access: using someone else's ID/password, attacking security holes |
| **個人情報保護法** | personal information: purpose of use, consent for third parties, security measures, breach reporting |
| **労働基準法** | working hours, overtime (36協定), paid leave |
| **労働者派遣法** | dispatch work (派遣): the client company directs the worker |
| **請負契約 / 準委任契約** | contract for a deliverable / for doing work (e.g. SES); **the client must NOT give direct orders to workers under 請負 or 準委任**, or it becomes illegal disguised dispatch (偽装請負) |
| **下請法** | protects subcontractors from unfair treatment by bigger companies |

The last two rows matter if you join an **SES** (System Engineering Service) company: you may work at a client's office under a 準委任 contract, and who may give you instructions is a legal question, not just a polite one.

@@quiz
Which path determines the shortest possible duration of a project?
* The critical path (the longest path through the network)
- The shortest path
- The path with the most tasks
- The path with the cheapest tasks
? Any delay on it delays the whole project.

@@quiz
EV = ¥800,000, PV = ¥1,000,000, AC = ¥900,000. What's the situation?
* Behind schedule and over budget
- Ahead of schedule and under budget
- On schedule, over budget
- Behind schedule, under budget
? SV = EV − PV = −200,000 (behind); CV = EV − AC = −100,000 (over budget).

@@quiz
Fixed costs are ¥3,000,000 and the variable cost ratio is 0.4. What are break-even sales?
* ¥5,000,000
- ¥7,500,000
- ¥1,200,000
- ¥4,200,000
? 3,000,000 ÷ (1 − 0.4).

@@quiz
In PPM, a product with high market share in a low-growth market is a...
* Cash cow (金のなる木)
- Star (花形)
- Question mark (問題児)
- Dog (負け犬)
? It generates cash to invest in stars and question marks.

@@quiz
A test that checks values at the edges of valid ranges (e.g. 0, 1, 99, 100 for 1-99) is...
* Boundary value analysis (限界値分析)
- Equivalence partitioning only
- White-box testing
- A regression test
? Bugs cluster at boundaries.

@@quiz
An engineer works at a client's office under a 準委任 (quasi-mandate) contract. The client's manager gives them daily direct orders. What's the problem?
* It may be disguised dispatch (偽装請負), which is illegal; instructions should go through the engineer's own company
- Nothing, clients can always direct contractors
- It violates the Copyright Act
- It's only a problem on weekends
? Direct command is only allowed under a dispatch (派遣) contract.

@@quiz
Which law is violated by logging in with a colleague's password without permission?
* 不正アクセス禁止法 (Unauthorized Computer Access Act)
- 著作権法
- 下請法
- 労働基準法
? Even if no damage is done.

@@quiz
In the balanced scorecard, which is NOT one of the four perspectives?
* Competitor perspective
- Financial perspective
- Customer perspective
- Learning and growth perspective
? The fourth is the internal business process perspective.

@@flashcards
WBS :: Work breakdown structure
クリティカルパス :: Critical path: the longest path
EVM: PV / EV / AC :: Planned value / earned value / actual cost
SV / CV :: EV − PV / EV − AC
ファストトラッキング / クラッシング :: Do in parallel / add resources
単体 / 結合 / システム / 受入れテスト :: Unit / integration / system / acceptance test
限界値分析 / 同値分割 :: Boundary values / equivalence partitioning
要件定義 → 基本設計 → 詳細設計 :: Requirements → basic design → detailed design
SWOT :: Strengths, weaknesses, opportunities, threats
PPM :: 花形, 金のなる木, 問題児, 負け犬
BSC :: Financial, customer, internal process, learning & growth
KGI / KPI :: Goal indicator / performance indicator
損益分岐点 :: Break-even = fixed costs ÷ (1 − variable cost ratio)
減価償却 :: Depreciation
著作権法 :: Copyright Act
不正アクセス禁止法 :: Unauthorized Computer Access Act
個人情報保護法 :: Personal Information Protection Act
労働者派遣法 :: Worker Dispatch Act
請負 / 準委任 :: Contract for work (deliverable) / quasi-mandate (service, e.g. SES)
偽装請負 :: Disguised dispatch: illegal direct command of contractors
36協定 :: Overtime agreement under the Labor Standards Act

@@drill
What does CV stand for in EVM? (two words)
= cost variance
? CV = EV − AC.

@@exercise Critical path on an arrow diagram (アローダイアグラム)
In an arrow diagram, activities are arrows between numbered nodes (結合点). Compute each node's earliest time (最早結合点時刻, the maximum over incoming arrows) and latest time (最遅結合点時刻, the minimum over outgoing arrows, working backwards from the end), each activity's total float, and the critical path (クリティカルパス). Dummy activities (ダミー) have duration 0.
@@lang python
@@starter
activities = [
    ("A", 1, 2, 3), ("B", 1, 3, 5), ("C", 2, 4, 4), ("D", 3, 4, 2),
    ("E", 3, 5, 6), ("dummy", 4, 5, 0), ("F", 4, 6, 3), ("G", 5, 6, 4),
]
nodes = sorted({n for _, a, b, _ in activities for n in (a, b)})

def earliest(activities, nodes):
    raise NotImplementedError

def latest(activities, nodes, end_time):
    raise NotImplementedError

es = earliest(activities, nodes)
lt = latest(activities, nodes, es[nodes[-1]])
print("node earliest latest")
for n in nodes:
    print(f"{n:>4} {es[n]:>8} {lt[n]:>6}")
critical = []
for name, a, b, d in activities:
    total_float = lt[b] - es[a] - d
    if total_float == 0 and name != "dummy":
        critical.append(name)
    print(f"{name:<6} {a}->{b} duration {d}  total float {total_float}")
print("critical path:", " -> ".join(critical), f"({es[nodes[-1]]} days)")
@@expected
node earliest latest
   1        0      0
   2        3      7
   3        5      5
   4        7     11
   5       11     11
   6       15     15
A      1->2 duration 3  total float 4
B      1->3 duration 5  total float 0
C      2->4 duration 4  total float 4
D      3->4 duration 2  total float 4
E      3->5 duration 6  total float 0
dummy  4->5 duration 0  total float 4
F      4->6 duration 3  total float 5
G      5->6 duration 4  total float 0
critical path: B -> E -> G (15 days)
@@requires
def earliest
def latest
max(
@@hint
Nodes are numbered in order, so process them ascending for the earliest times: `es[b] = max(es[b], es[a] + d)` for each arrow into b (start with every node at 0). For the latest times, start every node at `end_time` and go through the arrows from the highest start node down: `lt[a] = min(lt[a], lt[b] - d)`. Total float = `lt[b] - es[a] - d`.
@@solution
activities = [
    ("A", 1, 2, 3), ("B", 1, 3, 5), ("C", 2, 4, 4), ("D", 3, 4, 2),
    ("E", 3, 5, 6), ("dummy", 4, 5, 0), ("F", 4, 6, 3), ("G", 5, 6, 4),
]
nodes = sorted({n for _, a, b, _ in activities for n in (a, b)})

def earliest(activities, nodes):
    es = {n: 0 for n in nodes}
    for _, a, b, d in sorted(activities, key=lambda x: x[1]):
        es[b] = max(es[b], es[a] + d)
    return es

def latest(activities, nodes, end_time):
    lt = {n: end_time for n in nodes}
    for _, a, b, d in sorted(activities, key=lambda x: -x[2]):
        lt[a] = min(lt[a], lt[b] - d)
    return lt

es = earliest(activities, nodes)
lt = latest(activities, nodes, es[nodes[-1]])
print("node earliest latest")
for n in nodes:
    print(f"{n:>4} {es[n]:>8} {lt[n]:>6}")
critical = []
for name, a, b, d in activities:
    total_float = lt[b] - es[a] - d
    if total_float == 0 and name != "dummy":
        critical.append(name)
    print(f"{name:<6} {a}->{b} duration {d}  total float {total_float}")
print("critical path:", " -> ".join(critical), f"({es[nodes[-1]]} days)")
