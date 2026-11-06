@@lesson n1-numbers Numbers & Math
@@concepts numbers
@@text
Computers are very good at math, and IT is full of it: disk sizes, bandwidth, uptime percentages, how many licenses to buy.

## The operators

```python
print(7 + 3)    # add
print(7 - 3)    # subtract
print(7 * 3)    # multiply
print(7 / 2)    # divide: always gives a float
print(7 // 2)   # floor division: whole part only
print(7 % 2)    # modulo: the remainder
print(2 ** 10)  # power: 2 to the 10th
```

Everything after `#` is a **comment**: a note for humans that Python ignores.

`//` and `%` look odd but are used all the time:

- `//` answers "how many *whole* times does it fit?" 125 minutes `// 60` is 2 hours.
- `%` answers "what's left over?" 125 minutes `% 60` is 5 minutes.
- `n % 2 == 0` is the classic test for "is n even?".

```python
total_minutes = 125
hours = total_minutes // 60
minutes = total_minutes % 60
print(hours, "h", minutes, "min")
```

## Order of operations

Like in school math: `**` first, then `* / // %`, then `+ -`. Use brackets to make it clear:

```python
print(2 + 3 * 4)     # 14
print((2 + 3) * 4)   # 20
```

## int and float

Whole numbers are `int`, numbers with a decimal point are `float`. Mixing them gives a float. Floats are stored in binary, so some decimals can't be exact:

```python
print(0.1 + 0.2)
print(round(0.1 + 0.2, 2))
print(round(3.14159, 3))
```

`round(x, n)` rounds to `n` decimal places. That tiny `0.30000000000000004` isn't a Python bug; every language does it. (For money, professionals use whole cents or the `decimal` module.)

## Useful built-ins

```python
print(abs(-12))         # distance from zero
print(max(4, 17, 9))    # the biggest
print(min(4, 17, 9))    # the smallest
print(int(7.9))         # cut off the decimals: 7
print(float(3))         # 3.0
```

## Shortcuts

`x += 5` means `x = x + 5`. The same works for `-=`, `*=`, `/=`:

```python
downloads = 100
downloads += 25
downloads *= 2
print(downloads)
```

@@quiz
What is `17 % 5`?
- 3.4
- 3
* 2
- 85
? 5 fits into 17 three times (15), with **2** left over. `%` gives the remainder.

@@quiz
A disk holds 1000 GB. Files of 64 GB each. How many *complete* files fit? Which expression gives it?
- `1000 / 64`
* `1000 // 64`
- `1000 % 64`
- `64 // 1000`
? `//` gives the whole number of times 64 fits: 15. `/` would give 15.625.

@@quiz
What is `2 + 3 * 4`?
* 14
- 20
- 24
- 9
? Multiplication happens before addition; use brackets to change the order.

@@drill
What is `7 // 2`?
= 3
? // is floor division; 7 / 2 is 3.5.

@@flashcards
`//` :: floor division: the whole part of a division (7 // 2 is 3)
`%` :: modulo: the remainder of a division (7 % 2 is 1)
`**` :: power (2 ** 3 is 8)
`x += 1` :: shortcut for x = x + 1

@@exercise Uptime report
A server has been up for `200000` seconds. Using `//` and `%`, print how long that is in days, hours, minutes and seconds, exactly like this:

```text
2 days, 7 hours, 33 minutes, 20 seconds
```

(One day is 86400 seconds, one hour is 3600.)
@@starter
seconds = 200000

days = 0
hours = 0
minutes = 0
secs = 0

print(days, "days,", hours, "hours,", minutes, "minutes,", secs, "seconds")
@@expected
2 days, 7 hours, 33 minutes, 20 seconds
@@requires
//
%
@@hint
`days = seconds // 86400`. What's left after the full days is `seconds % 86400`; get the hours out of *that* with `// 3600`, and so on.
@@solution
seconds = 200000

days = seconds // 86400
rest = seconds % 86400
hours = rest // 3600
rest = rest % 3600
minutes = rest // 60
secs = rest % 60

print(days, "days,", hours, "hours,", minutes, "minutes,", secs, "seconds")

@@exercise Bandwidth math
A file of `750` megabytes (MB) downloads over a `100` megabit per second (Mbps) connection. There are **8 bits in a byte**. Print how many seconds the download takes, as a float rounded to 1 decimal place:

```text
60.0
```
@@starter
size_mb = 750
speed_mbps = 100

# megabytes -> megabits, then divide by the speed

@@expected
60.0
@@hint
Size in megabits is `size_mb * 8`. Time is megabits divided by megabits per second. Wrap it in `round(..., 1)`.
@@solution
size_mb = 750
speed_mbps = 100

seconds = size_mb * 8 / speed_mbps
print(round(seconds, 1))
@@text
> Bits vs bytes trip up everyone. Internet speeds are in **bits** (Mbps, lowercase b), file sizes in **bytes** (MB, uppercase B). Divide a speed by 8 to get MB per second. This exact calculation comes up in help desk calls ("why is my 100 Mb connection only downloading at 12 MB/s?") and on the CompTIA exams.
