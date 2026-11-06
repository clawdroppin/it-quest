@@lesson x0-welcome Welcome to C++
@@concepts hello, variables
@@text
## Why C++?

C++ is the language under most other software. Windows, Linux tools, browsers (Chrome, Firefox), databases (MySQL, MongoDB), game engines, network equipment firmware, trading systems and machine-learning libraries are written in C or C++. In IT you'll meet it when you:

- build or debug software from source on Linux (`make`, `cmake`, compiler errors);
- work in **embedded** or **manufacturing** (very common in Japan: cars, factory robots, electronics);
- need speed: C++ compiles to machine code with no garbage collector.

It's harder than Python or C#: you manage memory yourself, and some mistakes cause **undefined behaviour** instead of a clear error. Learn it slowly and carefully, and you'll understand how computers really work.

## Your first program

```cpp
#include <iostream>

int main() {
    std::cout << "Hello, world!\n";
    std::cout << "こんにちは C++" << '\n';
    return 0;
}
```

- `#include <iostream>` brings in input/output from the **standard library**.
- Every program starts in `int main()`. Returning `0` means success (it's your exit code).
- `std::cout << x` sends `x` to the console. `std::` is the namespace of the standard library.
- `'\n'` is a new line. `std::endl` does the same and also flushes the output, which is slower.

## Compiling

C++ source must be **compiled** into an executable before it runs. The app does this for you with whatever compiler it finds: MSVC (`cl`, from Visual Studio) on Windows, or `g++`/`clang++` on Linux.

```bash,norun
g++ -std=c++20 -Wall -Wextra -o hello hello.cpp    # Linux / MinGW
./hello
cl /EHsc /std:c++20 /W4 hello.cpp                   # Windows, in a "Developer Command Prompt"
hello.exe
```

Always turn warnings on (`-Wall -Wextra`, or `/W4`) and read them. In C++ a warning is very often a real bug.

## Variables and types

```cpp
#include <iostream>
#include <string>

int main() {
    std::string host = "web01";
    int port = 443;
    double cpu = 73.5;
    bool online = true;
    char grade = 'A';

    std::cout << host << ':' << port << " cpu=" << cpu << "% online=" << online << " grade=" << grade << '\n';

    auto cores = 8;          // auto: the compiler works out the type (int)
    const int maxUsers = 50; // const: can't change
    cores *= 2;
    std::cout << cores << " cores, max " << maxUsers << " users\n";
}
```

`bool` prints as `1`/`0`; write `std::cout << std::boolalpha` once to get `true`/`false`. (`main` without a `return` returns 0 automatically.)

**Always give variables a value.** `int x;` inside a function holds whatever garbage was in memory. Reading it is undefined behaviour: the program might print nonsense, or work by luck and fail later. Use `int x = 0;` or `int x{};`.

## Reading compiler errors

```cpp,norun
#include <iostream>
int main() {
    std::cout << "missing semicolon"
    return 0;
}
```

```text
main.cpp(3): error C2143: syntax error: missing ';' before 'return'     (MSVC)
main.cpp:3:37: error: expected ';' before 'return'                      (g++)
```

C++ error messages, especially about templates, can be pages long. Read the **first** error, find its line, fix it, and compile again.

@@quiz
What does `return 0;` at the end of `main` mean?
* The program finished successfully (exit code 0)
- Print 0
- Restart the program
- Nothing: it's required syntax only
? The return value of `main` is the process exit code. Scripts and CI pipelines check it.

@@quiz
`int count;` inside a function, then `std::cout << count;`. What happens?
* Undefined behaviour: it may print any value
- It prints 0
- A compile error, always
- It throws an exception
? Local variables of built-in types aren't initialized. Compilers often warn about it. Always initialize.

@@quiz
Why compile with `-Wall -Wextra` or `/W4`?
* Warnings point out likely bugs that still compile
- It makes the program faster
- It's required to compile C++20
- It removes warnings
? Many C++ bugs compile fine. Warnings are free bug reports.

@@drill
Which header gives you `std::cout`?
= <iostream>
= iostream
? #include <iostream>.

@@flashcards
`#include <iostream>` :: bring in console input/output
`int main()` :: where every C++ program starts
`std::cout << x` :: print x
`'\n'` :: new line (prefer it to `std::endl`)
`auto` :: let the compiler infer the type
`g++ -std=c++20 -Wall -o app main.cpp` :: compile with g++
undefined behaviour :: code the language gives no meaning to: anything can happen

@@exercise Server card
Print exactly these lines, using variables `host` (a `std::string`), `ip` and `ram`:

```text
Host: app01
IP:   10.0.0.15
RAM:  16 GB
```
@@starter
#include <iostream>
#include <string>

int main() {
    std::string host = "app01";
    // add ip and ram, then print three lines
}
@@expected
Host: app01
IP:   10.0.0.15
RAM:  16 GB
@@requires
std::cout
@@hint
`std::cout << "RAM:  " << ram << " GB\n";`
@@solution
#include <iostream>
#include <string>

int main() {
    std::string host = "app01";
    std::string ip = "10.0.0.15";
    int ram = 16;
    std::cout << "Host: " << host << '\n';
    std::cout << "IP:   " << ip << '\n';
    std::cout << "RAM:  " << ram << " GB\n";
}

@@exercise Fix the build
Fix the errors so the program prints `Backups kept: 7`.
@@starter
#include <iostream>

int main() {
    int days = 7
    std::cout << "Backups kept: " << Days << '\n';
}
@@expected
Backups kept: 7
@@hint
A statement is missing `;`, and C++ names are case-sensitive.
@@solution
#include <iostream>

int main() {
    int days = 7;
    std::cout << "Backups kept: " << days << '\n';
}
