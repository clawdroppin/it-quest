@@lesson n1-variables Variables
@@concepts variables
@@text
A **variable** is a name for a value, so you can use the value later. You create one with `=`:

```python
hostname = "web01"
cpu_cores = 8
print(hostname)
print(cpu_cores)
```

Read `hostname = "web01"` as "**hostname** now refers to the text `"web01"`". The `=` here doesn't mean "equals" like in math; it means **assign**: put the value on the right into the name on the left.

## Changing a variable

A variable can be given a new value at any time. The old value is forgotten:

```python
status = "starting"
print(status)
status = "running"
print(status)
```

You can also compute the new value from the old one:

```python
users_online = 10
users_online = users_online + 5
print(users_online)
```

## Printing several things

`print` accepts several values separated by commas, and puts a space between them:

```python
server = "db02"
users = 42
print("Server", server, "has", users, "users")
```

## Naming rules

- Names can contain letters, digits and `_`, but can't **start** with a digit.
- They're **case-sensitive**: `Server` and `server` are different variables.
- Python style is **snake_case**: lowercase words joined with underscores: `free_disk_gb`, `max_retries`.
- Pick names that say what the value *means*. `free_disk_gb` beats `x`.

## Types of values

Each value has a **type**. The basic ones:

| Type | Example | What it is |
|---|---|---|
| `str` | `"web01"` | text (a *string* of characters) |
| `int` | `42` | whole number |
| `float` | `3.75` | number with a decimal point |
| `bool` | `True`, `False` | yes/no |

`type()` tells you a value's type:

```python
print(type("web01"))
print(type(42))
print(type(3.75))
print(type(True))
```
## Variables are labels, not boxes

It helps to picture a variable as a **label** stuck onto a value, not a box that holds it. `a = b` doesn't copy anything; it sticks a second label onto the same value. For numbers and strings this makes no visible difference, but it matters later with lists, which you can change: two labels on one list means changes through one name show up through the other.

## Swapping and multiple assignment

Python can assign several variables in one line, which makes swapping two values easy:

```python
host, port = "db01", 5432
print(host, port)
primary, backup = "web01", "web02"
primary, backup = backup, primary
print("primary is now", primary)
```

The right-hand side is worked out first, then the values are assigned left to right, so no temporary variable is needed.

@@quiz
What does this print?

```text
count = 3
count = count + 2
print(count)
```
- 3
* 5
- count + 2
- An error
? `count + 2` is calculated first (3 + 2 = 5), then stored back into `count`.

@@quiz
Which is a good Python variable name for "number of failed logins"?
- `2failed`
- `FailedLogins`
* `failed_logins`
- `x`
? Python style is snake_case, names can't start with a digit, and the name should explain itself.

@@quiz
After `primary, backup = "web01", "web02"` and then `primary, backup = backup, primary`, what is `primary`?
* `"web02"`
- `"web01"`
- an error
- a tuple of both values
? The right side is evaluated first, then assigned.

@@drill
Which built-in function tells you the type of a value?
= type
= type()
? type(42) is int.

@@flashcards
variable :: a name that refers to a value
= :: assign a value to a name
snake_case :: Python's naming style: failed_logins
int / float / str / bool :: whole number / decimal / text / True or False
type(x) :: shows the type of a value
a, b = b, a :: swap two variables

@@exercise Server info card
Create three variables: `name` with the text `"mail01"`, `ram_gb` with the number `16`, and `location` with `"Tokyo"`. Then print them so the output is exactly:

```text
Server: mail01
RAM: 16 GB
Location: Tokyo
```
@@starter
# Create the three variables here

# Then print them
print("Server:", name)
@@expected
Server: mail01
RAM: 16 GB
Location: Tokyo
@@requires
name =
ram_gb =
location =
@@hint
`print("RAM:", ram_gb, "GB")` prints the three parts with spaces between them.
@@solution
name = "mail01"
ram_gb = 16
location = "Tokyo"

print("Server:", name)
print("RAM:", ram_gb, "GB")
print("Location:", location)

@@exercise Swap shifts
Two technicians swap shifts. Without typing the names again, make `morning` hold what `night` held and the other way round, so the program prints:

```text
Morning: Ben
Night: Ana
```
@@starter
morning = "Ana"
night = "Ben"

# Swap them here (tip: use a third variable)

print("Morning:", morning)
print("Night:", night)
@@expected
Morning: Ben
Night: Ana
@@hint
If you do `morning = night` first, Ana is lost. Save one of them in a temporary variable first: `temp = morning`.
@@solution
morning = "Ana"
night = "Ben"

temp = morning
morning = night
night = temp

print("Morning:", morning)
print("Night:", night)
@@text
> Python also has a shortcut for swapping: `morning, night = night, morning`. You'll see why it works in the lesson on tuples.
