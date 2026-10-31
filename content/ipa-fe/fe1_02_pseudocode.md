@@lesson fe1-pseudocode 疑似言語: Reading the Exam's Pseudo-Code
@@lang python
@@text
科目B gives you programs in IPA's **疑似言語** and asks what they output, what goes in a blank (空欄), or what's wrong. The notation is small, so learn it once and every question gets easier.

## The notation

```text
○整数型: sumUntil(整数型: n)        // a function: return type, name, parameters
  整数型: i, total ← 0              // declare variables, optionally with a first value
  for (i を 1 から n まで 1 ずつ増やす)
    if (i mod 3 ＝ 0)
      total ← total ＋ i            // ← is assignment
    elseif (i ＝ n)
      total ← total × 2
    endif
  endfor
  return total
```

| Notation | Meaning | Python |
|---|---|---|
| `x ← 5` | assignment | `x = 5` |
| `＝ ≠ ≦ ≧` | comparisons | `== != <= >=` |
| `÷`, `mod` | integer division, remainder | `//`, `%` (for non-negative numbers) |
| `and`, `or`, `not` | logic | the same |
| `if (...) ... elseif (...) ... else ... endif` | branches | `if/elif/else` |
| `while (cond) ... endwhile` | test-first loop | `while` |
| `do ... while (cond)` | test-last loop: runs at least once | `while True: ... if not cond: break` |
| `for (i を 1 から n まで 1 ずつ増やす)` | i = 1, 2, ..., n | `for i in range(1, n + 1)` |
| `整数型の配列: a ← {3, 1, 4}` | an array | a list |
| `a[1]` | **the FIRST element** (arrays start at 1!) | `a[0]` |
| `aの要素数` | number of elements | `len(a)` |
| `未定義の値` | undefined | `None` |
| `○手続: name(...)` | a procedure (no return value) | a function returning nothing |
| `大域: 整数型: count` | a global variable | module-level variable |

**Arrays start at 1.** Almost every trap is an off-by-one. When translating to Python, either subtract 1 at every index or put a dummy at position 0.

## Trace tables (トレース表)

For `sumUntil(5)`:

| i | i mod 3 | i ＝ n? | total |
|---|---|---|---|
| (start) | | | 0 |
| 1 | 1 | no | 0 |
| 2 | 2 | no | 0 |
| 3 | 0 | – | 3 |
| 4 | 1 | no | 3 |
| 5 | 2 | yes | 6 |

Return value: **6**. Do this on paper for every 科目B question: it's slower at first, and much faster than guessing.

## Exam technique

- Read the **question first** (what is asked?), then the program.
- Try the **smallest input** that exercises the code (n = 1, an array of 2 elements).
- For blank-filling questions, try each answer choice against a tiny example: wrong choices usually fail immediately.
- Time: 100 minutes for 20 questions is 5 minutes each. Skip and come back.

@@exercise Translate a pseudo-code program
Translate this into Python and print `countPairs({3, 7, 1, 7, 3, 3})`. Remember the arrays start at 1.

```text
○整数型: countPairs(整数型の配列: a)
  整数型: i, j, count ← 0
  for (i を 1 から aの要素数 − 1 まで 1 ずつ増やす)
    for (j を i ＋ 1 から aの要素数 まで 1 ずつ増やす)
      if (a[i] ＝ a[j])
        count ← count ＋ 1
      endif
    endfor
  endfor
  return count
```
@@starter
def count_pairs(a):
    count = 0
    # i from 1 to len(a) - 1, j from i + 1 to len(a); compare a[i] and a[j] (1-based!)
    return count


print(count_pairs([3, 7, 1, 7, 3, 3]))
@@expected
4
@@hint
`for i in range(1, len(a)):` and `for j in range(i + 1, len(a) + 1):`, then compare `a[i - 1] == a[j - 1]`. The pairs are (3,3) three ways and (7,7) once.
@@solution
def count_pairs(a):
    count = 0
    for i in range(1, len(a)):
        for j in range(i + 1, len(a) + 1):
            if a[i - 1] == a[j - 1]:
                count += 1
    return count


print(count_pairs([3, 7, 1, 7, 3, 3]))

@@quiz
In 疑似言語, `整数型の配列: a ← {5, 9, 2}`. What is `a[1]`?
* 5
- 9
- 2
- undefined
? Arrays start at 1 in IPA pseudo-code.

@@quiz
What does `for (i を 2 から 10 まで 2 ずつ増やす)` loop over?
* 2, 4, 6, 8, 10
- 2, 4, 6, 8
- 2 through 10, every number
- 10, 8, 6, 4, 2
? "From 2 to 10, increasing by 2"; 10 is included.

@@quiz
What is the result of `17 mod 5` and `17 ÷ 5` (integer division)?
* 2 and 3
- 3 and 2
- 2 and 3.4
- 5 and 17
? 17 = 5 × 3 + 2.

@@quiz
How is `do ... while (cond)` different from `while (cond) ... endwhile`?
* The do-while body always runs at least once
- It never runs
- It runs forever
- There's no difference
? The condition is checked after the body.

@@quiz
Trace: x ← 1; while (x < 20) x ← x × 3 endwhile. What is x at the end?
* 27
- 9
- 20
- 81
? 1 → 3 → 9 → 27, and 27 < 20 is false.

@@quiz
Trace: s ← 0; for (i を 1 から 4 まで 1 ずつ増やす) s ← s ＋ i × i endfor. What is s?
* 30
- 10
- 16
- 20
? 1 + 4 + 9 + 16.

@@flashcards
← :: Assignment
整数型 / 実数型 / 文字型 / 論理型 :: Integer / real / character / boolean types
配列 :: Array (index starts at 1)
要素数 :: Number of elements
要素番号 :: Element index
mod :: Remainder
÷ :: Integer division (in integer context)
for (i を 1 から n まで 1 ずつ増やす) :: for i = 1..n step 1
while ... endwhile :: Test-first loop
do ... while :: Test-last loop (runs at least once)
○手続 :: Procedure (no return value)
大域 :: Global (variable)
未定義 :: Undefined
空欄 :: Blank (to fill in)
トレース :: Trace (step through by hand)
戻り値 :: Return value
引数 :: Argument / parameter

@@drill
In IPA pseudo-code, what's the first index of an array? (a number)
= 1
? Python starts at 0: watch out when translating.
