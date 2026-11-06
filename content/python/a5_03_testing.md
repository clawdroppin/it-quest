@@lesson a5-testing Testing Your Code
@@concepts testing
@@text
Every exercise in this app is checked by **tests**: small pieces of code that call your functions and compare the result with what's expected. Professionals write tests for their own code too. Then, when you change something next month, the tests tell you within seconds whether you broke anything.

## assert

The simplest test is an `assert`: it does nothing if the condition is true, and raises `AssertionError` if it's false.

```python
def to_gb(mb):
    return mb / 1024

assert to_gb(1024) == 1
assert to_gb(0) == 0
print("all good")
```

## unittest

Python's built-in test framework. Tests are methods whose names start with `test_`, inside a class that inherits from `unittest.TestCase`:

```python
import unittest

def slugify(title):
    return "-".join(title.lower().split())


class SlugifyTests(unittest.TestCase):
    def test_simple(self):
        self.assertEqual(slugify("VPN Setup Guide"), "vpn-setup-guide")

    def test_extra_spaces(self):
        self.assertEqual(slugify("  Reset   password "), "reset-password")

    def test_empty(self):
        self.assertEqual(slugify(""), "")


unittest.main(argv=["x"], exit=False, verbosity=2)
```

Run in a project, it's usually `python -m unittest` in a terminal: it finds files named `test*.py` and runs every test in them. (Many teams use the `pytest` package instead, which allows plain `assert` statements. The ideas are the same.)

Useful assertions: `assertEqual`, `assertTrue`, `assertIn`, `assertIsNone`, `assertAlmostEqual` (for floats), and `assertRaises`:

```python
import unittest

def parse_port(text):
    p = int(text)
    if not 1 <= p <= 65535:
        raise ValueError("out of range")
    return p


class PortTests(unittest.TestCase):
    def test_ok(self):
        self.assertEqual(parse_port("443"), 443)

    def test_out_of_range(self):
        with self.assertRaises(ValueError):
            parse_port("0")


unittest.main(argv=["x"], exit=False)
```

## What to test

- The **normal** case.
- **Edge cases**: empty input, zero, one item, the biggest allowed value, just over the limit.
- **Bad input**: does it raise the right error?

A good test name says what it checks: `test_empty_list_returns_zero`.

## Test-driven habits

The project cards in the 🛠 Projects tab work this way: they give you failing tests first, and you write code until they pass. It's a great way to work: the tests describe exactly what "done" means.

@@quiz
Which inputs are the best extra tests for `average(numbers)`?
- More normal lists like [1, 2, 3] and [4, 5, 6]
* An empty list and a list with one number
- Very long variable names
- None: one test is enough
? Edge cases (empty, one item) are where bugs hide; more normal cases rarely find new ones.

@@quiz
In unittest, which method checks that two values are equal?
* `self.assertEqual(a, b)`
- `self.check(a, b)`
- `assert.equal(a, b)`
- `self.same(a, b)`
? There are also assertTrue, assertIn and assertRaises.

@@quiz
What does `with self.assertRaises(ValueError):` test?
* That the code inside raises a ValueError
- That the code never raises an error
- That a ValueError is printed
- That the function returns None
? Testing failure cases is as important as testing the happy path.

@@drill
Which command discovers and runs unittest tests from the project folder?
= python -m unittest
= python3 -m unittest
? Add -v for one line per test.

@@flashcards
assert x == y :: stop with an error if the condition is false
unittest.TestCase :: a class whose test_ methods are run
assertEqual :: check two values are equal
assertRaises :: check that code raises an exception
python -m unittest :: discover and run tests
edge cases :: empty input, zero, negatives, huge values

@@exercise Write the tests
The function `grade(score)` is given. Write a `unittest.TestCase` class named `GradeTests` with **at least 3** test methods that check it: a normal case, the exact boundaries (90 → `"A"`, 89 → `"B"`), and that a score above 100 raises `ValueError`. The app checks that your tests run and pass on this correct `grade`, and **fail** on a broken one.
@@starter
import unittest


def grade(score):
    if score > 100 or score < 0:
        raise ValueError("score must be 0-100")
    if score >= 90:
        return "A"
    if score >= 75:
        return "B"
    return "C"


class GradeTests(unittest.TestCase):
    pass


unittest.main(argv=["x"], exit=False)
@@tests
import unittest as _u

def _run(tests_cls):
    suite = _u.defaultTestLoader.loadTestsFromTestCase(tests_cls)
    result = _u.TextTestRunner(stream=open("nul" if __import__("os").name == "nt" else "/dev/null", "w")).run(suite)
    return result.testsRun, len(result.failures) + len(result.errors)

runs, problems = _run(GradeTests)
check(runs >= 3, True, "GradeTests has at least 3 tests")
check(problems, 0, "your tests pass on the correct grade()")

_good = grade
def grade(score):          # a broken version: the boundary is wrong
    if score > 100 or score < 0:
        raise ValueError("score must be 0-100")
    return "A" if score > 90 else ("B" if score >= 75 else "C")
_, problems_broken = _run(GradeTests)
check(problems_broken > 0, True, "your tests catch a wrong boundary (90 should be an A)")

def grade(score):          # broken: no validation
    return "A" if score >= 90 else ("B" if score >= 75 else "C")
_, problems_broken = _run(GradeTests)
check(problems_broken > 0, True, "your tests catch missing validation (101 should raise ValueError)")
grade = _good
@@hint
Three methods, e.g. `def test_boundary_a(self): self.assertEqual(grade(90), "A")`, and one with `with self.assertRaises(ValueError): grade(101)`.
@@solution
import unittest


def grade(score):
    if score > 100 or score < 0:
        raise ValueError("score must be 0-100")
    if score >= 90:
        return "A"
    if score >= 75:
        return "B"
    return "C"


class GradeTests(unittest.TestCase):
    def test_normal(self):
        self.assertEqual(grade(50), "C")

    def test_boundaries(self):
        self.assertEqual(grade(90), "A")
        self.assertEqual(grade(89), "B")
        self.assertEqual(grade(75), "B")

    def test_too_high(self):
        with self.assertRaises(ValueError):
            grade(101)


unittest.main(argv=["x"], exit=False)
