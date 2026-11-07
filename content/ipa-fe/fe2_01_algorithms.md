@@lesson fe2-algorithms アルゴリズムとデータ構造
@@lang python
@@text
## Data structures (データ構造)

| Structure | 日本語 | Key idea |
|---|---|---|
| array | 配列 | fixed size, direct access by index |
| list (linked) | 連結リスト | each cell points to the next (ポインタ); fast insert/delete, slow search |
| stack | スタック | **LIFO** (後入れ先出し): push / pop. Function calls, undo, RPN evaluation |
| queue | キュー (待ち行列) | **FIFO** (先入れ先出し): enqueue / dequeue. Print jobs, buffers |
| tree | 木 | root, nodes, leaves (葉). Binary tree: at most 2 children |
| binary search tree | 2分探索木 | left < node < right: in-order traversal gives sorted order |
| heap | ヒープ | parent ≤ children (min-heap): priority queues, heap sort |
| hash table | ハッシュ表 | key → hash function → slot; collisions (衝突, シノニム) handled by chaining or open addressing (線形探査) |

Tree traversals (走査): **pre-order** (先行順: node, left, right), **in-order** (中間順: left, node, right), **post-order** (後行順: left, right, node).

## Searching (探索)

| Method | Comparisons (average) | Needs |
|---|---|---|
| Linear search (線形探索) | n/2 → O(n) | nothing; a **sentinel** (番兵) at the end saves a check per step |
| Binary search (2分探索) | log₂n → O(log n) | **sorted** data |
| Hash search (ハッシュ探索) | ~1 → O(1) | a hash table |

Binary search of 1,000 items: at most about 10 comparisons (2¹⁰ = 1024).

## Sorting (整列, ソート)

| Sort | 日本語 | Average | Notes |
|---|---|---|---|
| bubble | バブルソート | O(n²) | swap neighbours; largest "bubbles" to the end |
| selection | 選択ソート | O(n²) | find the minimum, put it first |
| insertion | 挿入ソート | O(n²) | fast for nearly sorted data |
| quick | クイックソート | O(n log n) | pivot (基準値), partition, recurse; worst O(n²) |
| merge | マージソート | O(n log n) | split, sort halves, merge; needs extra memory; **stable** |
| heap | ヒープソート | O(n log n) | build a heap, take the top repeatedly |

**Stable** (安定) sort: equal keys keep their original order.

## Recursion (再帰)

A function that calls itself, with a **base case** that stops it. `fact(n) = n × fact(n − 1)`, `fact(0) = 1`. Each call uses stack space; deep recursion can overflow it.

## Complexity (計算量)

O(1) < O(log n) < O(n) < O(n log n) < O(n²) < O(2ⁿ). Doubling the input of an O(n²) algorithm makes it about **4×** slower.

@@exercise Bubble sort with a swap counter
科目B loves asking "how many times does line X run?". Write `bubble_sort(a)` that sorts a copy of the list with bubble sort (compare neighbours, swap when the left one is bigger, repeat passes until no swaps) and returns `(sorted_list, swaps)`. Print the result for `[5, 1, 4, 2, 8]` and `[3, 2, 1]`.
@@starter
def bubble_sort(a):
    a = list(a)
    swaps = 0
    # repeat: go through neighbours, swap if a[k] > a[k + 1], count swaps; stop when a pass had no swaps
    return a, swaps


print(bubble_sort([5, 1, 4, 2, 8]))
print(bubble_sort([3, 2, 1]))
@@expected
([1, 2, 4, 5, 8], 4)
([1, 2, 3], 3)
@@hint
`while True:` with a `swapped = False` flag; inside, `for k in range(len(a) - 1):` and `a[k], a[k + 1] = a[k + 1], a[k]` when needed. `break` when nothing was swapped.
@@solution
def bubble_sort(a):
    a = list(a)
    swaps = 0
    while True:
        swapped = False
        for k in range(len(a) - 1):
            if a[k] > a[k + 1]:
                a[k], a[k + 1] = a[k + 1], a[k]
                swaps += 1
                swapped = True
        if not swapped:
            return a, swaps


print(bubble_sort([5, 1, 4, 2, 8]))
print(bubble_sort([3, 2, 1]))

@@quiz
Which data structure is LIFO (last in, first out)?
* Stack
- Queue
- Hash table
- Binary search tree
? 後入れ先出し: like a stack of plates.

@@quiz
What does binary search require?
* The data must be sorted
- The data must be in a hash table
- A linked list
- An even number of elements
? It halves the search range each step by comparing with the middle.

@@quiz
About how many comparisons does binary search need at most for 1,000,000 sorted items?
* About 20
- About 1,000
- About 500,000
- About 1,000,000
? 2²⁰ ≈ 1,000,000.

@@quiz
In-order traversal of a binary search tree gives...
* The values in ascending order
- The values in random order
- The root first
- Only the leaves
? Left subtree, node, right subtree.

@@quiz
Which sort has an average of O(n log n) but a worst case of O(n²)?
* Quick sort
- Merge sort
- Heap sort
- Bubble sort
? A bad pivot every time (e.g. already sorted data with the first element as pivot) gives O(n²).

@@quiz
In a hash table, two different keys get the same hash value. What is this called?
* A collision (衝突 / シノニム)
- A stack overflow
- A sentinel
- A pivot
? Resolved by chaining or open addressing.

@@quiz
An O(n²) algorithm takes 1 second for 1,000 items. Roughly how long for 3,000?
* 9 seconds
- 3 seconds
- 6 seconds
- 27 seconds
? (3)² = 9.

@@quiz
Pre-order traversal of this tree: root A, left child B (children D, E), right child C. What's the order?
* A B D E C
- D B E A C
- D E B C A
- A B C D E
? Node, then left subtree, then right subtree.

@@flashcards
スタック (LIFO) :: Last in, first out: push/pop
キュー (FIFO) :: First in, first out: enqueue/dequeue
連結リスト :: Linked list
2分探索木 :: Binary search tree
ヒープ :: Heap (parent ≤ children)
ハッシュ表 :: Hash table
シノニム / 衝突 :: Collision
番兵 :: Sentinel value that ends a search loop
線形探索 / 2分探索 :: Linear O(n) / binary O(log n) search
先行順 / 中間順 / 後行順 :: Pre-order / in-order / post-order
バブル / 選択 / 挿入ソート :: O(n²) sorts
クイック / マージ / ヒープソート :: O(n log n) sorts
安定ソート :: Stable sort: equal keys keep order
再帰 :: Recursion
計算量 :: Complexity (Big O)
基準値 (ピボット) :: Pivot

@@drill
What's the Japanese term for a stack's ordering, "last in, first out"?
= 後入れ先出し
? Queue: 先入れ先出し.

@@exercise Trace the Tower of Hanoi recursively
A favourite 再帰 question: how many moves does the Tower of Hanoi take? Write `hanoi(n, src, dst, via, moves)` that appends each move `(disk, from, to)` to `moves`, using the recursive rule: move n−1 disks out of the way, move the biggest disk, move the n−1 disks back on top.

Print every move for 3 disks, then check that the number of moves is 2ⁿ − 1 for n = 1 to 10, and print how long 64 disks would take at one move per second (in years).
@@lang python
@@starter
def hanoi(n, src, dst, via, moves):
    raise NotImplementedError


moves = []
hanoi(3, "A", "C", "B", moves)
for i, (disk, a, b) in enumerate(moves, 1):
    print(f"{i}: disk {disk} {a} -> {b}")
for n in range(1, 11):
    m = []
    hanoi(n, "A", "C", "B", m)
    assert len(m) == 2 ** n - 1, n
print("moves = 2^n - 1 for n = 1..10: OK")
years = (2 ** 64 - 1) / (365.25 * 24 * 3600)
print(f"64 disks at 1 move/s: {years:.3e} years")
@@expected
1: disk 1 A -> C
2: disk 2 A -> B
3: disk 1 C -> B
4: disk 3 A -> C
5: disk 1 B -> A
6: disk 2 B -> C
7: disk 1 A -> C
moves = 2^n - 1 for n = 1..10: OK
64 disks at 1 move/s: 5.845e+11 years
@@requires
def hanoi
hanoi(n - 1
@@hint
The base case is `n == 0` (nothing to move). Each call makes two recursive calls with `n - 1`, swapping which peg is the helper.
@@solution
def hanoi(n, src, dst, via, moves):
    if n == 0:
        return
    hanoi(n - 1, src, via, dst, moves)
    moves.append((n, src, dst))
    hanoi(n - 1, via, dst, src, moves)

moves = []
hanoi(3, "A", "C", "B", moves)
for i, (disk, a, b) in enumerate(moves, 1):
    print(f"{i}: disk {disk} {a} -> {b}")
for n in range(1, 11):
    m = []
    hanoi(n, "A", "C", "B", m)
    assert len(m) == 2 ** n - 1, n
print("moves = 2^n - 1 for n = 1..10: OK")
years = (2 ** 64 - 1) / (365.25 * 24 * 3600)
print(f"64 disks at 1 move/s: {years:.3e} years")
