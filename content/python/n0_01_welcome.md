@@lesson n0-welcome Welcome to Python
@@concepts hello
@@text
Welcome to the **Python** track! Python is the most useful first language for a career in IT:

- **Automation**: help desk and sysadmin teams use it to create user accounts, clean up disks, rename thousands of files, check that servers are alive.
- **Security**: most security tools and scripts (scanners, log analyzers, exploit proofs) are written in Python.
- **Cloud and DevOps**: AWS, Azure and Google Cloud all have Python libraries; tools like Ansible are written in it.
- **Data**: reports, spreadsheets, dashboards and machine learning.

Python reads almost like English, so you can focus on *ideas* instead of punctuation.

## How this track works

Every lesson has explanations, small **examples** you can run with **▶ Try it**, quick **quizzes**, and **practice** exercises on the right. You write code in the editor, press **▶ Run** to see what it does, and **✔ Check** to have the app test it.

The track goes **Novice → Intermediate → Advanced → Expert**. Lessons only use what earlier lessons taught, so go in order. Bigger projects (a log analyzer, a network scanner and more) live in the **🛠 Projects** tab.

If you get stuck, press **🤖 AI Helper** at the top. It sees your lesson, your code and your error, and gives hints rather than answers.

## Your first program

Here is a complete Python program. Press **▶ Try it**:

```python
print("Hello, IT world!")
```

`print` is a **function**: a named piece of code that does a job. Its job is to show text on the screen. The text goes between the brackets, inside quotes.

A program can have many lines. Python runs them **top to bottom**, one after another:

```python
print("Starting system check...")
print("Disk: OK")
print("Network: OK")
print("All checks passed.")
```

@@quiz
In what order does Python run the lines of a program?
- In a random order
- Bottom to top
* Top to bottom, one after another
- All at the same time
? Python reads your program like a recipe: line 1, then line 2, and so on.

@@exercise Say hello
Write a program that prints exactly:

```text
Hello, IT Quest!
I am learning Python.
```
@@starter
# Write your two print lines below.

@@expected
Hello, IT Quest!
I am learning Python.
@@hint
You need two lines, each with `print("...")`. Spelling, capitals and the punctuation must match exactly.
@@solution
print("Hello, IT Quest!")
print("I am learning Python.")
