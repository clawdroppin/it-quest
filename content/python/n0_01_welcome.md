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
## What Python is used for in IT

Python is the most common language for **automation** in IT jobs: renaming thousands of files, checking which servers are up, reading log files, calling cloud APIs, generating reports from spreadsheets, and gluing tools together. Network engineers use it to configure routers, security analysts to sift through alerts, data engineers to move data around, and developers to build web services. Ansible, the AWS and Azure command-line tools, and many monitoring systems are written in Python or are scripted with it.

It's popular because the code reads almost like English, there's a library for nearly everything, and the same script runs on Windows, macOS and Linux.

## How to learn it well

- **Type the examples yourself** rather than only reading them. Your fingers learn the syntax faster than your eyes do.
- **Change things and run again.** What happens if you print a number instead of text? If you remove the quotes? Breaking code on purpose is how you learn what each part does.
- **Read the error messages.** They look scary at first, but the next lesson shows they're mostly telling you exactly what's wrong and where.
- **Little and often** beats long sessions: twenty minutes a day builds the habit.

@@quiz
In what order does Python run the lines of a program?
- In a random order
- Bottom to top
* Top to bottom, one after another
- All at the same time
? Python reads your program like a recipe: line 1, then line 2, and so on.

@@quiz
What does `print("Hello")` do?
* Shows the text Hello on the screen
- Sends Hello to a printer
- Saves Hello to a file
- Creates a variable called Hello
? print displays its value in the output.

@@quiz
Why does `print(Hello)` (without quotes) fail?
* Without quotes Python looks for a variable named Hello, which doesn't exist
- print only works with numbers
- Python doesn't allow capital letters
- The brackets are wrong
? Quotes make text; a bare word is a name.

@@drill
Which function prints text to the screen?
= print
= print()
? print("Hello").

@@flashcards
print() :: show a value in the output
"text" :: a string: text in quotes
# comment :: a note for humans; Python ignores it
program :: instructions run from top to bottom

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
