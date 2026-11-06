@@lesson t13-science 🎓 Beyond IT: Scientific Computing and NumPy
@@concepts numbers, lists, functions
@@text
> **Beyond IT scope.** Scientific and numerical computing is the domain of researchers and engineers. It's included because NumPy underpins almost every Python data and ML library, and the time-series ideas here (smoothing, z-scores) are used in IT monitoring too.

## Why NumPy

Python lists hold pointers to separate objects; a loop over a million numbers runs a million bytecode steps. A **NumPy array** stores raw numbers contiguously and runs operations in compiled C, often 50–100× faster:

```python,norun
import numpy as np

a = np.array([1.0, 2.0, 3.0])
a * 2 + 1                    # array([3., 5., 7.])  elementwise, no loop
a.mean(), a.std()            # statistics
m = np.arange(6).reshape(2, 3)
m.T                          # transpose, shape (3, 2)
m @ m.T                      # matrix multiplication
m[:, 1]                      # column 1
m[m > 2]                     # boolean mask: elements greater than 2
np.linalg.solve(A, b)        # solve linear equations
```

Key ideas:

- **Vectorisation**: write operations on whole arrays, not loops.
- **Broadcasting**: arrays of different shapes combine by stretching size-1 dimensions (`matrix - row_means[:, None]`).
- **dtype**: every element has the same type (`float64`, `int32`, `bool`), which is why it's fast.

The wider ecosystem: **SciPy** (optimisation, signal processing, statistics), **SymPy** (symbolic maths), **matplotlib** (plots), **Numba** (JIT-compiles numeric Python loops).

## Matrices without NumPy

Under the hood, a matrix product is three nested loops:

```python
def matmul(a, b):
    return [[sum(x * y for x, y in zip(row, col)) for col in zip(*b)] for row in a]

print(matmul([[1, 2], [3, 4]], [[5, 6], [7, 8]]))
```

`zip(*b)` transposes `b`, giving its columns. Exercise 1 builds a small matrix toolkit like this.

## Time series: smoothing and anomalies

Sensor readings and server metrics are noisy. Two standard tools:

- **Moving average**: the mean of the last *w* points smooths noise (with a delay).
- **Z-score**: `(x − mean) / stdev` measures how many standard deviations a point is from normal. |z| > 3 is a common anomaly threshold, as long as the data is roughly bell-shaped.

Use a **rolling** window so the "normal" level adapts as the system changes (more traffic in the daytime).

## Numerical methods

Many problems have no exact formula, so we approximate:

- **Newton's method** finds roots: `x_next = x − f(x) / f'(x)`, converging very fast near the answer.
- **Numerical integration** (trapezoid, Simpson's rule) estimates areas under curves.
- **Floating point** has limited precision: `0.1 + 0.2 != 0.3`. Compare with `math.isclose`, and use `math.fsum` for accurate sums.

@@quiz
Why are NumPy operations much faster than Python loops?
* Arrays store raw numbers contiguously and operations run in compiled code without per-element interpreter overhead
- NumPy uses the GPU automatically
- Python loops are disabled
- NumPy skips some elements
? This is called vectorisation.

@@quiz
A reading has a z-score of 4.2 against the last hour. What does that suggest?
- It's typical
* It's 4.2 standard deviations from the recent mean: likely an anomaly
- The data is wrong
- The mean is 4.2
? |z| > 3 is a common alert threshold.

@@quiz
How should you compare two floats for equality?
- With ==
* With `math.isclose` (a tolerance)
- Convert to strings
- With is
? Floating-point results carry small rounding errors.

@@drill
Which NumPy function creates an array of evenly spaced numbers over an interval with a given count?
= linspace
= np.linspace
= numpy.linspace
? np.linspace(0, 1, 5) gives five points.

@@flashcards
ndarray :: NumPy's fast, fixed-type, contiguous array
Vectorisation :: operating on whole arrays instead of loops
Broadcasting :: combining arrays of different shapes by stretching dimensions
`@` :: matrix multiplication operator
Moving average :: mean over a sliding window; smooths noise
Z-score :: distance from the mean in standard deviations
Newton's method :: iterative root finding with the derivative
`math.isclose` :: compare floats with a tolerance

@@exercise A small matrix toolkit
Write, for matrices as lists of rows:

- `shape(m)` → `(rows, cols)`; raise `ValueError("ragged matrix")` if rows have different lengths
- `transpose(m)`
- `matmul(a, b)`; raise `ValueError("cannot multiply 2x3 by 2x2")` (using the shapes) when the inner sizes differ
- `identity(n)`
@@starter
def shape(m):
    pass


def transpose(m):
    pass


def matmul(a, b):
    pass


def identity(n):
    pass


a = [[1, 2, 3], [4, 5, 6]]
print(shape(a), transpose(a))
print(matmul(a, transpose(a)), matmul(identity(2), a) == a)
@@expected
(2, 3) [[1, 4], [2, 5], [3, 6]]
[[14, 32], [32, 77]] True
@@tests
try:
    matmul(a, [[1, 0], [0, 1]])
    check("multiplied", "ValueError", "bad shapes")
except ValueError as e:
    check(str(e), "cannot multiply 2x3 by 2x2", "message")
try:
    shape([[1, 2], [3]])
    check("ok", "ValueError", "ragged")
except ValueError as e:
    check(str(e), "ragged matrix", "ragged message")
check(identity(3), [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "identity 3")
check(matmul([[2]], [[3]]), [[6]], "1x1")
@@hint
`transpose(m)` is `[list(col) for col in zip(*m)]`. In `matmul`, compute both shapes first, compare `a`'s columns with `b`'s rows, then use the comprehension from the lesson.
@@solution
def shape(m):
    cols = {len(row) for row in m}
    if len(cols) > 1:
        raise ValueError("ragged matrix")
    return len(m), cols.pop() if cols else 0


def transpose(m):
    return [list(col) for col in zip(*m)]


def matmul(a, b):
    (ar, ac), (br, bc) = shape(a), shape(b)
    if ac != br:
        raise ValueError(f"cannot multiply {ar}x{ac} by {br}x{bc}")
    return [[sum(x * y for x, y in zip(row, col)) for col in zip(*b)] for row in a]


def identity(n):
    return [[1 if i == j else 0 for j in range(n)] for i in range(n)]


a = [[1, 2, 3], [4, 5, 6]]
print(shape(a), transpose(a))
print(matmul(a, transpose(a)), matmul(identity(2), a) == a)

@@exercise Rolling anomaly detection
Write:

- `moving_average(values, window)`: the mean of each full window, rounded to 2 decimals (`len(values) - window + 1` results)
- `anomalies(values, window, threshold=3.0)`: for each index `i >= window`, compute the mean and **population** standard deviation (`statistics.pstdev`) of the previous `window` values; if the stdev is 0, skip the point; otherwise flag `i` when `abs(z) > threshold`. Return a list of `(index, value, z)` with z rounded to 1 decimal.
@@starter
import statistics


def moving_average(values, window):
    pass


def anomalies(values, window, threshold=3.0):
    pass


cpu = [31, 33, 30, 32, 34, 31, 33, 95, 32, 30]
print(moving_average(cpu[:6], 3))
print(anomalies(cpu, 5))
@@expected
[31.33, 31.67, 32.0, 32.33]
[(7, 95, 44.5)]
@@tests
check(moving_average([1, 2], 3), [], "window bigger than data")
check(anomalies([5, 5, 5, 5, 50], 4), [], "flat history skipped")
check(anomalies([10, 12, 10, 12, 4], 4, threshold=2), [(4, 4, -7.0)], "low anomaly")
@@hint
`moving_average`: `[round(sum(values[i:i + window]) / window, 2) for i in range(len(values) - window + 1)]`. In `anomalies`, the history for index `i` is `values[i - window:i]`.
@@solution
import statistics


def moving_average(values, window):
    return [round(sum(values[i:i + window]) / window, 2) for i in range(len(values) - window + 1)]


def anomalies(values, window, threshold=3.0):
    found = []
    for i in range(window, len(values)):
        history = values[i - window:i]
        mean = statistics.mean(history)
        sd = statistics.pstdev(history)
        if sd == 0:
            continue
        z = (values[i] - mean) / sd
        if abs(z) > threshold:
            found.append((i, values[i], round(z, 1)))
    return found


cpu = [31, 33, 30, 32, 34, 31, 33, 95, 32, 30]
print(moving_average(cpu[:6], 3))
print(anomalies(cpu, 5))
