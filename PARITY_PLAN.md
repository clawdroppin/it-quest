# Course parity plan

Goal: every programming and scripting track reaches the depth of the Rust track. Modules that go beyond IT work carry the label **🎓 Beyond IT** in the module title and a note at the top of each lesson.

## Where things stand (November 2026)

| Track | Lessons | Exercises | Quizzes | Words | vs Rust (words) |
|---|---:|---:|---:|---:|---:|
| Rust | 106 | 179 | 254 | 175k | 100% |
| Python | 84 | 155 | 216 | 103k | 59% (done, see below) |
| PowerShell | 46 | 90 | 109 | 58k | 33% (done, see below) |
| Bash | 45 | 85 | 115 | 55k | 32% (done, see below) |
| JavaScript | 55 | 106 | 153 | 83k | 47% (done, see below) |
| SQL | 40 | 109 | 110 | 62k | 35% (done, see below) |
| C# | 40 | 76 | 112 | 60k | 34% (done, see below) |
| C++ | 40 | 75 | 111 | 61k | 35% (done, see below) |
| Go | 39 | 74 | 113 | 69k | 39% (done, see below) |
| HTML/CSS | 38 | 86 | 135 | 61k | 35% (done, see below) |
| TypeScript | 39 | 71 | 116 | 75k | 43% (done, see below) |

## The Rust template

Each track should cover these areas:

1. **Getting started:** what a program is, the toolchain, the first program, reading errors. Several short lessons.
2. **Fundamentals:** one lesson per concept, about 10 in all.
3. **Collections,** finishing with a module project.
4. **The language's own memory or execution model:**
   - Rust: ownership.
   - Python, JavaScript, C#: references, mutability and garbage collection.
   - C++: pointers and lifetimes.
   - Go: pointers and escape analysis.
   - Bash: processes and subshells.
   - PowerShell: the pipeline and objects.
   - SQL: sets and NULL.
   - HTML/CSS: the cascade and layout.
5. **Modeling data:** types, classes, records, enums, pattern matching.
6. **Errors and input.**
7. **Organizing code:** packages, tooling, tests.
8. **Abstraction:** interfaces, generics, closures, iterators.
9. **Real programs:** files, data formats, architecture.
10. **GUI capstone:** a multi-lesson desktop or web app.
11. **Advanced language:** concurrency, async, metaprogramming, interop, performance.
12. **Real world:** CLI, networking, web, databases, packaging, advanced testing, observability.
13. **🎓 Beyond IT,** where it applies.
14. **Guided builds:** four or five multi-part builds.

Every lesson has text, 3–4 quizzes, flashcards and two verified exercises. Do not rename existing lesson ids or reorder exercises. New lessons are added next to the existing ones.

## Planned additions per track (about 40–55 new lessons each)

- **Python**
  - More granular fundamentals: booleans and None, while vs for, scope, comprehensions.
  - Object model: references, mutability, copying, dunder methods, inheritance, properties, ABCs and Protocols.
  - Functional tools: itertools and functools.
  - Advanced:
    - descriptors and metaclasses
    - deep asyncio
    - multiprocessing
    - ctypes and C extensions
    - profiling and performance
  - Real world:
    - FastAPI and Flask
    - SQLAlchemy
    - packaging with pyproject
    - pytest in depth
    - logging and observability
  - GUI capstone: Tkinter.
  - 🎓 Beyond IT: data science (NumPy and pandas), machine learning basics, games with pygame.

- **JavaScript**
  - More granular fundamentals.
  - Scope, closures, `this` and prototypes.
  - The event loop in depth.
  - Iterators and generators, modules.
  - Node streams, worker threads.
  - Testing with node:test.
  - React basics.
  - Electron or Tauri GUI capstone.
  - Performance and security.
  - 🎓 Beyond IT: canvas games, three.js.

- **TypeScript**
  - Type system in depth: mapped, conditional and template-literal types, `infer`, variance, branded types.
  - Declaration files.
  - Decorators.
  - Build tooling.
  - React with TypeScript.
  - Node API with validation.
  - Testing.
  - GUI capstone.

- **Go**
  - More granular fundamentals.
  - Pointers and escape analysis.
  - Interfaces and generics in depth.
  - Error wrapping.
  - Concurrency patterns: select, context, sync, the race detector.
  - Testing, benchmarks and fuzzing.
  - CLI with cobra.
  - database/sql.
  - gRPC.
  - pprof.
  - Reflection, unsafe and cgo.
  - GUI capstone with Fyne.
  - 🎓 Beyond IT: TinyGo, Ebitengine games.

- **C#**
  - More granular fundamentals.
  - Value vs reference types, GC, Span.
  - OOP in depth.
  - Generics.
  - Delegates and events.
  - LINQ in depth.
  - Pattern matching.
  - Nullable references.
  - async in depth.
  - xUnit, DI, EF Core, minimal APIs, Blazor.
  - Reflection, attributes, source generators.
  - Performance.
  - WPF/MAUI capstone.
  - 🎓 Beyond IT: Unity.

- **C++**
  - More granular fundamentals.
  - References vs pointers.
  - Inheritance and virtual functions.
  - Operator overloading.
  - The STL in depth.
  - constexpr and concepts.
  - Move semantics in depth.
  - Atomics and memory order.
  - CMake, vcpkg and Conan.
  - GoogleTest and Catch2.
  - Sanitizers.
  - Asio networking.
  - Qt capstone.
  - 🎓 Beyond IT: game development (SDL/raylib), Arduino, SIMD.

- **Bash**
  - More granular fundamentals.
  - Arrays and parameter expansion.
  - Here-docs and process substitution.
  - Signals and job control.
  - Regex with `=~`.
  - curl and jq.
  - Text UI capstone (whiptail).
  - Performance and portability (POSIX sh).

- **PowerShell**
  - More granular fundamentals.
  - Providers.
  - The pipeline in depth.
  - Runspaces and `-Parallel`.
  - DSC.
  - Microsoft Graph and Azure.
  - JEA.
  - PowerShell 7 cross-platform.
  - Performance.

- **SQL**
  - NULL semantics.
  - Data types.
  - Self, outer and anti joins.
  - Set operations.
  - Views.
  - Functions, procedures and triggers.
  - Recursive CTEs.
  - JSON.
  - Full-text search.
  - Isolation levels and locking.
  - Backup and replication.
  - PostgreSQL specifics.
  - Warehousing and star schemas.

- **HTML/CSS**
  - More granular HTML elements.
  - Forms in depth.
  - Cascade and specificity.
  - Positioning.
  - Typography.
  - Custom properties.
  - Animations.
  - Container queries.
  - Accessibility in depth.
  - SEO.
  - SVG and canvas.
  - Web components.
  - Build tools.
  - 🎓 Beyond IT: creative coding.

## Rust (done in this pass)

- **Module 10, Advanced Rust (9 lessons):** advanced traits, dispatch, lifetimes, concurrency, async, macros, unsafe/FFI, performance, thread-pool project.
- **Module 11, Rust in the Real World (9 lessons):** CLI, networking, web, databases, the Cargo ecosystem, advanced testing, errors and observability, WebAssembly, key-value store project.
- **Module 12, 🎓 Beyond IT: Embedded and Game Development (2 lessons).**

## Python (done in this pass)

Python went from 39 to 84 lessons (155 exercises, 216 quizzes, about 103k words). The whole track verifies with 167 checks and 0 problems. Python is written more concisely than Rust, so it doesn't need as many words to cover the same ground.

- **Granular fundamentals (6 lessons, inserted into modules 0–2):**
  - how Python runs
  - booleans and None
  - scope and arguments
  - loop patterns
  - comprehensions
  - collections and sorting
- **Module 9, The Python Object Model (8 lessons):**
  - references and copying
  - dunder methods
  - inheritance
  - properties and classmethods
  - ABCs and Protocols
  - enums and pattern matching
  - functional tools
  - project: a formula engine
- **Module 10, Advanced Python (9 lessons):**
  - descriptors and metaclasses
  - asyncio
  - parallelism
  - performance
  - introspection
  - advanced typing
  - binary data and interop
  - memory
  - project: a job scheduler
- **Module 11, Python in the Real World (9 lessons):**
  - FastAPI
  - ORMs
  - packaging
  - pytest
  - logging
  - configuration and secrets
  - security
  - dates and times
  - project: a webhook relay
- **Module 12, GUI Capstone: Desktop Apps with Tkinter (9 lessons):**
  - basics
  - layout
  - events and validation
  - MVP and undo
  - tables and themes
  - menus, dialogs and files
  - canvas charts
  - background threads
  - project: an asset manager, packaged with PyInstaller
- **Module 13, 🎓 Beyond IT: Data Science, Machine Learning and Games (4 lessons):**
  - data analysis
  - machine learning from scratch
  - NumPy and numerical methods
  - Pygame

## JavaScript (done in this pass)

JavaScript went from 19 to 55 lessons (106 exercises, 153 quizzes, about 83k words). The whole track verifies with 114 checks and 0 problems. The exercises run under Node.js. Browser, React and Electron code is shown as examples, and the exercises test the logic behind it.

- **Granular fundamentals (7 lessons, inserted into modules 0–2):**
  - how JavaScript runs
  - types and coercion
  - scope and hoisting
  - loop patterns
  - destructuring and copying
  - Map and Set
  - dates and Intl
- **Module 7, JavaScript in Depth (9 lessons):**
  - this and binding
  - prototypes and classes
  - modules
  - iterators and generators
  - the event loop
  - metaprogramming
  - error handling
  - functional patterns
  - project: a concurrent task queue
- **Module 8, Advanced Node.js (9 lessons):**
  - streams
  - workers and child processes
  - node:test
  - binary data and crypto
  - performance and memory
  - security
  - observability
  - databases
  - project: an incremental backup tool
- **Module 9, GUI Capstone: React and Desktop Apps (8 lessons):**
  - components and the virtual DOM
  - React state and reducers
  - hooks and effects
  - forms and schemas
  - routing and server state
  - accessibility and styling
  - Electron and Tauri
  - project: an asset tracker
- **Module 10, 🎓 Beyond IT: Games, 3D and Creative Coding (3 lessons):**
  - canvas games
  - three.js and 3D maths
  - generative art and Web Audio

## TypeScript (done in this pass)

TypeScript went from 12 to 39 lessons (71 exercises, 116 quizzes, about 75k words). The whole track verifies with 77 checks and 0 problems. The runner strips types without checking them, so every new solution, test and runnable example was also type-checked separately with `tsc --strict` (plus `erasableSyntaxOnly`). Type-level exercises include `Expect<Equal<...>>` and `@ts-expect-error` type tests; they're erased at runtime, and the lessons explain how to check them in an editor. Decorators and constructor parameter properties can't run under Node's type stripping, so decorators are taught as plain functions applied by hand.

- **Granular fundamentals (4 lessons, inserted into modules 0–2):**
  - how type checking works (structural typing, any/unknown/never, strict mode)
  - object types, interfaces, index signatures and Record
  - tuples, literal types, `as const` and `satisfies`
  - function types: overloads, type predicates, assertion functions and `this`
- **Module 6, The Type System in Depth (7 lessons):**
  - generics: constraints, inference, `NoInfer`, `const` type parameters
  - mapped types and key remapping
  - conditional types and `infer`
  - template literal types
  - variance and branded types
  - recursive types and type-level programming
  - project: a type-safe query builder
- **Module 7, TypeScript in the Real World (8 lessons):**
  - declaration files and module augmentation
  - decorators
  - build tooling, tsconfig and publishing packages
  - testing with typed fakes, spies and fake clocks
  - runtime validation with inferred types
  - typed errors and Result types
  - typed routers and middleware
  - project: a typed incident service
- **Module 8, GUI Capstone: React and Tauri Desktop Apps (5 lessons):**
  - React with TypeScript
  - reducers and stores
  - data fetching and forms
  - Tauri commands, events and settings migrations
  - project: a keyboard-first asset desk
- **Module 9, 🎓 Beyond IT: Interpreters, Game Engines and Generative Art (3 lessons):**
  - a Pratt parser and interpreter
  - a typed ECS game engine
  - seeded simulations, cellular automata and L-systems

## Go (done in this pass)

Go went from 13 to 39 lessons (74 exercises, 113 quizzes, about 69k words). The whole track verifies with 80 checks and 0 problems. The runner builds with `GOTOOLCHAIN=local` and only the standard library, so third-party tools (cobra, gRPC, database drivers, Fyne, Ebitengine, TinyGo) are taught with read-only examples, and their exercises test the stdlib logic behind them: flag-based subcommands, a query builder and migrations runner, protobuf varints and wire encoding, view-models and bindings, fixed-point physics. Concurrent exercises print deterministic output: results are sorted, timing is checked as booleans, and UI updates go through a single queue.

- **Granular fundamentals (4 lessons, inserted into modules 0–2):**
  - the toolchain, constants and iota
  - pointers vs values and nil receivers
  - strings, bytes and runes (display width for CJK text)
  - slices in depth: aliasing, capacity, the slices and maps packages
- **Module 6, Go in Depth (8 lessons):**
  - interfaces in depth: embedding, io.Writer decorators, type switches
  - generics in depth
  - range-over-func iterators and iter.Pull
  - errors in depth: wrapping, errors.Join, panics and recover
  - concurrency patterns: select, pipelines, fan-out/fan-in, an errgroup with a limit
  - context, mutexes, Once/OnceValue, atomics and the race detector
  - reflection, struct tags, unsafe and cgo
  - project: a concurrent link checker with retries and backoff
- **Module 7, Production Go (7 lessons):**
  - testing: table tests, benchmarks, fuzzing, examples, synctest
  - CLIs: flag, subcommands, exit codes, config precedence, cobra
  - database/sql: queries, transactions, placeholders, migrations
  - Protocol Buffers and gRPC
  - performance: pprof, allocations, escape analysis, GOGC/GOMEMLIMIT, PGO
  - observability: slog, redaction, request logging middleware, metrics and tracing
  - project: a production HTTP service with graceful shutdown
- **Module 8, GUI Capstone: Desktop Apps with Fyne (4 lessons):**
  - apps, widgets, layouts and form validation
  - data binding, virtualized lists and view-models
  - background work on the UI thread, cancellation and debouncing
  - project: a server status dashboard with versioned settings
- **Module 9, 🎓 Beyond IT: Embedded, Games and Generative Code (3 lessons):**
  - TinyGo on microcontrollers (debouncing, fixed-point sensors) and WebAssembly
  - 2D games with Ebitengine (sub-pixel tilemap physics, pathfinding)
  - procedural generation (seeded mazes, the Game of Life)

## C# (done in this pass)

C# went from 14 to 40 lessons (76 exercises, 112 quizzes, about 60k words). The whole track verifies with 82 checks and 0 problems. The runner compiles with csc against Microsoft.NETCore.App only, so ASP.NET Core, EF Core, xUnit, Blazor, WPF/MAUI/Avalonia, CommunityToolkit.Mvvm, BenchmarkDotNet and Unity are taught with read-only examples. Their exercises rebuild the BCL logic behind them: a reflection-based test runner, a DI container, a change tracker and expression visitors, a route matcher, Problem Details JSON, an EditContext, a binding engine, collection views, a SynchronizationContext pump and a game-loop lifecycle. Concurrent exercises stay deterministic: they use TaskCompletionSource gates, injected TimeProvider clocks and grouped output.

- **Granular fundamentals (4 lessons, inserted into modules 0–2):**
  - the .NET toolchain, project files, argument parsing and [Flags] enums
  - strings in depth: StringBuilder, formatting, Unicode text elements
  - value vs reference types, the GC and IDisposable
  - generics and collections in depth: an LRU cache, generic math with INumber<T>
- **Module 6, C# in Depth (8 lessons):**
  - OOP in depth (template method, decorators)
  - delegates and events
  - LINQ internals and custom operators
  - pattern matching
  - nullable reference types and a Result<T> type
  - async in depth (SemaphoreSlim, cancellation, Channels)
  - reflection and attributes
  - project: a reflection-discovered job runner
- **Module 7, Production .NET (7 lessons):**
  - testing with xUnit and fakes
  - DI and layered configuration
  - EF Core change tracking and expression trees
  - minimal APIs and Problem Details
  - Blazor components and forms
  - high-performance C# (Span, stackalloc, ArrayPool, allocation measurement)
  - project: a worker service with graceful shutdown and health checks
- **Module 8, Desktop Apps in Depth (4 lessons):**
  - XAML data binding and a mini binding engine
  - async commands, navigation and messaging
  - live collection views and the UI thread
  - project: an asset desk app with INotifyDataErrorInfo and coalescing undo/redo
- **Module 9, 🎓 Beyond IT: Game Development (3 lessons):**
  - Unity's lifecycle and a fixed-timestep accumulator
  - coroutines as iterators and enemy AI state machines
  - Godot/MonoGame, seeded dungeon generation and A* pathfinding

## C++ (done in this pass)

C++ went from 14 to 40 lessons (75 exercises, 111 quizzes, about 61k words). The whole track verifies with 81 checks and 0 problems. The runner compiles with MSVC `/std:c++20` or g++ `-std=c++20`, so C++23 features (`std::expected`, `std::print`, `std::generator`) are shown as read-only examples. Exercises avoid anything whose output differs between standard libraries: unordered iteration order, `*_distribution` classes, timings and vector growth. Qt, Dear ImGui, Asio, GoogleTest, CMake, raylib/SDL and Arduino are taught with read-only examples. Their exercises rebuild the mechanisms behind them: signals/slots, the QObject tree, proxy models, QML bindings, an immediate-mode UI, an event loop, a test framework, frame and HTTP parsers, an ECS, and GPIO registers.

- **Granular fundamentals (4 lessons, inserted into modules 0–2):**
  - how C++ is built (preprocessor, compiler, linker), argv parsing, fixed-width integers and overflow
  - references, pointers, const and std::span (the IP checksum)
  - text in depth: string_view, std::format, from_chars, UTF-8
  - choosing containers, custom hashes, priority queues
- **Module 6, C++ in Depth (8 lessons):**
  - special members and the rule of zero/five, the spaceship operator
  - virtual functions, std::variant/visit and CRTP
  - move semantics and perfect forwarding
  - variadic templates, if constexpr, concepts and a compile-time CRC table
  - exception safety and an expected type
  - condition variables, scoped_lock, atomics and jthread
  - ranges and a generator coroutine
  - project: a thread pool with futures and parallel map-reduce
- **Module 7, Production C++ (7 lessons):**
  - CMake, vcpkg/Conan, topological build order and version resolution
  - GoogleTest/Catch2 internals and property tests
  - undefined behaviour, sanitizers and checked containers
  - network framing and an incremental HTTP parser
  - pmr allocators and data-oriented design
  - C APIs, callbacks and language bindings
  - project: a log shipper with batching, backoff and a circuit breaker
- **Module 8, Desktop GUIs: Qt and Dear ImGui (4 lessons):**
  - signals and slots and the object tree
  - model/view proxies and QML-style bindings
  - immediate-mode UI and the ID stack
  - project: a server monitor event loop and INI settings
- **Module 9, 🎓 Beyond IT: Games, Embedded and SIMD (3 lessons):**
  - a sparse-set ECS and AABB collisions
  - GPIO registers, an ISR ring buffer and debouncing
  - SWAR byte counting and bitmap indexes with popcount

## Bash (done in this pass)

Bash went from 23 to 45 lessons (85 exercises, 115 quizzes, about 55k words; the counts include the three build projects). The whole track verifies with 93 checks and 0 problems. Git Bash has no jq, bc, whiptail or python3, so these tools are taught with read-only examples. Exercises use pure Bash and awk instead (a flat-JSON reader and writer, `select` menus, key decoding). Sorted output uses `LC_ALL=C`, and timings go to stderr, so the expected output is the same on every machine.

- **Granular fundamentals (4 lessons, inserted into modules 1–2):**
  - parameter expansion: defaults, trimming, replacement, case and indirection
  - arrays in depth: mapfile, read -a, associative arrays and namerefs
  - redirection in depth: file descriptors, the order of redirections, here-docs, process substitution, exec
  - regular expressions: grep -E, sed -E, `[[ =~ ]]` and BASH_REMATCH validators
- **Module 7, Bash in Depth (6 lessons):**
  - printf formatting and safe `read` (NUL-separated data)
  - functions: return values, dynamic scope, libraries and stack traces
  - signals, traps, job control and `wait -n` concurrency limits
  - subshells, PIPESTATUS, pipefail and replacing eval
  - awk as a programming language (joins, arrays, functions)
  - project: a task runner with topological ordering
- **Module 8, Production Bash (6 lessons):**
  - curl, retries with backoff, and JSON (jq, plus a pure-Bash reader and writer)
  - portability: POSIX sh, dash, BusyBox, macOS, GNU vs BSD tools
  - performance: counting forks, builtins vs external commands, awk for bulk work
  - testing: Bats, a small test framework, mocking commands
  - security: injection, option injection, validation, mktemp, umask, atomic writes
  - project: a release script with Conventional Commits, a changelog and a dry run
- **Module 9, Terminal UIs (3 lessons):**
  - ANSI escapes, tput, NO_COLOR, boxes, bars and sparklines
  - select menus, key decoding and whiptail
  - project: a full-screen service dashboard (render, scrolling, an event loop)
- **Module 10, 🎓 Beyond IT: Games, Generative Art and Interpreters (3 lessons):**
  - Game of Life and 2048
  - a fixed-point Mandelbrot set and a maze from a seeded random generator (an LCG)
  - an RPN calculator and a Brainf*ck interpreter

## PowerShell (done in this pass)

PowerShell went from 24 to 46 lessons (90 exercises, 109 quizzes, about 58k words; the counts include the build projects). The whole track verifies with 98 checks and 0 problems. The runner is Windows PowerShell 5.1, so exercises avoid PS7-only syntax, and PS7 features are taught with read-only examples. Exercises print exception type names or error ids instead of localized messages, and format numbers with the invariant culture, so the expected output is the same on non-English Windows.

- **Granular fundamentals (4 lessons, inserted into modules 1–3):**
  - operators in depth: comparison, collection filtering, -like/-match, -f and splatting
  - script blocks, closures, scopes and dot-sourcing
  - the pipeline in depth: ValueFromPipeline, begin/process/end and Select-Object tricks
  - dates, time zones and culture
- **Module 7, PowerShell in Depth (6 lessons):**
  - the six output streams, redirection and Write-Information
  - .NET collections: List, Dictionary, HashSet, Queue and StringBuilder
  - errors in depth: terminating vs non-terminating, ErrorRecord, $ErrorActionPreference
  - parallel work with runspace pools (and ForEach-Object -Parallel in PS7)
  - classes in depth: interfaces, IComparable, static members and Flags enums
  - project: a configuration drift detector with Test/Set and ShouldProcess
- **Module 8, Production PowerShell (6 lessons):**
  - testing: Pester, a small test framework and mocks
  - performance: arrays vs lists, the pipeline cost, hashtable lookups
  - security: execution policy, AST scanning for risky code, validation, Export-Clixml secrets
  - PowerShell 7 and cross-platform scripts: encodings and portable helpers
  - logging: JSON lines, rotation and exit codes
  - project: an access review report with rules and an HTML report
- **Module 9, WPF Apps with MVVM (3 lessons):**
  - XAML, FindName, events and Grid layout
  - data binding, INotifyPropertyChanged, ObservableCollection and RelayCommand
  - project: a help desk ticket app tested headlessly
- **Module 10, 🎓 Beyond IT: Puzzles, Graphics and Sound (3 lessons):**
  - Sudoku with backtracking and the MRV heuristic; N-Queens
  - a System.Drawing plasma bitmap and SVG spirographs
  - note frequencies and writing a WAV file byte by byte

## SQL (done in this pass)

SQL went from 16 to 40 lessons (109 exercises, 110 quizzes, about 62k words; the counts include the two build projects). The whole track verifies with 115 checks and 0 problems, and every runnable example in the new lesson text runs against the company database. Exercises run on Python's SQLite, so PostgreSQL, SQL Server and MySQL features are taught with read-only examples and the exercises use their portable SQLite equivalents (FILTER, ROW_NUMBER for DISTINCT ON, sorted subqueries for string_agg). The runner stops at the first SQL error, so rules that reject changes are shown with `RAISE(IGNORE)` and waitlist tables instead of failing statements.

- **Granular fundamentals (6 lessons, inserted into modules 1–3):**
  - NULL and three-valued logic: COALESCE, NULLIF, NULLs in aggregates, the NOT IN trap
  - data types, SQLite affinity and STRICT, integer division, CAST and rounding
  - joins in depth: self joins, ON vs WHERE in outer joins, semi and anti joins, CROSS JOIN, fan-out
  - set operations: UNION (ALL), INTERSECT, EXCEPT, VALUES, comparing tables
  - recursive CTEs: hierarchies, date series, graphs and cycle guards
  - views, temporary tables, materialised views and generated columns
- **Module 7, SQL in Depth (6 lessons):**
  - window frames (ROWS/RANGE), LAG/LEAD, NTILE, named windows, gaps and islands
  - JSON: json_extract, json_each, building JSON, indexing, PostgreSQL jsonb
  - text search: LIKE and collations, FTS5 with porter and trigram tokenizers, highlight
  - dates and time zones: instants vs civil time, modifiers, business-day calendars, fiscal years
  - constraints, foreign key actions, partial unique indexes, triggers and INSTEAD OF views
  - project: a help desk SLA report built from layered CTEs
- **Module 8, Production Databases (6 lessons):**
  - transactions: anomalies, isolation levels, MVCC, lost updates, deadlocks, savepoints, SKIP LOCKED queues
  - query tuning: sargable predicates, composite and covering indexes, redundant indexes, N+1
  - security: least privilege, row-level security, masking, auditing, anonymisation
  - PostgreSQL, SQL Server and MySQL differences and portable SQL
  - operations: backups and PITR, replication, migrations, expand/contract, health checks
  - project: a software licence compliance database with a seat-limit trigger
- **Module 9, Analytics and Data Warehousing (3 lessons):**
  - star schemas, grain, date dimensions, semi-additive measures
  - ETL/ELT: staging, cleaning, deduplication, incremental loads, SCD type 2, data quality gates
  - project: funnels, cohort retention and power users from an event table
- **Module 10, 🎓 Beyond IT: Fractals, Graphs and Statistics (3 lessons):**
  - Sierpinski, Mandelbrot and Rule 90 drawn with recursive CTEs
  - shortest paths, reachability and the Game of Life in SQL
  - medians, percentiles, histograms and linear regression

## HTML/CSS (done in this pass)

HTML/CSS went from 13 to 38 lessons (86 exercises, 135 quizzes, about 61k words; the counts include the two build projects). The whole track verifies with 92 checks and 0 problems. The HTML checker only lints tag balance and `@@requires`, so every new solution was also loaded in a browser and checked for script errors. The only failures were expected 404s for placeholder media files.

- **Granular fundamentals (7 lessons, inserted into modules 1–3):**
  - text semantics
  - media
  - forms in depth
  - the cascade and specificity
  - selectors
  - typography
  - positioning and stacking
- **Module 6, CSS in Depth (6 lessons):**
  - grid areas, auto-fit/minmax and subgrid
  - transitions, transforms, keyframes and reduced motion
  - container queries, logical properties, nesting and popover
  - design tokens, BEM, utilities and cascade layers
  - SVG icons, sprites and accessible charts
  - project: an operations dashboard layout
- **Module 7, Production Front End (6 lessons):**
  - ARIA tabs, dialogs, disclosures and live regions
  - Core Web Vitals, images, fonts and loading
  - the Constraint Validation API, accessible errors and error summaries
  - validation, audits, in-page tests and a contrast checker
  - SEO, Open Graph, JSON-LD, Intl and Japanese typography (ruby, emphasis dots, line breaking)
  - project: an accessible IT request form
- **Module 8, Web Components and Tooling (3 lessons):**
  - custom elements, shadow DOM and slots
  - ES modules, import maps, npm, Vite and supply-chain security
  - project: a component-based service status board
- **Module 9, 🎓 Beyond IT: Canvas, CSS Art and Games (3 lessons):**
  - generative canvas art
  - pure-CSS illustration
  - an original arcade game (Packet Catcher)

## IT and certification tracks (second pass)

With every language track deepened, the biggest gaps are now the IT and certification tracks. In November 2026 most had 3–10 lessons, under 10k words and almost no exercises. These tracks don't need Rust's length; they need full coverage of their exam objectives or job skills. The target for each:

- every exam domain (or job area) has its own lesson, typically 15–25 lessons per track
- every lesson has 4+ quizzes, flashcards, drills and two verified exercises (Python, PowerShell or Bash, simulating the real task: subnet calculators, log triage, permission checks, cost estimates)
- 25k+ words for the big certifications (A+, Network+, Security+, CCNA, AWS, Azure)

Order: Foundations first (the entry point), then A+, Network+, Security+, Linux+, CCNA, then cloud, then management, then Japan.

### Foundations (done in this pass)

Foundations went from 6 to 20 lessons, from 1 to 29 exercises, and from 24 to 83 quizzes (about 28k words, up from 5k). The whole track verifies with 32 checks and 0 problems. New lessons:

- **Computers:** CPUs, cores and the memory hierarchy; ports, docks, displays, printers and mobile devices (MDM, BYOD); text encodings and mojibake, image formats, file types and hashes.
- **Networks:** IPv4 subnetting, private ranges, APIPA and IPv6; switches, VLANs, PoE, cabling and Wi-Fi; the web request chain, HTTP status codes, certificates and the email path (SPF, DKIM, DMARC).
- **Working in IT:** users, groups, NTFS and Linux permissions, and the joiner/mover/leaver process; the Windows and Linux command line; backups (3-2-1-1-0, RPO and RTO); incident, problem and change management; careers and certifications, including Japan.
- **New module 3, Data, Software and Automation:** databases and data formats; how software is built, versioned and licensed; scripting IT tasks safely.

### CompTIA A+ (done in this pass)

A+ went from 10 to 22 lessons, from 0 to 24 exercises, and from 51 to 109 quizzes (about 32k words, up from 11k). The whole track verifies with 28 checks and 0 problems. Each Core 220-1201 and 220-1202 objective area now has its own lesson:

- **Core 1:** motherboards, CPUs, RAM and power supplies; storage and RAID; mobile devices; network configuration (DHCP, DNS, Wi-Fi, SOHO); network types, servers, appliances, IoT and legacy systems; the troubleshooting method and hardware symptoms.
- **Core 2:** Windows admin tools (MMC snap-ins, Event Viewer IDs, sfc/DISM); installs and upgrades (editions, MBR/GPT, file systems, Windows 11 readiness); Windows networking and sharing (domains, profiles, share vs NTFS permissions); workstation, browser, mobile and SOHO hardening, plus data destruction; software, malware and mobile troubleshooting (the 7-step malware removal procedure); script types, script risks and remote access tools.

### CompTIA Network+ (done in this pass)

Network+ went from 8 to 19 lessons, from 1 to 23 exercises, and from about 40 to 102 quizzes (about 27k words, up from 9k). The whole track verifies with 27 checks and 0 problems. Every N10-009 domain now has depth:

- **Concepts:** cabling, fibre, transceivers and PoE budgets; DNS resolution, zones, DHCP timers and options, NTP/PTP; spine-and-leaf, SDN, SD-WAN, VXLAN, ZTNA/SASE and cloud networking.
- **Implementation:** route selection (longest prefix, AD, metric), routing protocols, NAT/PAT and FHRP; trunks, native VLANs, spanning tree, LACP, MTU and switch security; Wi-Fi standards, channel planning, 802.1X/EAP and site surveys.
- **Operations and security:** SNMP, flows, syslog severities and metrics; MTBF/MTTR, RPO/RTO, DR sites, documentation and change management; attacks and defences, ACL evaluation, segmentation and remote access (IPsec, out-of-band).
- **New module 4, Troubleshooting in Depth:** the 7-step method, interface counters, err-disabled ports and fibre loss budgets; addressing, routing, service and performance problems with the commands that reveal them.

### CompTIA Security+ (done in this pass)

Security+ went from 7 to 19 lessons, from 1 to 25 exercises, and from 46 to 106 quizzes (about 27k words, up from 8k). The whole track verifies with 29 checks and 0 problems. Content stays defensive: attacks are explained by how to recognise and stop them. Every SY0-701 domain now has depth:

- **General concepts:** the control category and type matrix, physical security, deception technology and change management; applied cryptography (TLS 1.3, signatures, salted and stretched password storage, certificate validation and revocation).
- **Threats:** social engineering, phishing indicators and email authentication (SPF, DKIM, DMARC); application vulnerabilities with secure coding fixes (parameterised queries, output encoding).
- **Architecture:** firewall types, zones, failure modes and secure protocols; shared responsibility, IaC scanning, containers, IoT and OT.
- **Operations:** CVSS, KEV and remediation SLAs; IAM life cycle, federation, MFA, access control models and PAM; incident response, order of volatility, chain of custody and timelines; SIEM correlation, SOAR, EDR and baselines.
- **Programme management:** SLE, ARO, ALE and control value; risk registers and BIA; vendor assessment, agreements, privacy roles and audits.

### Linux+ (done in this pass)

Linux+ went from 7 to 19 lessons, from 1 to 25 exercises, and from 33 to 93 quizzes (about 24k words, up from 7k). The new exercises are Bash (awk, sed, parameter expansion, associative arrays) working on realistic command output, so they run anywhere Bash does. The whole track verifies with 28 checks and 0 problems. New lessons:

- **System management:** text processing and redirection; processes, signals, cron and systemd timers; apt/dnf/rpm, repositories, kernel modules, sysctl and GRUB; df/du, inodes, fstab, LVM growth and file system repair.
- **Security:** octal permissions, umask, SUID/SGID/sticky, ACLs, attributes and least-privilege sudoers; shadow aging, SSH keys and sshd hardening, PAM, directory services and auditd; ip/nmcli/netplan, DNS, sockets and host firewalls.
- **Troubleshooting:** the USE method with vmstat, iostat, free and OOM; boot failures, systemd services, login and file access problems, and SELinux contexts.
- **New module 4, Scripting, Containers and Automation:** Bash scripting with strict mode and exit codes; Podman/Docker, Containerfiles and container security; Git, Ansible, YAML, IaC and CI/CD.

### Cisco CCNA (done in this pass)

CCNA went from 6 to 16 lessons, from 0 to 20 exercises, and from 31 to 88 quizzes (about 25k words, up from 7.5k). The exercises are Python simulations of what the devices do (MAC learning, STP elections, longest-prefix lookup, PAT, ACL evaluation, port security and DAI) and parsers for real `show` output and controller JSON. The whole track verifies with 24 checks and 0 problems. New lessons:

- **Fundamentals (module 0):** subnetting by block size, VLSM, wildcard masks and summarisation; IPv6 addressing, EUI-64, SLAAC/DHCPv6, NDP and static routes; MAC learning, interface status and errors, duplex mismatches, TCP vs UDP and virtualisation.
- **Switching and routing:** STP root and port elections, Rapid PVST+, PortFast/BPDU Guard, LACP/PAgP and CDP/LLDP; longest prefix match, administrative distance, floating statics, OSPF cost, neighbour states, adjacency requirements and DR/BDR.
- **Services, security and automation:** NAT terminology and tables, DHCP relay, QoS marking and queuing, SNMPv3, syslog levels; ACL placement, port security, DHCP snooping, DAI, device hardening, AAA and VPNs; RF, AP modes and WLC WLAN setup; planes, REST, JSON, Ansible and Terraform.
- **New module 3, Troubleshooting:** a layer-by-layer method with the show commands, ping symbols and traceroute patterns that prove each cause.

### AWS Cloud (done in this pass)

AWS went from 7 to 16 lessons, from 3 to 21 exercises, and from 38 to 89 quizzes (about 26k words, up from 10k). The exercises are Python models of AWS behaviour and day-to-day tasks: purchasing and EBS choices, S3 lifecycle and bucket audits, VPC subnet plans and NACL/SG evaluation, DynamoDB capacity, full IAM evaluation with SCPs and boundaries, CloudWatch alarms, CloudTrail investigation, cost estimates and waste, Lambda cost and concurrency, SQS visibility and DLQs, DR strategy choice and Route 53 routing. Prices are labelled as illustrative. The whole track verifies with 25 checks and 0 problems. New lessons:

- **Core services:** EC2 instance families, purchasing options, AMIs and IMDSv2, EBS volume types, placement groups, ELB and Auto Scaling; S3 classes, lifecycle, versioning, Object Lock, replication, encryption and performance; VPC CIDR planning, route tables, NAT, security groups vs NACLs, endpoints, peering, Transit Gateway, VPN and Direct Connect; RDS Multi-AZ vs read replicas, Aurora, DynamoDB keys and capacity, caching and analytics.
- **Security and operations:** roles, policy types and the full evaluation logic, Organizations, Control Tower and Identity Center; CloudWatch, CloudTrail, Config, EventBridge and Systems Manager; pricing, data transfer, Budgets, Cost Explorer, tagging and optimisation; Lambda, API Gateway, SQS/SNS/EventBridge/Kinesis, Step Functions, ECS/EKS/Fargate.
- **Solutions Architect:** RTO/RPO, the four DR strategies, AWS Backup, Route 53 routing policies, Global Accelerator, Shield and WAF.

### Microsoft Azure (done in this pass)

Azure went from 5 to 11 lessons, from 0 to 12 exercises, and from 27 to 62 quizzes (about 18k words, up from 8k). The new lessons follow the AZ-104 domains at configuration depth, and the exercises are Python models of how Azure decides: RBAC inheritance with NotActions and deny assignments, dynamic group rules, Policy effects in order, resource locks, redundancy choice, SAS validation, availability set placement, VMSS autoscale with cooldown, subnet-then-NIC NSG evaluation, effective routes (UDR > BGP > system), a KQL-style query and GFS backup retention. The whole track verifies with 16 checks and 0 problems. New lessons:

- **Identity and governance:** Entra roles vs Azure RBAC, groups and administrative units, role definitions, scope inheritance, Conditional Access, PIM and managed identities; management groups, Policy effects and initiatives, locks, tags and cost tools.
- **Administration in depth:** storage redundancy, access tiers, lifecycle, SAS and private endpoints, Azure Files; VM sizes and disks, availability sets and zones, scale sets, App Service plans and slots, containers; VNets, NSGs and ASGs, UDRs, peering, VPN and ExpressRoute, the four load balancers, private DNS; Azure Monitor, KQL, alerts, Application Insights, Network Watcher, Backup and Site Recovery.
