@@lesson n1-input Input & Converting Types
@@concepts input
@@text
Programs get more useful when they react to the user. `input()` waits for the user to type a line and press Enter, and **returns** what they typed as a string:

```python,norun
name = input("What's your name? ")
print(f"Hello, {name}!")
```

In this app, the text "typed" by the user comes from the **Input for the program** box under the editor. Each line in that box is one answer to one `input()`.

## Input is always text

`input()` always returns a `str`, even if the user types digits. To do math, **convert** it:

```python,norun
age_text = input("Age? ")
age = int(age_text)        # "42" -> 42
print(age + 1)
```

Usually written in one line: `age = int(input("Age? "))`.

| Function | Converts to | Example |
|---|---|---|
| `int("42")` | whole number | `42` |
| `float("3.5")` | decimal number | `3.5` |
| `str(42)` | text | `"42"` |
| `bool(...)` | True/False | (rarely used on input) |

If the text isn't a valid number, `int("abc")` raises a `ValueError`. You'll learn to handle that gracefully in the Errors lesson.

```python
print(int("42") + int("8"))
print("42" + "8")
print(float("2.5") * 2)
```

See the difference? `"42" + "8"` joins two strings: `"428"`. A classic bug when you forget to convert input.

## Splitting one line into parts

Often a user types several values on one line. `.split()` cuts a string at spaces and gives you the pieces:

```python
line = "web01 8 16"
parts = line.split()
print(parts)
print(parts[0], int(parts[1]) * 2)
```

You'll learn about the `[...]` list it returns in the Lists lesson. For now: `parts[0]` is the first piece, `parts[1]` the second.

@@quiz
The user types `5` at `n = input()`. What does `print(n * 3)` show?
- `15`
* `555`
- An error
- `5 5 5`
? `n` is the string `"5"`, and a string times 3 is the string repeated. Use `int(input())` to get a number.

@@exercise Subnet host counter
Read a **prefix length** (like `24`) from the input. A network with that prefix has `2 ** (32 - prefix) - 2` usable host addresses. Print the result like this, for the input `24`:

```text
A /24 network has 254 usable hosts.
```
@@stdin
24
@@starter
prefix = input()

@@expected
A /24 network has 254 usable hosts.
@@hint
Convert first: `prefix = int(input())`. Then compute `2 ** (32 - prefix) - 2` and use an f-string.
@@solution
prefix = int(input())
hosts = 2 ** (32 - prefix) - 2
print(f"A /{prefix} network has {hosts} usable hosts.")
@@text
> You'll meet this formula again in the Network+ track. The "minus 2" is because the first address of a network is the **network address** and the last is the **broadcast address**; neither can be given to a device.

@@exercise Ticket intake
A help desk script asks for a user's name and their number of open tickets, on two lines. Read both and print, for the input `Ana` and `3`:

```text
Ana has 3 open tickets. After closing one: 2.
```
@@stdin
Ana
3
@@starter
name = input()

@@expected
Ana has 3 open tickets. After closing one: 2.
@@hint
Call `input()` twice. Only the second one needs `int(...)`.
@@solution
name = input()
tickets = int(input())
print(f"{name} has {tickets} open tickets. After closing one: {tickets - 1}.")
