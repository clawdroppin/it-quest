@@lesson fe2-graphs-rpn Stacks, Queues and Graphs: Reverse Polish Notation, BFS and Shortest Paths (科目B)
@@concepts lists, loops, functions
@@lang python
@@text
Two families of 科目B questions appear again and again: **stack and queue** programs (most famously converting and evaluating **逆ポーランド記法**, reverse Polish notation) and **graph** programs that walk an adjacency matrix with **breadth-first search (幅優先探索)** or find the **shortest path (最短経路)**. The exam writes them in its pseudo-language with **1-based arrays**, and asks you to fill in a blank line or predict the output. Implementing them once yourself makes those questions quick.

## Stacks and queues

- A **stack (スタック)** is last in, first out (**LIFO**): push (積む) and pop (取り出す) at the same end. It's used for function calls, undo, bracket matching and expression evaluation.
- A **queue (キュー, 待ち行列)** is first in, first out (**FIFO**): enqueue at the back and dequeue at the front. It's used for BFS, print jobs and buffers.
- In the exam's pseudo-language they're usually arrays with a top or head/tail index. Watch the off-by-one: is `top` the index of the last element or of the next free slot?

## Reverse Polish notation (逆ポーランド記法)

- Ordinary **infix** notation: `(A + B) × C`. **Reverse Polish** (postfix) notation puts the operator after its operands: `A B + C ×`. No brackets are needed.
- **Evaluating** RPN with a stack: read tokens left to right. Push numbers. For an operator, pop two values (**the first pop is the right operand**), apply the operator and push the result. At the end the stack holds one value, the answer.
- **Converting** infix to RPN (Dijkstra's shunting-yard algorithm):
  - output numbers and names immediately.
  - for an operator, first pop to the output any operators on the stack with **higher or equal precedence** (for left-associative operators), then push it.
  - `(` is pushed. `)` pops to the output until the matching `(`, which is discarded.
  - at the end, pop everything left.
- Precedence: × and ÷ before + and −. Example: `A + B × C` becomes `A B C × +`, and `(A + B) × C` becomes `A B + C ×`.
- The exam also uses the **tree view**: RPN is the **post-order traversal (後行順・帰りがけ順)** of the expression tree. Pre-order (先行順・行きがけ順) gives Polish notation, and in-order (中間順・通りがけ順) gives infix.

## Graphs

- A graph has **vertices (頂点)** and **edges (辺)**. It's **directed (有向)** or **undirected (無向)**, and weighted (each edge has a cost) or not.
- An **adjacency matrix (隣接行列)** `adj[i][j]` is 1 (or the weight) if there's an edge from i to j and 0 otherwise. For an undirected graph it's symmetric. The exam uses it because it's easy to write with 2-D arrays.
- **Breadth-first search (BFS)** uses a **queue**. It visits vertices in order of distance (number of edges) from the start, so it finds the **shortest path in an unweighted graph**. Record each vertex's predecessor to rebuild the path.
- **Depth-first search (DFS)** uses a **stack** or recursion and goes as deep as possible first. It's used for exploring, cycle detection and topological sort.
- **Dijkstra's algorithm** finds shortest paths with **non-negative weights**:
  1. Set every distance to ∞ and the start's to 0.
  2. Repeatedly pick the unvisited vertex with the smallest distance, mark it visited, and **relax** its edges: if `dist[u] + w < dist[v]`, update `dist[v]` and `prev[v] = u`.
  3. With an adjacency matrix and a simple scan, it takes O(n²).

## Exam tips

- Draw the stack or queue contents after each step. Most mistakes are about which value is popped first.
- For BFS questions, write the queue after each dequeue and mark visited vertices **when they're enqueued**, not when dequeued, or they get added twice.
- With 1-based arrays, a loop is usually `i を 1 から n まで 1 ずつ増やす`. Check whether the question's array starts at 1 before you index.

@@quiz
Evaluate the RPN expression `3 4 + 2 ×`.
* 14
- 11
- 10
- 24
? (3 + 4) × 2.

@@quiz
What is `A + B × C` in reverse Polish notation?
* A B C × +
- A B + C ×
- + A × B C
- A × B C +
? × has higher precedence.

@@quiz
When evaluating `8 2 −` with a stack, which value is the right operand?
* 2, the first value popped
- 8, the first value popped
- Either; subtraction is commutative
- Neither
? First pop = right operand, so 8 − 2 = 6.

@@quiz
Which data structure does breadth-first search use?
* A queue
- A stack
- A heap only
- A hash table
? DFS uses a stack or recursion.

@@quiz
Which traversal of an expression tree produces reverse Polish notation?
* Post-order (帰りがけ順)
- Pre-order (行きがけ順)
- In-order (通りがけ順)
- Level order
? Pre-order gives Polish notation.

@@quiz
Dijkstra's algorithm requires which condition on edge weights?
* No negative weights
- All weights equal
- Weights below 10
- Only integer weights
? Use Bellman-Ford with negative weights.

@@quiz
In an undirected graph's adjacency matrix, what is always true?
* It is symmetric: adj[i][j] = adj[j][i]
- The diagonal is all 1
- Every row sums to 1
- It's upper triangular
? Each edge goes both ways.

@@flashcards
スタック :: stack, LIFO
キュー :: queue, FIFO
逆ポーランド記法 :: reverse Polish (postfix) notation
RPN evaluation :: push numbers, pop two for an operator
first pop :: the right operand
shunting-yard :: infix to RPN with an operator stack
帰りがけ順 :: post-order → RPN
行きがけ順 :: pre-order → Polish notation
通りがけ順 :: in-order → infix
隣接行列 :: adjacency matrix
有向 / 無向 :: directed / undirected
幅優先探索 :: BFS, queue, shortest path by edges
深さ優先探索 :: DFS, stack or recursion
ダイクストラ法 :: shortest paths, non-negative weights
relax :: dist[u] + w < dist[v] → update

@@drill
Evaluate `5 1 2 + 4 × + 3 −`.
= 14
? 5 + (1 + 2) × 4 − 3.

@@drill
Convert `(A − B) × (C + D)` to reverse Polish notation (separate tokens with spaces, use × and −).
= A B − C D + ×
= A B - C D + *
= AB−CD+×
? Each bracket becomes a sub-expression.

@@exercise Convert to and evaluate reverse Polish notation
Implement the shunting-yard conversion for `+ - * /` and brackets (all left-associative), and an RPN evaluator that uses a list as a stack (the first pop is the right operand; use integer division `//` for `/`). Print the RPN and the value of each expression, with the stack after every step for the first one.
@@starter
PREC = {"+": 1, "-": 1, "*": 2, "/": 2}

def to_rpn(tokens):
    raise NotImplementedError

def evaluate(rpn, trace=False):
    raise NotImplementedError

exprs = ["( 3 + 4 ) * 2", "3 + 4 * 2", "8 - 2 - 1", "( 1 + 2 ) * ( 3 + 4 ) / 7", "5 + ( 1 + 2 ) * 4 - 3"]
for i, e in enumerate(exprs):
    rpn = to_rpn(e.split())
    print(f"{e:<28} -> {' '.join(rpn):<22} = {evaluate(rpn, trace=(i == 0))}")
@@expected
    3  stack [3]
    4  stack [3, 4]
    +  stack [7]
    2  stack [7, 2]
    *  stack [14]
( 3 + 4 ) * 2                -> 3 4 + 2 *              = 14
3 + 4 * 2                    -> 3 4 2 * +              = 11
8 - 2 - 1                    -> 8 2 - 1 -              = 5
( 1 + 2 ) * ( 3 + 4 ) / 7    -> 1 2 + 3 4 + * 7 /      = 3
5 + ( 1 + 2 ) * 4 - 3        -> 5 1 2 + 4 * + 3 -      = 14
@@requires
def to_rpn
def evaluate
stack.pop()
@@hint
`to_rpn`: for a number, append it to the output; for an operator `t`, `while stack and stack[-1] != "(" and PREC[stack[-1]] >= PREC[t]: out.append(stack.pop())`, then push t; `(` is pushed; `)` pops until `(` and drops it; finally pop the rest. `evaluate`: for an operator, `b = stack.pop(); a = stack.pop()` and push the result. When tracing, print the token and the stack after each step.
@@solution
PREC = {"+": 1, "-": 1, "*": 2, "/": 2}

def to_rpn(tokens):
    out, stack = [], []
    for t in tokens:
        if t in PREC:
            while stack and stack[-1] != "(" and PREC[stack[-1]] >= PREC[t]:
                out.append(stack.pop())
            stack.append(t)
        elif t == "(":
            stack.append(t)
        elif t == ")":
            while stack[-1] != "(":
                out.append(stack.pop())
            stack.pop()
        else:
            out.append(t)
    while stack:
        out.append(stack.pop())
    return out

def evaluate(rpn, trace=False):
    stack = []
    for t in rpn:
        if t in PREC:
            b = stack.pop()
            a = stack.pop()
            stack.append({"+": a + b, "-": a - b, "*": a * b, "/": a // b}[t])
        else:
            stack.append(int(t))
        if trace:
            print(f"    {t:<2} stack {stack}")
    return stack.pop()

exprs = ["( 3 + 4 ) * 2", "3 + 4 * 2", "8 - 2 - 1", "( 1 + 2 ) * ( 3 + 4 ) / 7", "5 + ( 1 + 2 ) * 4 - 3"]
for i, e in enumerate(exprs):
    rpn = to_rpn(e.split())
    print(f"{e:<28} -> {' '.join(rpn):<22} = {evaluate(rpn, trace=(i == 0))}")

@@exercise BFS and Dijkstra on an adjacency matrix
Write BFS on an unweighted adjacency matrix with 1-based vertex numbers (row and column 0 unused), printing the queue after each dequeue and the shortest path by number of edges from vertex 1 to vertex 6. Then run Dijkstra's algorithm on a weighted matrix (0 means no edge) with a simple O(n²) scan, and print the distance and path to every vertex.
@@starter
N = 6
adj = [[0] * (N + 1) for _ in range(N + 1)]
for a, b in [(1, 2), (1, 3), (2, 4), (3, 4), (4, 5), (3, 5), (5, 6)]:
    adj[a][b] = adj[b][a] = 1

w = [[0] * (N + 1) for _ in range(N + 1)]
for a, b, cost in [(1, 2, 7), (1, 3, 2), (2, 4, 1), (3, 4, 5), (3, 5, 8), (4, 5, 2), (5, 6, 3), (4, 6, 9)]:
    w[a][b] = w[b][a] = cost

def bfs(start, goal):
    raise NotImplementedError

def dijkstra(start):
    raise NotImplementedError

def path(prev, v):
    p = []
    while v:
        p.append(v)
        v = prev[v]
    return p[::-1]

print("BFS path 1 -> 6:", bfs(1, 6))
dist, prev = dijkstra(1)
for v in range(1, N + 1):
    print(f"to {v}: {dist[v]:>2}  via {path(prev, v)}")
@@expected
  took 1, queue [2, 3]
  took 2, queue [3, 4]
  took 3, queue [4, 5]
  took 4, queue [5]
  took 5, queue [6]
  took 6, queue []
BFS path 1 -> 6: [1, 3, 5, 6]
to 1:  0  via [1]
to 2:  7  via [1, 2]
to 3:  2  via [1, 3]
to 4:  7  via [1, 3, 4]
to 5:  9  via [1, 3, 4, 5]
to 6: 12  via [1, 3, 4, 5, 6]
@@requires
def bfs
def dijkstra
visited[
@@hint
BFS: `queue = [start]`, `visited[start] = True`, `prev = [0] * (N + 1)`. Pop from the front with `queue.pop(0)`, print the queue, and for `v` in 1..N with `adj[u][v] == 1` and not visited, mark it visited, set `prev[v] = u` and append it. Rebuild the path with `path(prev, goal)`. Dijkstra: `dist = [INF] * (N + 1)`, `dist[start] = 0`; N times pick the unvisited `u` with the smallest dist, mark it, and relax every `v` with `w[u][v] > 0`.
@@solution
N = 6
adj = [[0] * (N + 1) for _ in range(N + 1)]
for a, b in [(1, 2), (1, 3), (2, 4), (3, 4), (4, 5), (3, 5), (5, 6)]:
    adj[a][b] = adj[b][a] = 1

w = [[0] * (N + 1) for _ in range(N + 1)]
for a, b, cost in [(1, 2, 7), (1, 3, 2), (2, 4, 1), (3, 4, 5), (3, 5, 8), (4, 5, 2), (5, 6, 3), (4, 6, 9)]:
    w[a][b] = w[b][a] = cost

def bfs(start, goal):
    visited = [False] * (N + 1)
    prev = [0] * (N + 1)
    queue = [start]
    visited[start] = True
    while queue:
        u = queue.pop(0)
        for v in range(1, N + 1):
            if adj[u][v] == 1 and not visited[v]:
                visited[v] = True
                prev[v] = u
                queue.append(v)
        print(f"  took {u}, queue {queue}")
    return path(prev, goal)

def dijkstra(start):
    INF = float("inf")
    dist = [INF] * (N + 1)
    prev = [0] * (N + 1)
    visited = [False] * (N + 1)
    dist[start] = 0
    for _ in range(N):
        u = min((v for v in range(1, N + 1) if not visited[v]), key=lambda v: dist[v])
        visited[u] = True
        for v in range(1, N + 1):
            if w[u][v] > 0 and dist[u] + w[u][v] < dist[v]:
                dist[v] = dist[u] + w[u][v]
                prev[v] = u
    return dist, prev

def path(prev, v):
    p = []
    while v:
        p.append(v)
        v = prev[v]
    return p[::-1]

print("BFS path 1 -> 6:", bfs(1, 6))
dist, prev = dijkstra(1)
for v in range(1, N + 1):
    print(f"to {v}: {dist[v]:>2}  via {path(prev, v)}")
