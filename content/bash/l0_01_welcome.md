@@lesson l0-welcome Welcome to the Linux Command Line
@@concepts hello
@@text
Most of the internet runs on **Linux**: web servers, cloud machines on AWS and Azure, Docker containers, routers, Android. If you want a job in servers, cloud, DevOps or security, the command line is not optional. It's also most of **Linux+**, and a big part of every cloud certification.

The program you type commands into is a **shell**. The most common one is **Bash**. A Bash **script** is just a file of the same commands.

## How this course runs Bash

On Windows, the examples here run in **Git Bash**, which comes with Git for Windows and has the real Bash plus the standard tools (`grep`, `sed`, `awk`, `find`, `sort`...). Everything in the scripting lessons works the same on a real Linux server. Commands that need a real Linux system (`systemctl`, `apt`, `ip`) are shown but not run: try those on a Linux machine. Free options: **WSL** (`wsl --install` in an admin PowerShell gives you Ubuntu inside Windows), a VirtualBox VM, or a free-tier cloud server.

## First commands

```bash
echo "Hello, Linux!"
whoami
date +%Y-%m-%d
pwd
```

- `echo` prints its arguments.
- `whoami` shows your username, `pwd` the folder you're in ("print working directory").
- `date +%Y-%m-%d` shows how **options** work: the command, then its arguments.

## Anatomy of a command

```text
ls  -l  -a  /etc
│   └─ options ┘  └─ argument
command
```

Options usually start with `-` (short: `-l`) or `--` (long: `--all`). Short options can be combined: `ls -la` is `ls -l -a`.

## Getting help

| Command | Shows |
|---|---|
| `man ls` | the full manual page (press `q` to quit, `/word` to search) |
| `ls --help` | a short summary |
| `type cd` | whether it's a program, a builtin or an alias |
| `which python3` | where a program lives |

Linux is **case-sensitive**: `File.txt` and `file.txt` are different files, and `LS` isn't a command.

## Keyboard skills that save hours

- **Tab** completes commands and file names. Press it constantly.
- **↑ / ↓** go through your command history; **Ctrl+R** searches it.
- **Ctrl+C** stops the running command; **Ctrl+L** clears the screen.
- `history` lists past commands; `!!` repeats the last one (`sudo !!` = "that again, as root").

@@quiz
Which statement is true?
- Linux treats `Report.txt` and `report.txt` as the same file
* Linux file names are case-sensitive
- Options must always use `--`
- `man` only works on Windows
? Unlike Windows, Linux file systems are case-sensitive.

@@quiz
You started a command that won't stop. Which key combination interrupts it?
* Ctrl+C
- Ctrl+Z then Enter
- Ctrl+Q
- Esc
? Ctrl+C sends SIGINT, which stops most programs. (Ctrl+Z *pauses* it in the background instead.)

@@quiz
What does pressing Tab do while typing a command or file name?
* Completes the name (press twice to list the options)
- Runs the command
- Inserts four spaces
- Opens the help page
? Tab completion saves typing and avoids typos.

@@drill
Which command shows the manual page for a command?
= man
? man ls; press q to quit.

@@flashcards
shell :: The program that reads your commands; Bash is the most common
pwd :: Print the current (working) directory
whoami :: Print your username
man :: Show a command's manual page
Tab :: Auto-complete commands and file names
Ctrl+R :: Search your command history
sudo :: Run a command as root (administrator)

@@exercise Hello, server
Print exactly two lines: `Hello from Bash` and `Ready to automate`.
@@starter
# use echo twice

@@expected
Hello from Bash
Ready to automate
@@hint
`echo "Hello from Bash"` and the same for the second line.
@@solution
echo "Hello from Bash"
echo "Ready to automate"

@@exercise Variables in a message
A variable `host` is given. Print `Connecting to web01...` by using the variable inside double quotes.
@@starter
host="web01"

@@expected
Connecting to web01...
@@requires
$host
@@hint
`echo "Connecting to $host..."`. In Bash there are **no spaces** around `=` when assigning.
@@solution
host="web01"
echo "Connecting to $host..."
