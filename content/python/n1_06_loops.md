@@lesson n1-loops Loops: while and for
@@concepts loops
@@text
Automation is mostly **repetition**: do this for every user, every server, every file, every line of a log. Loops repeat code.

## for loops with range

`for` repeats once for each item in a sequence. `range(n)` gives the numbers `0` to `n - 1`:

```python
for i in range(5):
    print("ping attempt", i)
```

`range(start, stop)` and `range(start, stop, step)` give more control. The stop number is **never** included:

```python
for n in range(1, 6):
    print(n, end=" ")
print()
for n in range(10, 0, -3):
    print(n, end=" ")
print()
```

(`end=" "` makes `print` end with a space instead of a new line.)

## Looping over a string

`for` works on anything you can step through, like the characters of a string:

```python
for ch in "DNS":
    print(ch)
```

## Accumulating results

A very common pattern: start a variable before the loop, update it inside:

```python
total = 0
for day in range(1, 8):
    total += day * 10    # 10 GB logs on day 1, 20 on day 2, ...
print("Logs this week:", total, "GB")
```

## while loops

`while` repeats **as long as** its condition is true. Use it when you don't know in advance how many times:

```python
free_gb = 50
day = 0
while free_gb > 0:
    free_gb -= 12
    day += 1
print(f"Disk full after {day} days")
```

Make sure something inside the loop eventually makes the condition false, or it loops **forever**. (In this app a runaway program is stopped after a few seconds.)

## break and continue

- `break` leaves the loop immediately.
- `continue` skips to the next round.

```python
for port in range(20, 30):
    if port == 23:
        print("23 is telnet: skipping, it's insecure")
        continue
    if port == 26:
        print("stopping scan")
        break
    print("scanning port", port)
```

## Nested loops

A loop inside a loop runs the inner one completely for each round of the outer one:

```python
for rack in range(1, 3):
    for slot in range(1, 4):
        print(f"rack{rack}-slot{slot}", end="  ")
    print()
```

@@quiz
How many times does `for i in range(2, 8):` run?
- 8
- 7
* 6
- 5
? It produces 2, 3, 4, 5, 6, 7: six numbers. The stop value 8 is not included.

@@quiz
What's the difference between `break` and `continue`?
* `break` ends the whole loop; `continue` skips only the rest of the current round
- They do the same thing
- `continue` ends the loop; `break` skips one round
- `break` only works in while loops
? `continue` jumps to the next round; `break` jumps out of the loop.

@@exercise Retry with backoff
When a connection fails, good scripts wait longer before each retry: 1 second, then 2, 4, 8... (doubling). Print the wait time for 6 retries like this:

```text
retry 1: wait 1s
retry 2: wait 2s
retry 3: wait 4s
retry 4: wait 8s
retry 5: wait 16s
retry 6: wait 32s
total wait: 63s
```
@@starter
wait = 1
total = 0

@@expected
retry 1: wait 1s
retry 2: wait 2s
retry 3: wait 4s
retry 4: wait 8s
retry 5: wait 16s
retry 6: wait 32s
total wait: 63s
@@requires
for
@@hint
`for attempt in range(1, 7):` — inside, print, add `wait` to `total`, then double `wait`.
@@solution
wait = 1
total = 0
for attempt in range(1, 7):
    print(f"retry {attempt}: wait {wait}s")
    total += wait
    wait *= 2
print(f"total wait: {total}s")
@@text
> This is called **exponential backoff**. Cloud APIs (AWS, Azure) expect clients to do exactly this when they're told to slow down.

@@exercise Find the first free IP
The addresses `192.168.1.10` to `192.168.1.13` are taken. Read a starting number from the input, and starting there, count upward until you reach a number that's **not** in the taken range (10–13). Print the address. For the input `11`:

```text
first free: 192.168.1.14
```
@@stdin
11
@@starter
n = int(input())

@@expected
first free: 192.168.1.14
@@requires
while
@@hint
`while 10 <= n <= 13:` add 1 to `n`. After the loop, `n` is free.
@@solution
n = int(input())
while 10 <= n <= 13:
    n += 1
print(f"first free: 192.168.1.{n}")

@@exercise FizzBuzz, the interview classic
Print the numbers 1 to 15, but for multiples of 3 print `Fizz`, for multiples of 5 print `Buzz`, and for multiples of both print `FizzBuzz`. One per line.
@@starter
for n in range(1, 16):
    print(n)
@@expected
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
@@hint
Check "multiple of both" first: `n % 15 == 0`. Then `n % 3 == 0`, then `n % 5 == 0`.
@@solution
for n in range(1, 16):
    if n % 15 == 0:
        print("FizzBuzz")
    elif n % 3 == 0:
        print("Fizz")
    elif n % 5 == 0:
        print("Buzz")
    else:
        print(n)
