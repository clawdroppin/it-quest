@@lesson fe9-drills 科目B Algorithm Drills
@@text
科目B questions are always built from the same dozen classic algorithms. If you've written each one yourself, the pseudo-code on exam day looks familiar instead of scary. This project is ten of them, in Python, with **exam-style details**: positions start at **1**, binary search uses `(low + high) ÷ 2`, and the functions count comparisons and shifts, because that's exactly what the questions ask ("line α is executed how many times?").

| Function | 日本語 | What the exam asks |
|---|---|---|
| `to_base` | 基数変換 | convert 2026 to hex |
| `twos_complement` | 2の補数 | the bit pattern of −6 in 4 bits |
| `binary_search` | 2分探索 | how many comparisons until found |
| `insertion_sort` | 挿入ソート | how many times the shift line runs |
| `rpn` | 逆ポーランド記法 + スタック | the value of the expression; the stack's contents mid-way |
| `RingQueue` | リングバッファのキュー | the head/tail positions after some operations |
| `sieve` | エラトステネスのふるい | which numbers are crossed out |
| `gcd_steps` | ユークリッドの互除法 | how many loop iterations |
| `traverse` | 2分木の走査 | the visiting order (pre/in/post) |
| `hash_insert` | ハッシュ表 (線形探査) | which slot a key ends up in |

**No shortcuts**: no `sorted()`, `bin()`, `int(s, 2)`, `math.gcd` or `collections.deque`. The point is to know the inside.

When the tests pass, `python main.py` asks you random tracing questions and checks your answers **with your own code**. Do them on paper first, as on the exam.
@@project fe_drills 科目B Algorithm Drills
Write every function (and `RingQueue.enqueue` / `dequeue`) in `fe_drills.py` until `python -m unittest` passes. Then practise with `python main.py 20`: trace each question on paper before typing the answer.
@@file fe_drills.py
"""科目B drills: the algorithms the 基本情報技術者試験 asks you to trace, in Python.

The exam's pseudo-language (疑似言語) numbers arrays from 1, so functions that return
positions here do too. Write each one by hand, no built-in shortcuts like sorted(),
bin() or int(s, 2): the exam is about knowing what happens inside.
"""

DIGITS = "0123456789ABCDEF"


def to_base(n, base):
    """基数変換: n (>= 0) as a string in base 2-16, using DIGITS. to_base(255, 16) == "FF".

    Repeatedly divide by the base and collect the remainders (they come out in reverse).
    """
    raise NotImplementedError


def twos_complement(n, bits):
    """2の補数: n as a `bits`-wide two's complement bit string. twos_complement(-1, 8) == "11111111".

    Range is -2**(bits-1) .. 2**(bits-1) - 1, else ValueError. For negative n: 2**bits + n.
    """
    raise NotImplementedError


def binary_search(arr, target):
    """二分探索 on a sorted list. Returns (position 1-based or -1, number of comparisons).

    One comparison = one look at the middle element. Middle = (low + high) // 2 with 1-based
    low = 1, high = len(arr), exactly like the exam's pseudo-code.
    """
    raise NotImplementedError


def insertion_sort(arr):
    """挿入ソート. Returns (new sorted list, number of shifts). The input list isn't changed.

    A shift is moving one element one place to the right to make room.
    """
    raise NotImplementedError


def rpn(tokens):
    """逆ポーランド記法: evaluate e.g. ["3", "4", "+", "2", "*"] == 14 with a stack.

    Operators + - * and / (integer division that truncates toward zero, like most exam answers).
    Raise ValueError for a stack underflow, leftover values, division by zero or an unknown token.
    """
    raise NotImplementedError


class RingQueue:
    """キュー in a fixed-size array (リングバッファ). head points at the front, count = items stored.

    enqueue on a full queue and dequeue on an empty one raise OverflowError / IndexError.
    The tail position is (head + count) % capacity: no shifting, ever.
    """

    def __init__(self, capacity):
        self.items = [None] * capacity
        self.head = 0
        self.count = 0

    def enqueue(self, value):
        raise NotImplementedError

    def dequeue(self):
        raise NotImplementedError


def sieve(n):
    """エラトステネスのふるい: all primes <= n, in order."""
    raise NotImplementedError


def gcd_steps(a, b):
    """ユークリッドの互除法: returns (gcd, number of remainder steps). gcd_steps(1071, 1029) == (21, 3)."""
    raise NotImplementedError


def traverse(tree, root, order):
    """二分木の走査 over an exam-style tree: tree[i] = (value, left, right), node numbers 1-based,
    0 = no child. tree[0] is unused. order is "pre", "in" or "post". Returns the list of values."""
    raise NotImplementedError


def hash_insert(keys, size):
    """ハッシュ表 with linear probing (線形探査): h(k) = k % size; on a collision try the next slot
    (wrapping around). Returns the table (None = empty). Raise OverflowError when it's full."""
    raise NotImplementedError
@@file test_fe_drills.py
import unittest

from fe_drills import (RingQueue, binary_search, gcd_steps, hash_insert, insertion_sort, rpn, sieve,
                       to_base, traverse, twos_complement)


class NumberTests(unittest.TestCase):
    def test_to_base(self):
        self.assertEqual(to_base(255, 16), "FF")
        self.assertEqual(to_base(10, 2), "1010")
        self.assertEqual(to_base(0, 2), "0")
        self.assertEqual(to_base(64, 8), "100")
        self.assertEqual(to_base(2026, 16), "7EA")
        for bad in [(5, 1), (5, 17), (-1, 2)]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                to_base(*bad)

    def test_twos_complement(self):
        self.assertEqual(twos_complement(5, 8), "00000101")
        self.assertEqual(twos_complement(-1, 8), "11111111")
        self.assertEqual(twos_complement(-128, 8), "10000000")
        self.assertEqual(twos_complement(127, 8), "01111111")
        self.assertEqual(twos_complement(-6, 4), "1010")
        for bad in [128, -129]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                twos_complement(bad, 8)


class SearchSortTests(unittest.TestCase):
    def test_binary_search(self):
        arr = [3, 8, 15, 21, 30, 42, 57]
        self.assertEqual(binary_search(arr, 21), (4, 1))     # middle first
        self.assertEqual(binary_search(arr, 8), (2, 2))
        self.assertEqual(binary_search(arr, 57), (7, 3))
        self.assertEqual(binary_search(arr, 3), (1, 3))
        self.assertEqual(binary_search(arr, 10), (-1, 3))
        self.assertEqual(binary_search([], 1), (-1, 0))

    def test_insertion_sort(self):
        data = [5, 2, 4, 6, 1, 3]
        self.assertEqual(insertion_sort(data), ([1, 2, 3, 4, 5, 6], 9))
        self.assertEqual(data, [5, 2, 4, 6, 1, 3], "don't change the input")
        self.assertEqual(insertion_sort([1, 2, 3]), ([1, 2, 3], 0))
        self.assertEqual(insertion_sort([3, 2, 1]), ([1, 2, 3], 3))
        self.assertEqual(insertion_sort([]), ([], 0))


class StackQueueTests(unittest.TestCase):
    def test_rpn(self):
        self.assertEqual(rpn("3 4 + 2 *".split()), 14)
        self.assertEqual(rpn("5 1 2 + 4 * + 3 -".split()), 14)
        self.assertEqual(rpn("7 2 /".split()), 3)
        self.assertEqual(rpn("-7 2 /".split()), -3)
        self.assertEqual(rpn("2 5 -".split()), -3)
        for bad in ["+", "1 2", "1 0 /", "1 x +"]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                rpn(bad.split())

    def test_ring_queue(self):
        q = RingQueue(3)
        for v in "ABC":
            q.enqueue(v)
        with self.assertRaises(OverflowError):
            q.enqueue("D")
        self.assertEqual(q.dequeue(), "A")
        q.enqueue("D")                          # wraps around into slot 0
        self.assertEqual(q.items, ["D", "B", "C"])
        self.assertEqual((q.head, q.count), (1, 3))
        self.assertEqual([q.dequeue() for _ in range(3)], ["B", "C", "D"])
        self.assertEqual(q.head, 1)
        with self.assertRaises(IndexError):
            q.dequeue()


class MathTests(unittest.TestCase):
    def test_sieve(self):
        self.assertEqual(sieve(30), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
        self.assertEqual(sieve(2), [2])
        self.assertEqual(sieve(1), [])
        self.assertEqual(len(sieve(1000)), 168)

    def test_gcd(self):
        self.assertEqual(gcd_steps(1071, 1029), (21, 3))
        self.assertEqual(gcd_steps(48, 18), (6, 3))
        self.assertEqual(gcd_steps(7, 0), (7, 0))
        self.assertEqual(gcd_steps(17, 5), (1, 3))


class TreeHashTests(unittest.TestCase):
    #        A(1)
    #       /    \
    #     B(2)   C(3)
    #    /   \      \
    #  D(4)  E(5)   F(6)
    TREE = [None, ("A", 2, 3), ("B", 4, 5), ("C", 0, 6), ("D", 0, 0), ("E", 0, 0), ("F", 0, 0)]

    def test_traversals(self):
        self.assertEqual(traverse(self.TREE, 1, "pre"), list("ABDECF"))
        self.assertEqual(traverse(self.TREE, 1, "in"), list("DBEACF"))
        self.assertEqual(traverse(self.TREE, 1, "post"), list("DEBFCA"))
        self.assertEqual(traverse(self.TREE, 3, "pre"), list("CF"))
        with self.assertRaises(ValueError):
            traverse(self.TREE, 1, "level")

    def test_hash(self):
        self.assertEqual(hash_insert([12, 44, 13, 88, 23, 94], 11), [44, 12, 13, 88, 23, None, 94, None, None, None, None])
        with self.assertRaises(OverflowError):
            hash_insert([1, 2, 3], 2)


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""科目B practice: random tracing questions, checked with YOUR fe_drills.py.

    python main.py            10 questions
    python main.py 20         20 questions
"""
import random
import sys

import fe_drills as fe


def q_base(rng):
    n, base = rng.randint(20, 999), rng.choice([2, 8, 16])
    return f"Write {n} in base {base}.", fe.to_base(n, base)


def q_twos(rng):
    n = rng.randint(-128, -1)
    return f"8-bit two's complement (2の補数) of {n}?", fe.twos_complement(n, 8)


def q_search(rng):
    arr = sorted(rng.sample(range(1, 100), 15))
    target = rng.choice(arr)
    pos, steps = fe.binary_search(arr, target)
    return (f"二分探索 for {target} in {arr}: how many comparisons until it's found? "
            "(mid = (low + high) div 2, positions from 1)", str(steps))


def q_sort(rng):
    data = rng.sample(range(1, 10), 5)
    return f"挿入ソート of {data}: how many shifts?", str(fe.insertion_sort(data)[1])


def q_rpn(rng):
    a, b, c = rng.randint(1, 9), rng.randint(1, 9), rng.randint(1, 9)
    expr = rng.choice([f"{a} {b} + {c} *", f"{a} {b} {c} * +", f"{a} {b} - {c} *", f"{a} {b} * {c} -"])
    return f"Evaluate the 逆ポーランド記法 expression: {expr}", str(fe.rpn(expr.split()))


def q_gcd(rng):
    a, b = rng.randint(100, 999), rng.randint(10, 99)
    g, steps = fe.gcd_steps(a, b)
    return f"ユークリッドの互除法 on {a} and {b}: what's the GCD?", str(g)


def q_hash(rng):
    keys = rng.sample(range(10, 60), 5)
    table = fe.hash_insert(keys, 7)
    k = keys[-1]
    return f"Insert {keys} into a size-7 table, h(k) = k mod 7, linear probing. Which slot (0-6) holds {k}?", str(table.index(k))


QUESTIONS = [q_base, q_twos, q_search, q_sort, q_rpn, q_gcd, q_hash]


def main(argv):
    sys.stdout.reconfigure(errors="replace")   # a console without Japanese fonts shows ? instead of crashing
    count = int(argv[0]) if argv else 10
    rng = random.Random()
    score = 0
    for i in range(1, count + 1):
        try:
            question, answer = rng.choice(QUESTIONS)(rng)
        except NotImplementedError:
            print("Some functions in fe_drills.py aren't written yet: run the tests first.")
            return 1
        print(f"\nQ{i}. {question}")
        try:
            given = input("> ").strip().upper()
        except EOFError:
            return 0
        if given == answer.upper():
            score += 1
            print("正解! Correct")
        else:
            print(f"Not quite: the answer is {answer}")
    print(f"\nScore: {score}/{count}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
@@text
> Tips: for `to_base`, build the string from the right: `DIGITS[n % base] + out`. For two's complement of a negative `n`, `2 ** bits + n` is the unsigned value with the same bits; then pad with `rjust(bits, "0")`. In `binary_search`, remember `arr[mid - 1]` because positions are 1-based. In `rpn`, pop **b first, then a** (`a − b`, not `b − a`).
@@project fe_drills_finished 科目B Algorithm Drills: the finished version
Reference solutions. Compare them with yours, then rewrite three of them in IPA pseudo-code by hand: that's the best 科目B practice there is. Next steps: add merge sort with a merge counter, a min-heap with `push`/`pop`, or a question type for your weakest topic in `main.py`.
@@file fe_drills.py
"""科目B drills: the algorithms the 基本情報技術者試験 asks you to trace, in Python.

The exam's pseudo-language (疑似言語) numbers arrays from 1, so functions that return
positions here do too. Write each one by hand, no built-in shortcuts like sorted(),
bin() or int(s, 2): the exam is about knowing what happens inside.
"""

DIGITS = "0123456789ABCDEF"


def to_base(n, base):
    """基数変換: n (>= 0) as a string in base 2-16, using DIGITS. to_base(255, 16) == "FF".

    Repeatedly divide by the base and collect the remainders (they come out in reverse).
    """
    if not 2 <= base <= 16 or n < 0:
        raise ValueError("base must be 2-16 and n >= 0")
    if n == 0:
        return "0"
    out = ""
    while n > 0:
        out = DIGITS[n % base] + out
        n //= base
    return out


def twos_complement(n, bits):
    """2の補数: n as a `bits`-wide two's complement bit string. twos_complement(-1, 8) == "11111111".

    Range is -2**(bits-1) .. 2**(bits-1) - 1, else ValueError. For negative n: 2**bits + n.
    """
    if not -(2 ** (bits - 1)) <= n <= 2 ** (bits - 1) - 1:
        raise ValueError(f"{n} doesn't fit in {bits} bits")
    if n < 0:
        n += 2 ** bits
    return to_base(n, 2).rjust(bits, "0")


def binary_search(arr, target):
    """二分探索 on a sorted list. Returns (position 1-based or -1, number of comparisons).

    One comparison = one look at the middle element. Middle = (low + high) // 2 with 1-based
    low = 1, high = len(arr), exactly like the exam's pseudo-code.
    """
    low, high = 1, len(arr)
    steps = 0
    while low <= high:
        mid = (low + high) // 2
        steps += 1
        if arr[mid - 1] == target:
            return mid, steps
        if arr[mid - 1] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1, steps


def insertion_sort(arr):
    """挿入ソート. Returns (new sorted list, number of shifts). The input list isn't changed.

    A shift is moving one element one place to the right to make room.
    """
    a = list(arr)
    shifts = 0
    for i in range(1, len(a)):
        key = a[i]
        j = i - 1
        while j >= 0 and a[j] > key:
            a[j + 1] = a[j]
            shifts += 1
            j -= 1
        a[j + 1] = key
    return a, shifts


def rpn(tokens):
    """逆ポーランド記法: evaluate e.g. ["3", "4", "+", "2", "*"] == 14 with a stack.

    Operators + - * and / (integer division that truncates toward zero, like most exam answers).
    Raise ValueError for a stack underflow, leftover values, division by zero or an unknown token.
    """
    stack = []
    for t in tokens:
        if t in ("+", "-", "*", "/"):
            if len(stack) < 2:
                raise ValueError("stack underflow")
            b = stack.pop()
            a = stack.pop()
            if t == "+":
                stack.append(a + b)
            elif t == "-":
                stack.append(a - b)
            elif t == "*":
                stack.append(a * b)
            else:
                if b == 0:
                    raise ValueError("division by zero")
                q = abs(a) // abs(b)
                stack.append(q if (a >= 0) == (b >= 0) else -q)
        else:
            try:
                stack.append(int(t))
            except ValueError:
                raise ValueError(f"unknown token {t!r}") from None
    if len(stack) != 1:
        raise ValueError("expression doesn't reduce to one value")
    return stack[0]


class RingQueue:
    """キュー in a fixed-size array (リングバッファ). head points at the front, count = items stored.

    enqueue on a full queue and dequeue on an empty one raise OverflowError / IndexError.
    The tail position is (head + count) % capacity: no shifting, ever.
    """

    def __init__(self, capacity):
        self.items = [None] * capacity
        self.head = 0
        self.count = 0

    def enqueue(self, value):
        if self.count == len(self.items):
            raise OverflowError("queue is full")
        self.items[(self.head + self.count) % len(self.items)] = value
        self.count += 1

    def dequeue(self):
        if self.count == 0:
            raise IndexError("queue is empty")
        value = self.items[self.head]
        self.items[self.head] = None
        self.head = (self.head + 1) % len(self.items)
        self.count -= 1
        return value


def sieve(n):
    """エラトステネスのふるい: all primes <= n, in order."""
    if n < 2:
        return []
    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False
    i = 2
    while i * i <= n:
        if is_prime[i]:
            for m in range(i * i, n + 1, i):
                is_prime[m] = False
        i += 1
    return [i for i in range(n + 1) if is_prime[i]]


def gcd_steps(a, b):
    """ユークリッドの互除法: returns (gcd, number of remainder steps). gcd_steps(1071, 1029) == (21, 3)."""
    steps = 0
    while b != 0:
        a, b = b, a % b
        steps += 1
    return a, steps


def traverse(tree, root, order):
    """二分木の走査 over an exam-style tree: tree[i] = (value, left, right), node numbers 1-based,
    0 = no child. tree[0] is unused. order is "pre", "in" or "post". Returns the list of values."""
    if root == 0:
        return []
    value, left, right = tree[root]
    if order == "pre":
        return [value] + traverse(tree, left, order) + traverse(tree, right, order)
    if order == "in":
        return traverse(tree, left, order) + [value] + traverse(tree, right, order)
    if order == "post":
        return traverse(tree, left, order) + traverse(tree, right, order) + [value]
    raise ValueError("order must be pre, in or post")


def hash_insert(keys, size):
    """ハッシュ表 with linear probing (線形探査): h(k) = k % size; on a collision try the next slot
    (wrapping around). Returns the table (None = empty). Raise OverflowError when it's full."""
    table = [None] * size
    for k in keys:
        pos = k % size
        for _ in range(size):
            if table[pos] is None:
                table[pos] = k
                break
            pos = (pos + 1) % size
        else:
            raise OverflowError("hash table is full")
    return table
@@file test_fe_drills.py
import unittest

from fe_drills import (RingQueue, binary_search, gcd_steps, hash_insert, insertion_sort, rpn, sieve,
                       to_base, traverse, twos_complement)


class NumberTests(unittest.TestCase):
    def test_to_base(self):
        self.assertEqual(to_base(255, 16), "FF")
        self.assertEqual(to_base(10, 2), "1010")
        self.assertEqual(to_base(0, 2), "0")
        self.assertEqual(to_base(64, 8), "100")
        self.assertEqual(to_base(2026, 16), "7EA")
        for bad in [(5, 1), (5, 17), (-1, 2)]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                to_base(*bad)

    def test_twos_complement(self):
        self.assertEqual(twos_complement(5, 8), "00000101")
        self.assertEqual(twos_complement(-1, 8), "11111111")
        self.assertEqual(twos_complement(-128, 8), "10000000")
        self.assertEqual(twos_complement(127, 8), "01111111")
        self.assertEqual(twos_complement(-6, 4), "1010")
        for bad in [128, -129]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                twos_complement(bad, 8)


class SearchSortTests(unittest.TestCase):
    def test_binary_search(self):
        arr = [3, 8, 15, 21, 30, 42, 57]
        self.assertEqual(binary_search(arr, 21), (4, 1))     # middle first
        self.assertEqual(binary_search(arr, 8), (2, 2))
        self.assertEqual(binary_search(arr, 57), (7, 3))
        self.assertEqual(binary_search(arr, 3), (1, 3))
        self.assertEqual(binary_search(arr, 10), (-1, 3))
        self.assertEqual(binary_search([], 1), (-1, 0))

    def test_insertion_sort(self):
        data = [5, 2, 4, 6, 1, 3]
        self.assertEqual(insertion_sort(data), ([1, 2, 3, 4, 5, 6], 9))
        self.assertEqual(data, [5, 2, 4, 6, 1, 3], "don't change the input")
        self.assertEqual(insertion_sort([1, 2, 3]), ([1, 2, 3], 0))
        self.assertEqual(insertion_sort([3, 2, 1]), ([1, 2, 3], 3))
        self.assertEqual(insertion_sort([]), ([], 0))


class StackQueueTests(unittest.TestCase):
    def test_rpn(self):
        self.assertEqual(rpn("3 4 + 2 *".split()), 14)
        self.assertEqual(rpn("5 1 2 + 4 * + 3 -".split()), 14)
        self.assertEqual(rpn("7 2 /".split()), 3)
        self.assertEqual(rpn("-7 2 /".split()), -3)
        self.assertEqual(rpn("2 5 -".split()), -3)
        for bad in ["+", "1 2", "1 0 /", "1 x +"]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                rpn(bad.split())

    def test_ring_queue(self):
        q = RingQueue(3)
        for v in "ABC":
            q.enqueue(v)
        with self.assertRaises(OverflowError):
            q.enqueue("D")
        self.assertEqual(q.dequeue(), "A")
        q.enqueue("D")                          # wraps around into slot 0
        self.assertEqual(q.items, ["D", "B", "C"])
        self.assertEqual((q.head, q.count), (1, 3))
        self.assertEqual([q.dequeue() for _ in range(3)], ["B", "C", "D"])
        self.assertEqual(q.head, 1)
        with self.assertRaises(IndexError):
            q.dequeue()


class MathTests(unittest.TestCase):
    def test_sieve(self):
        self.assertEqual(sieve(30), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
        self.assertEqual(sieve(2), [2])
        self.assertEqual(sieve(1), [])
        self.assertEqual(len(sieve(1000)), 168)

    def test_gcd(self):
        self.assertEqual(gcd_steps(1071, 1029), (21, 3))
        self.assertEqual(gcd_steps(48, 18), (6, 3))
        self.assertEqual(gcd_steps(7, 0), (7, 0))
        self.assertEqual(gcd_steps(17, 5), (1, 3))


class TreeHashTests(unittest.TestCase):
    #        A(1)
    #       /    \
    #     B(2)   C(3)
    #    /   \      \
    #  D(4)  E(5)   F(6)
    TREE = [None, ("A", 2, 3), ("B", 4, 5), ("C", 0, 6), ("D", 0, 0), ("E", 0, 0), ("F", 0, 0)]

    def test_traversals(self):
        self.assertEqual(traverse(self.TREE, 1, "pre"), list("ABDECF"))
        self.assertEqual(traverse(self.TREE, 1, "in"), list("DBEACF"))
        self.assertEqual(traverse(self.TREE, 1, "post"), list("DEBFCA"))
        self.assertEqual(traverse(self.TREE, 3, "pre"), list("CF"))
        with self.assertRaises(ValueError):
            traverse(self.TREE, 1, "level")

    def test_hash(self):
        self.assertEqual(hash_insert([12, 44, 13, 88, 23, 94], 11), [44, 12, 13, 88, 23, None, 94, None, None, None, None])
        with self.assertRaises(OverflowError):
            hash_insert([1, 2, 3], 2)


if __name__ == "__main__":
    unittest.main()
@@file main.py
"""科目B practice: random tracing questions, checked with YOUR fe_drills.py.

    python main.py            10 questions
    python main.py 20         20 questions
"""
import random
import sys

import fe_drills as fe


def q_base(rng):
    n, base = rng.randint(20, 999), rng.choice([2, 8, 16])
    return f"Write {n} in base {base}.", fe.to_base(n, base)


def q_twos(rng):
    n = rng.randint(-128, -1)
    return f"8-bit two's complement (2の補数) of {n}?", fe.twos_complement(n, 8)


def q_search(rng):
    arr = sorted(rng.sample(range(1, 100), 15))
    target = rng.choice(arr)
    pos, steps = fe.binary_search(arr, target)
    return (f"二分探索 for {target} in {arr}: how many comparisons until it's found? "
            "(mid = (low + high) div 2, positions from 1)", str(steps))


def q_sort(rng):
    data = rng.sample(range(1, 10), 5)
    return f"挿入ソート of {data}: how many shifts?", str(fe.insertion_sort(data)[1])


def q_rpn(rng):
    a, b, c = rng.randint(1, 9), rng.randint(1, 9), rng.randint(1, 9)
    expr = rng.choice([f"{a} {b} + {c} *", f"{a} {b} {c} * +", f"{a} {b} - {c} *", f"{a} {b} * {c} -"])
    return f"Evaluate the 逆ポーランド記法 expression: {expr}", str(fe.rpn(expr.split()))


def q_gcd(rng):
    a, b = rng.randint(100, 999), rng.randint(10, 99)
    g, steps = fe.gcd_steps(a, b)
    return f"ユークリッドの互除法 on {a} and {b}: what's the GCD?", str(g)


def q_hash(rng):
    keys = rng.sample(range(10, 60), 5)
    table = fe.hash_insert(keys, 7)
    k = keys[-1]
    return f"Insert {keys} into a size-7 table, h(k) = k mod 7, linear probing. Which slot (0-6) holds {k}?", str(table.index(k))


QUESTIONS = [q_base, q_twos, q_search, q_sort, q_rpn, q_gcd, q_hash]


def main(argv):
    sys.stdout.reconfigure(errors="replace")   # a console without Japanese fonts shows ? instead of crashing
    count = int(argv[0]) if argv else 10
    rng = random.Random()
    score = 0
    for i in range(1, count + 1):
        try:
            question, answer = rng.choice(QUESTIONS)(rng)
        except NotImplementedError:
            print("Some functions in fe_drills.py aren't written yet: run the tests first.")
            return 1
        print(f"\nQ{i}. {question}")
        try:
            given = input("> ").strip().upper()
        except EOFError:
            return 0
        if given == answer.upper():
            score += 1
            print("正解! Correct")
        else:
            print(f"Not quite: the answer is {answer}")
    print(f"\nScore: {score}/{count}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
@@text
🎉 Ten classic algorithms, written and traced. Do one IPA 科目B sample question set (IPA publishes them on its site) under time pressure next. 合格を目指して頑張ってください!
