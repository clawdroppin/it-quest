@@lesson fe2-structures-hands-on データ構造 Hands-on: Heaps, Hash Tables, Trees and Binary Search
@@lang python
@@text
科目B questions rarely ask you to define a heap. Instead they give a program that **manipulates** one, using 1-based arrays (配列の要素番号は1から始まる), and ask what ends up in the array, or how many times a line runs. This lesson works through the four structures that appear most often, using the exam's conventions.

## Binary heap in an array (ヒープ)

With 1-based indexing, the node at index `i` has:
- parent `i ÷ 2` (integer division, 商)
- left child `2i`, right child `2i + 1`

**Insert**: put the value at the end, then **sift up** (上方移動): swap with the parent while it's smaller (for a min-heap).
**Remove the root**: move the last element to index 1, then **sift down** (下方移動): swap with the smaller child while it's larger.
Both take O(log n). Heap sort builds a heap and then removes the root n times: O(n log n), and it isn't stable (安定ではない).

## Hash tables (ハッシュ表)

- Hash function: often `key mod table size`, with a prime table size (e.g. 11) to spread keys.
- **Synonyms (シノニム)**: different keys with the same hash value, which collide.
- **Open addressing, linear probing (オープンアドレス法・線形探査)**: if the slot is taken, try `(h + 1) mod m`, `(h + 2) mod m`, and so on. Clusters grow, and deleted slots need a "deleted" marker so searches don't stop early.
- **Chaining (チェイン法)**: each slot holds a linked list.

## Binary search trees (2分探索木)

Insert by comparing from the root: smaller values go left, larger ones go right. In-order traversal (中間順) gives the values in sorted order. Inserting already sorted data produces a "list-shaped" tree with height n, which makes searches O(n). Balanced trees (AVL, B-trees in databases) prevent this.

## Binary search on a 1-based array

```
整数型: low ← 1, high ← dataの要素数, mid
while (low ≤ high)
  mid ← (low + high) ÷ 2
  if (data[mid] = target) return mid
  elseif (data[mid] < target) low ← mid + 1
  else high ← mid − 1
endwhile
return −1
```

The maximum number of comparisons is ⌊log₂n⌋ + 1: 7 for 100 items, and 10 for 1,000.

@@quiz
In a 1-based array heap, where is the parent of the node at index 9?
- 3
* 4
- 5
- 8
? 9 ÷ 2 = 4 (integer division).

@@quiz
Keys 13, 24 and 35 go into a table of size 11 with h(k) = k mod 11 and linear probing. Which slot does 35 end up in?
- 2
- 3
* 4
- 0
? All three hash to 2, so they take slots 2, 3 and 4.

@@quiz
The values 1, 2, 3, 4, 5 are inserted in that order into an empty binary search tree. What is its height (counting nodes)?
- 3
* 5
- 2
- 1
? Sorted input makes a list-shaped tree.

@@quiz
At most how many comparisons does binary search need for 1,000 sorted items?
- 100
- 500
* 10
- 20
? ⌊log₂1000⌋ + 1 = 10.

@@quiz
Which traversal of a binary search tree gives the values in ascending order?
- Pre-order (先行順)
* In-order (中間順)
- Post-order (後行順)
- Level order
? Left subtree, node, right subtree.

@@flashcards
parent of i :: i ÷ 2
children of i :: 2i and 2i + 1
sift up :: insert into a heap, swap with parent
sift down :: after removing the root
heap sort :: O(n log n), not stable
シノニム :: keys with the same hash
線形探査 :: try the next slot on a collision
チェイン法 :: a list per slot
2分探索木 :: left smaller, right larger
中間順 :: in-order, sorted output
sorted input into a BST :: degenerates into a list
binary search comparisons :: ⌊log₂n⌋ + 1

@@drill
In a 1-based heap array, what index is the right child of node 5?
= 11
? 2 × 5 + 1.

@@drill
With h(k) = k mod 7, which slot does key 23 hash to?
= 2
? 23 = 3 × 7 + 2.

@@exercise Trace a min-heap in a 1-based array
Implement insert (sift up) and remove-min (sift down) on a 1-based array (index 0 unused, like the exam's arrays starting at 1), printing the array and the number of swaps after each operation.
@@starter
heap = [None]

def insert(value):
    raise NotImplementedError

def remove_min():
    raise NotImplementedError

for v in [7, 3, 9, 1, 5, 8, 2]:
    swaps = insert(v)
    print(f"insert {v}: {heap[1:]} ({swaps} swaps)")
for _ in range(3):
    value, swaps = remove_min()
    print(f"remove {value}: {heap[1:]} ({swaps} swaps)")
@@expected
insert 7: [7] (0 swaps)
insert 3: [3, 7] (1 swaps)
insert 9: [3, 7, 9] (0 swaps)
insert 1: [1, 3, 9, 7] (2 swaps)
insert 5: [1, 3, 9, 7, 5] (0 swaps)
insert 8: [1, 3, 8, 7, 5, 9] (1 swaps)
insert 2: [1, 3, 2, 7, 5, 9, 8] (1 swaps)
remove 1: [2, 3, 8, 7, 5, 9] (1 swaps)
remove 2: [3, 5, 8, 7, 9] (2 swaps)
remove 3: [5, 7, 8, 9] (2 swaps)
@@requires
def insert
def remove_min
// 2
2 * i
@@hint
Insert: append the value, then set `i = len(heap) - 1`, and while `i > 1` and `heap[i] < heap[i // 2]`, swap and set `i = i // 2`. Remove: save `heap[1]`, move the last element to index 1, then repeatedly pick the smaller of the children `2 * i` and `2 * i + 1` (if they exist) and swap while it's smaller.
@@solution
heap = [None]

def insert(value):
    heap.append(value)
    i, swaps = len(heap) - 1, 0
    while i > 1 and heap[i] < heap[i // 2]:
        heap[i], heap[i // 2] = heap[i // 2], heap[i]
        i, swaps = i // 2, swaps + 1
    return swaps

def remove_min():
    top = heap[1]
    last = heap.pop()
    swaps = 0
    if len(heap) > 1:
        heap[1] = last
        i, n = 1, len(heap) - 1
        while 2 * i <= n:
            child = 2 * i
            if child + 1 <= n and heap[child + 1] < heap[child]:
                child += 1
            if heap[child] >= heap[i]:
                break
            heap[i], heap[child] = heap[child], heap[i]
            i, swaps = child, swaps + 1
    return top, swaps

for v in [7, 3, 9, 1, 5, 8, 2]:
    swaps = insert(v)
    print(f"insert {v}: {heap[1:]} ({swaps} swaps)")
for _ in range(3):
    value, swaps = remove_min()
    print(f"remove {value}: {heap[1:]} ({swaps} swaps)")

@@exercise Hash table with linear probing, and binary search counts
Insert keys into a size-11 table with `h(k) = k mod 11` and linear probing, counting the probes for each key. Then search for some keys (including one that's missing). Finally, count the comparisons that binary search makes on a sorted 1-based array.
@@starter
M = 11
keys = [27, 18, 29, 28, 39, 13, 16, 42, 17]

def put(table, key):
    raise NotImplementedError

def binary_search(data, target):
    raise NotImplementedError
@@expected
put 27: h= 5, probes=1
put 18: h= 7, probes=1
put 29: h= 7, probes=2
put 28: h= 6, probes=1
put 39: h= 6, probes=4
put 13: h= 2, probes=1
put 16: h= 5, probes=6
put 42: h= 9, probes=3
put 17: h= 6, probes=7
table: [42, 17, 13, '-', '-', 27, 28, 18, 29, 39, 16]
get 16: slot 10 after 6 probes
get 17: slot 1 after 7 probes
get 50: not found after 9 probes
binary search 13: index 1, 3 comparisons
binary search 42: index 9, 4 comparisons
binary search 30: index -1, 3 comparisons
@@requires
def put
% M
def binary_search
// 2
@@hint
`put`: start at `key % M` and step `(h + 1) % M` until you find `None`, then return the number of slots examined. Searching works the same way, but stops at the key or at an empty slot. For binary search, use `low = 1`, `high = n` and `data[0]` unused, and count every comparison with `data[mid]`.
@@solution
M = 11
keys = [27, 18, 29, 28, 39, 13, 16, 42, 17]

def put(table, key):
    h, probes = key % M, 1
    while table[h] is not None:
        h, probes = (h + 1) % M, probes + 1
    table[h] = key
    return probes

def get(table, key):
    h, probes = key % M, 1
    while table[h] is not None:
        if table[h] == key:
            return h, probes
        h, probes = (h + 1) % M, probes + 1
    return None, probes

def binary_search(data, target):
    low, high, comparisons = 1, len(data) - 1, 0
    while low <= high:
        mid = (low + high) // 2
        comparisons += 1
        if data[mid] == target:
            return mid, comparisons
        if data[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1, comparisons

table = [None] * M
for k in keys:
    print(f"put {k:>2}: h={k % M:>2}, probes={put(table, k)}")
print("table:", ["-" if v is None else v for v in table])
for k in (16, 17, 50):
    slot, probes = get(table, k)
    print(f"get {k}: {'slot ' + str(slot) if slot is not None else 'not found'} after {probes} probes")

data = [None] + sorted(keys)
for target in (13, 42, 30):
    index, comparisons = binary_search(data, target)
    print(f"binary search {target}: index {index}, {comparisons} comparisons")
