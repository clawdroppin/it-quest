@@lesson fe4-strategy-calcs ストラテジ系 Calculations: Break-even, Inventory, Depreciation, ABC and Decisions
@@text
The ストラテジ系 (strategy) part of 科目A looks like business studies, but its points come mostly from a handful of calculations: 損益分岐点 (break-even), 在庫 (inventory) ordering, 減価償却 (depreciation), ABC分析 (Pareto analysis), 投資の評価 (investment appraisal) and decision-making under uncertainty. This lesson gives each formula with a worked example and the Japanese terms the questions use.

## 損益分岐点 (break-even point)

- **売上高** (sales) − **変動費** (variable costs) − **固定費** (fixed costs) = **利益** (profit).
- **変動費率** (variable cost ratio) = variable costs / sales.
- **損益分岐点売上高** = 固定費 / (1 − 変動費率).
- Example: sales ¥10,000k, variable ¥6,000k, fixed ¥3,000k → ratio 0.6 → break-even = 3,000 / 0.4 = **¥7,500k**.
- Target profit sales = (固定費 + 目標利益) / (1 − 変動費率).

## 在庫管理 (inventory)

- **発注点方式** (reorder point): order when stock falls to **発注点 = 1日の使用量 × 調達期間 (lead time) + 安全在庫 (safety stock)**.
- **定期発注方式** (periodic ordering): order every period; quantity = expected use over (period + lead time) + safety stock − current stock − stock on order.
- **経済的発注量 (EOQ)** = √(2 × annual demand × cost per order / holding cost per unit per year).

## 減価償却 (depreciation)

- **定額法** (straight-line): (取得価額 − 残存価額) / 耐用年数 each year. Under current Japanese tax rules the residual value is effectively ¥0 (down to ¥1 memorandum value), and the exam states what to use.
- **定率法** (declining balance): book value × rate each year (exam gives the rate).
- **除却損** (loss on disposal): book value remaining when an asset is scrapped.

## ABC分析 and パレート図

Sort items by value, compute cumulative share: **A** = top items up to ~70–80% of value (manage tightly), **B** = next up to ~90–95%, **C** = the rest. Drawn as a Pareto chart (bars + cumulative line).

## 投資の評価 (investment appraisal)

- **ROI** (投資利益率) = profit / investment × 100%.
- **回収期間法** (payback period): years until cumulative cash flow covers the investment.
- **NPV** (正味現在価値): Σ cash flow / (1 + r)ᵗ − investment.

## 意思決定 (decision making)

Under uncertainty with a payoff table:
- **マキシマックス原理** (maximax, optimistic): choose the option with the best best-case.
- **マキシミン原理** (maximin, pessimistic): choose the option with the best worst-case.
- **期待値原理** (expected value): with probabilities, choose the highest expected value.
- **ミニマックスリグレット**: minimise the maximum regret.

## Other frequently tested items

- **SWOT**, **PPM** (花形 star, 金のなる木 cash cow, 問題児 question mark, 負け犬 dog), **バランススコアカード** (financial, customer, internal process, learning and growth), **4P / 4C**, **ファンクションポイント法** (function points) for estimating.
- Law: 著作権 (copyright) arises automatically; 特許 needs registration; 不正競争防止法 protects 営業秘密 (trade secrets: 秘密管理性, 有用性, 非公知性); 個人情報保護法; 労働者派遣法 (dispatched workers take instructions from the client, contract (請負) workers don't).

@@quiz
Sales ¥20,000k, variable costs ¥12,000k, fixed costs ¥6,000k. What is the break-even sales amount?
- ¥12,000k
* ¥15,000k
- ¥18,000k
- ¥10,000k
? 6,000 / (1 − 0.6).

@@quiz
Daily use is 20 units, lead time 5 days, safety stock 30 units. What is the reorder point?
- 100
* 130
- 150
- 30
? 20 × 5 + 30.

@@quiz
Equipment costs ¥1,200k, residual value ¥0, useful life 4 years, straight-line. What is the annual depreciation?
* ¥300k
- ¥240k
- ¥400k
- ¥1,200k
? 1,200 / 4.

@@quiz
In PPM, a business with high market share in a low-growth market is called?
- 花形 (star)
* 金のなる木 (cash cow)
- 問題児 (question mark)
- 負け犬 (dog)
? It generates cash for other businesses.

@@quiz
Which decision principle chooses the option whose worst outcome is best?
- Maximax
* Maximin
- Expected value
- Laplace
? The pessimistic choice.

@@quiz
Which law protects trade secrets that are kept confidential, useful and not publicly known?
- 著作権法
- 特許法
* 不正競争防止法
- 個人情報保護法
? 営業秘密 needs all three conditions.

@@flashcards
利益 :: 売上 − 変動費 − 固定費
変動費率 :: 変動費 / 売上
損益分岐点売上高 :: 固定費 / (1 − 変動費率)
目標利益売上 :: (固定費 + 利益) / (1 − 変動費率)
発注点 :: daily use × lead time + safety stock
EOQ :: √(2DS / H)
定額法 :: (cost − residual) / life
定率法 :: book value × rate
ABC分析 :: A ≈ top 70–80% of value
ROI :: profit / investment
回収期間 :: years to recover investment
NPV :: discounted cash flows − investment
マキシマックス :: best best-case
マキシミン :: best worst-case
期待値原理 :: highest expected value
金のなる木 :: cash cow
問題児 :: question mark
花形 :: star
負け犬 :: dog
営業秘密 :: 秘密管理性, 有用性, 非公知性
派遣 :: client gives instructions
請負 :: contractor gives instructions

@@drill
What is the break-even sales if fixed costs are ¥400k and the variable cost ratio is 0.6?
= 1000
= ¥1,000k
= 1,000
= 1000k
= ¥1000k
? 400 / 0.4.

@@drill
What is the Japanese term for the pessimistic decision principle that maximises the minimum payoff?
= マキシミン原理
= マキシミン
= maximin
? マキシマックス is the optimistic one.

@@exercise Break-even and target profit
For each product line, compute profit, the variable cost ratio, break-even sales, the safety margin ratio (安全余裕率 = (sales − break-even) / sales), and the sales needed for a target profit. Then show how break-even changes when fixed costs rise by 20%.
@@lang python
@@starter
lines = [("cloud backup", 50_000, 30_000, 12_000, 15_000), ("helpdesk outsourcing", 80_000, 56_000, 30_000, 10_000), ("training", 20_000, 8_000, 9_000, 5_000)]

def break_even(fixed, variable_ratio):
    raise NotImplementedError
@@expected
cloud backup: profit ¥8,000k, 変動費率 0.60, 損益分岐点 ¥30,000k, 安全余裕率 40.0%
  for profit ¥15,000k: sales ¥67,500k; with fixed +20%: break-even ¥36,000k
helpdesk outsourcing: profit ¥-6,000k, 変動費率 0.70, 損益分岐点 ¥100,000k, 安全余裕率 -25.0%
  for profit ¥10,000k: sales ¥133,333k; with fixed +20%: break-even ¥120,000k
training: profit ¥3,000k, 変動費率 0.40, 損益分岐点 ¥15,000k, 安全余裕率 25.0%
  for profit ¥5,000k: sales ¥23,333k; with fixed +20%: break-even ¥18,000k
@@requires
def break_even
1 - variable_ratio
@@hint
Each line is `(name, sales, variable, fixed, target profit)` in thousands of yen. Target sales = `break_even(fixed + target, ratio)`.
@@solution
lines = [("cloud backup", 50_000, 30_000, 12_000, 15_000), ("helpdesk outsourcing", 80_000, 56_000, 30_000, 10_000), ("training", 20_000, 8_000, 9_000, 5_000)]

def break_even(fixed, variable_ratio):
    return fixed / (1 - variable_ratio)

for name, sales, variable, fixed, target in lines:
    ratio = variable / sales
    profit = sales - variable - fixed
    be = break_even(fixed, ratio)
    margin = (sales - be) / sales
    print(f"{name}: profit ¥{profit:,}k, 変動費率 {ratio:.2f}, 損益分岐点 ¥{be:,.0f}k, 安全余裕率 {margin:.1%}")
    print(f"  for profit ¥{target:,}k: sales ¥{break_even(fixed + target, ratio):,.0f}k; with fixed +20%: break-even ¥{break_even(fixed * 1.2, ratio):,.0f}k")

@@exercise ABC analysis and decisions under uncertainty
First, classify inventory items by annual usage value (A up to 70% cumulative, B up to 90%, C the rest; an item belongs to the class in which its cumulative share starts). Then, for a payoff table, choose the option under maximax, maximin and expected value.
@@lang python
@@starter
items = {"servers": 4_800, "laptops": 3_000, "switches": 900, "monitors": 700, "cables": 250, "mice": 200, "keyboards": 150}
payoffs = {"expand": (900, 300, -400), "keep": (500, 300, 100), "outsource": (600, 400, 0)}
probabilities = (0.3, 0.5, 0.2)

def abc(items):
    raise NotImplementedError
@@expected
servers     4,800 cumulative  48.0%  A
laptops     3,000 cumulative  78.0%  A
switches      900 cumulative  87.0%  B
monitors      700 cumulative  94.0%  B
cables        250 cumulative  96.5%  C
mice          200 cumulative  98.5%  C
keyboards     150 cumulative 100.0%  C
maximax: expand
maximin: keep
expected value: outsource {'expand': 340, 'keep': 320, 'outsource': 380}
@@requires
def abc
max(
min(
@@hint
Sort by value descending. Keep a running total `before` (cumulative share before adding the item): class A if `before < 0.7`, B if `before < 0.9`, else C. For decisions use `max(payoffs, key=...)`.
@@solution
items = {"servers": 4_800, "laptops": 3_000, "switches": 900, "monitors": 700, "cables": 250, "mice": 200, "keyboards": 150}
payoffs = {"expand": (900, 300, -400), "keep": (500, 300, 100), "outsource": (600, 400, 0)}
probabilities = (0.3, 0.5, 0.2)

def abc(items):
    total = sum(items.values())
    running, result = 0, []
    for name, value in sorted(items.items(), key=lambda kv: -kv[1]):
        before = running / total
        running += value
        cls = "A" if before < 0.7 else "B" if before < 0.9 else "C"
        result.append((name, value, running / total, cls))
    return result

for name, value, cumulative, cls in abc(items):
    print(f"{name:<10} {value:>6,} cumulative {cumulative:6.1%}  {cls}")

expected = {k: sum(p * v for p, v in zip(probabilities, row)) for k, row in payoffs.items()}
print("maximax:", max(payoffs, key=lambda k: max(payoffs[k])))
print("maximin:", max(payoffs, key=lambda k: min(payoffs[k])))
print("expected value:", max(expected, key=expected.get), {k: round(v) for k, v in expected.items()})
