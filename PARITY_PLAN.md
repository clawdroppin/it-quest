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

### Google Cloud (done in this pass)

Google Cloud went from 4 to 10 lessons, from 0 to 12 exercises, and from 17 to 50 quizzes (about 16k words, up from 6k). The exercises model Google Cloud's own rules: compute choice and SUD/CUD/Spot pricing, VPC firewall priority with implied rules, subnet plans with 4 reserved addresses and GKE secondary ranges, storage class cost with early deletion, BigQuery bytes scanned, IAM inheritance with deny policies, org policy merge/replace, error budgets, multi-window burn-rate alerts, Dataflow-style windows with watermarks and Pub/Sub redelivery with dead-lettering. The whole track verifies with 16 checks and 0 problems. New lessons:

- **Foundations in depth:** machine families, discounts, MIGs, GKE Autopilot vs Standard, Cloud Run; global VPCs, firewall rules and policies, Cloud NAT, Private Google Access, Shared VPC, HA VPN and Interconnect, load balancers; Cloud Storage classes and Bucket Lock, Cloud SQL, AlloyDB, Spanner, Firestore, Bigtable and BigQuery.
- **Security and reliability:** IAM deny policies and conditions, service accounts without keys, org policy constraints, audit logs and security services; SLIs, SLOs, error budgets, burn-rate alerting, Cloud Monitoring and Logging, incident management.
- **Design:** the Pub/Sub → Dataflow/Dataproc → BigQuery pipeline, Composer, Dataplex, Datastream and Vertex AI, with clue words for each.

### Certified Kubernetes Administrator (done in this pass)

CKA went from 5 to 11 lessons, from 4 to 16 exercises, and from 21 to 57 quizzes (about 21k words). Each new lesson follows a CKA domain and comes with flashcards, drills and two checked Python exercises that model Kubernetes' own rules: RollingUpdate maxSurge/maxUnavailable rounding, QoS classes and HPA decisions with tolerance, the scheduler's filter phase with its Pending event message, topology spread skew, CoreDNS search domains, Gateway API HTTPRoute precedence and weights, PV/PVC binding with dynamic provisioning, reclaim policies, an RBAC authoriser, Pod Security Standards, a kubeadm upgrade plan with version skew, and Pod triage by state and exit code. The track verifies with 20 checks and 0 problems. New lessons:

- **Workloads and scheduling:** rollouts and rollbacks, DaemonSets, StatefulSets, Jobs and CronJobs, ConfigMaps and Secrets, requests, limits and QoS, probes, HPA; node affinity, taints and tolerations, topology spread, priority and preemption, cordon/drain, static Pods.
- **Services, networking and storage:** Service types, EndpointSlices, DNS names, Ingress and the Gateway API; volumes, PVs, PVCs, access modes, reclaim policies, StorageClasses and expansion.
- **Administration and troubleshooting:** authentication, RBAC, ServiceAccounts, kubeconfig, certificates and Pod Security Admission; kubeadm upgrades, etcd snapshot and restore, and diagnosing Pods, nodes, the control plane and Services.

### Terraform Associate (done in this pass)

Terraform went from 5 to 8 lessons, from 3 to 9 exercises, and from 22 to 40 quizzes (about 13k words, up from 8k). The new exercises model Terraform's own rules: variable precedence across defaults, TF_VAR_, tfvars, auto.tfvars and CLI flags; apply waves and reverse destroy order from the dependency graph with cycle detection; a plan with forces-replacement attributes and create_before_destroy; moved and removed blocks against state; version constraint resolution including `~>`; and HCP Terraform policy checks with advisory, soft-mandatory and hard-mandatory enforcement. The track verifies with 13 checks and 0 problems. New lessons:

- **Configuration in depth:** providers, aliases and the lock file, constraint operators, variables and validation, precedence, sensitive values, data sources, the graph and parallelism.
- **State and plans in depth:** plan symbols, replacement, saved plans, drift and refresh-only, -replace, import blocks with generated config, moved/removed, state commands, backends and locking.
- **Modules and HCP Terraform:** module sources and versions, boundaries, CLI vs HCP workspaces, run workflows, Sentinel/OPA enforcement levels.

### Cloud Computing (done in this pass)

The general cloud track went from 8 to 11 lessons, from 11 to 17 exercises, and from 46 to 64 quizzes (about 20k words, up from 15k). The new lessons fill the gaps in virtualization, containers and security, which had no exercises. The exercises cover packing VMs onto hosts with overcommit, reserve and anti-affinity plus an N+1 check; choosing a 7 Rs strategy with online vs offline transfer; simulating the Docker build cache with .dockerignore; scoring container hardening; envelope encryption with KEK rotation (toy cipher, labelled as such); and a CIS-style posture check with per-section compliance. The track verifies with 21 checks and 0 problems. New lessons:

- **Virtualization, capacity and migration:** hypervisor resource sharing, overcommit, NUMA, N+1, shared responsibility, the 7 Rs and transfer-time maths.
- **Containers in depth:** layers, build cache ordering, multi-stage builds, tags vs digests, signing and scanning, namespaces, cgroups and hardening.
- **Cloud security and compliance:** identity as the perimeter, encryption in transit/at rest/in use, envelope encryption and key management options, secrets, detection, CSPM/CNAPP, compliance reports and common misconfigurations.

### ITIL 4 Foundation (done in this pass)

ITIL went from 4 to 7 lessons, from 0 to 6 checked exercises, and from 25 to 44 quizzes. Each new lesson first restates what the Foundation exam expects, word for word where the syllabus does, then marks workplace practice as **(beyond Foundation)**. The exercises cover an impact × urgency priority matrix with SLA breach reporting; raising problems from recurring incidents and linking them to known errors; availability counted only within agreed service hours, with MTBF and MTRS; KPIs that expose the watermelon effect; value stream mapping with flow efficiency and the bottleneck; and routing changes (standard, emergency, or normal by risk score) with a change freeze. ITIL verifies with 10 checks and 0 problems. New lessons:

- **Guiding principles and value streams in practice:** key phrases and traps for each principle, value chain activities in scenarios, and the continual improvement model.
- **Incident, problem and change in practice:** exam definitions, priority matrices, escalation, major incidents, problem phases and techniques, and change types and authorities.
- **Service levels and measurement:** SLM, availability (series and parallel), capacity, continuity (RTO, RPO, BIA), metrics, CSFs and KPIs, relationship and supplier management, and XLAs.

### CISSP (done in this pass)

CISSP went from 4 to 7 lessons, from 0 to 6 checked lesson exercises (plus the risk register build), and from 20 to 41 quizzes. The material is managerial and defensive. The exercises cover choosing safeguards by ALE and safeguard value against a risk appetite; checking BIA metrics (RTO + WRT ≤ MTD, backup interval ≤ RPO, recovery site readiness); enforcing Bell-LaPadula (including strong star) and Biba; picking cryptographic operations, counting keys and flagging weak algorithms; building restore chains for full/incremental/differential schedules; and ordering evidence by volatility while validating a chain-of-custody log with hashes. CISSP verifies with 10 checks and 0 problems. New lessons:

- **Domain 1 in depth:** quantitative and qualitative risk, responses and control types, NIST RMF/CSF 2.0, ISO 27001, due care vs due diligence, the policy hierarchy, BCP steps, metrics and tests, personnel and supply chain security, and the canons.
- **Domains 3 and 5 in depth:** formal models, Common Criteria, the reference monitor, architecture vulnerabilities, cryptography choices and key counts, PKI, access control models, biometrics errors and federation protocols.
- **Domains 6–8 in depth:** the incident response lifecycle, evidence and order of volatility, logging and monitoring, backups, RAID and 3-2-1, operational controls, assessments and SOC reports, the secure SDLC and CMMI.

### PMP (done in this pass)

PMP went from 3 to 6 lessons, from 0 to 6 checked lesson exercises (plus the critical path/EVM build), and from 11 to 32 quizzes, across all three exam domains. The exercises cover a power/interest grid with engagement gaps and communication channels; RACI validation (exactly one A, overload, too many consulted); PERT path estimates with sigma bands for deadlines; EVM forecasts choosing the right EAC formula for each situation, with TCPI; release forecasting from a velocity range with scope growth and Little's law; and EMV decision trees with an FPIF contract that reaches its ceiling. PMP verifies with 10 checks and 0 problems. New lessons:

- **People in depth:** servant leadership and power, Tuckman, motivation, Thomas-Kilmann conflict modes, the power/interest grid and engagement matrix, communication channels and methods, RACI, negotiation and group decisions.
- **Schedule and cost in depth:** estimating techniques and ranges, PERT and path variance, float and dependencies, crashing vs fast tracking, reserves and baselines, EVM and the four EAC formulas, ETC, VAC and TCPI.
- **Agile, risk, procurement and business:** Scrum and Kanban metrics, prioritisation, EMV and decision trees, threat/opportunity responses, contract types and who bears risk, procurement documents, NPV/IRR/payback/BCR, benefits and organisational change.

### 基本情報技術者 (IPA FE) (done in this pass)

IPA FE went from 7 to 10 lessons, from 4 to 10 lesson exercises, and from 43 to 61 quizzes. The new lessons cover the calculation questions that recur in 科目A, using the Japanese terms the exam uses. The exercises cover 8-bit two's complement with logical and arithmetic shifts and overflow; encoding IEEE 754 single precision by hand, checked against struct (including why 0.1 differs); CPI from an instruction mix, MIPS, multi-level cache access time and pipelines; transfer times, line utilisation, image/audio sizes and RAID capacity; break-even, safety margin and target profit; and ABC analysis with maximax, maximin and expected-value decisions. IPA FE verifies with 14 checks and 0 problems. New lessons:

- **基礎理論 in depth:** 2の補数, シフト演算, 浮動小数点数, the named 誤差 types, information quantities and De Morgan.
- **テクノロジ系 calculations:** CPU performance, 実効アクセス時間, disk access time, transfer time, data sizes and RAID.
- **ストラテジ系 calculations:** 損益分岐点, 発注点 and EOQ, 減価償却, ABC分析, investment appraisal, decision principles, PPM and the most-tested laws.

### Japanese for IT Work (done in this pass)

Japanese went from 5 to 8 lessons, from 0 to 6 checked lesson exercises (plus the trainer build), and from 23 to 41 quizzes. The new lessons cover rule-based grammar, so learners implement the rules and the verifier checks them. The exercises conjugate verbs in all three groups (ます, て, た, ない, potential, with the 行く and ある exceptions); build support sentences (〜てください, 〜ています, 〜てもいいですか, 〜ないでください, できません); transform verbs into 尊敬語 and 謙譲語 ます forms, including the special verbs and the five い verbs; catch double keigo, humble verbs used for customers and ご苦労様; read numbers grouped by 万/億/兆 with sound changes; and read dates and times for a maintenance notice (irregular days, 翌, ふん/ぷん). Japanese verifies with 10 checks and 0 problems. New lessons:

- **Verbs:** the three groups and the -iru/-eru traps, ます, て/た, ない and potential forms, and their workplace uses.
- **Numbers, money, dates and times:** four-digit grouping, sound changes, irregular day readings, months, hours and minutes, counters.
- **敬語 forms:** the お/ご patterns, special respectful and humble verbs, common mistakes, and daily support phrases.

### IT Career in Japan (done in this pass)

The career track went from 5 to 7 lessons, from 0 to 4 checked lesson exercises (plus the application kit build), and from 22 to 34 quizzes. The new lessons cover the two places where numbers and form matter most: offers and application documents. Changeable figures (minimum wage, take-home ratio) are marked as examples to update, and the offers lesson says it's general information, not legal advice. The exercises check fixed overtime against the hourly base and the minimum wage, compare offers by real hourly pay, convert dates to 和暦 with 元年 and exact era boundaries, and build a 学歴・職歴 table with the standard phrases and full age. The track verifies with 8 checks and 0 problems. New lessons:

- **Reading offers in depth:** pay components, 固定残業代 legality, 36協定 limits and premiums, hourly base and real hourly pay, take-home rules of thumb, and other clauses to check.
- **履歴書 and 職務経歴書 in detail:** 和暦 and era changes, the 学歴・職歴 table and its phrases, certifications, and 職務経歴書 formats with quantified results.

### Cisco CCNP Enterprise (done in this pass)

CCNP went from 4 to 9 lessons, from 0 to 10 checked lesson exercises (plus the BGP simulator build), and from 17 to 48 quizzes (about 16k words, up from 7k). The exercises model the protocols' own rules: the EIGRP composite metric with feasible successors and variance; OSPF DR/BDR election (non-preemptive) and LSA filtering per area type; Cisco prefix lists with ge/le; route maps with BGP local preference and best-path choice; DSCP decoding, shaper Tc and LLQ share; wireless link budgets in dBm with FSPL; parsing RESTCONF ietf-interfaces JSON; configuration drift with RESTCONF remediation; STP root, root port and designated port election; and extended ACLs with wildcard masks. CCNP verifies with 14 checks and 0 problems. New lessons:

- **QoS and wireless in depth:** DSCP/CoS marking, trust boundaries, LLQ/CBWFQ/WRED, policing vs shaping and the token bucket, RF math, WLC architecture and roaming.
- **EIGRP and OSPF in depth:** metrics, feasibility, stub routers, OSPF cost, LSA and area types, DR election and neighbour troubleshooting.
- **Route control in depth:** prefix lists, route maps, redistribution and AD, BGP policy attributes and best path.
- **Automation and assurance in depth:** YANG, NETCONF vs RESTCONF, HTTP status codes, config management tools, IP SLA, NetFlow, SNMPv3, telemetry and syslog.
- **Switching and security in depth:** STP election and protection, FHRP comparison, ACLs and wildcards, AAA, Layer 2 security and 802.1X.

### DevOps and SRE (done in this pass)
- New lessons: d0_02 Git internals (blob IDs with hashlib, merge base and ranges), d1_03 deployment strategies (automated canary analysis, sticky feature-flag buckets), d1_04 release engineering (DORA metrics, conventional commits to semver and changelog), d2_02 logs, traces and incidents (structured log queries, trace critical path and self time).
- Totals now: 16 lessons, 28 exercises, 80 quizzes, about 38k words. Verifies with 34 checks, 0 problems.

### Linux Desktop & Environment (done in this pass)
- New lessons: le1_02 text tools (grep/sed/awk pipelines on an access log and a messy Windows CSV), le3_02 reliable shell scripts (getopts CLI, safe config parsing without eval), le4_02 systemd services, timers and networking (triaging systemctl output, layer-by-layer network diagnosis).
- Totals now: 8 lessons, 18 exercises, 40 quizzes, about 13k words. Verifies with 18 checks, 0 problems.

### UI Animation (done in this pass)
- New lessons: ua0_02 easing, springs and colour interpolation (spring simulation with damping ratio and settle time; sRGB vs linear vs OKLab blends), ua2_02 CSS and SVG animation (keyframes generator, path morph compatibility and interpolation), ua3_02 performance and accessibility (jank from rAF timings, animation audit against compositing and WCAG motion rules).
- Totals now: 7 lessons, 14 exercises, 34 quizzes, about 12k words. Verifies with 14 checks, 0 problems.

### Vector UI Design (done in this pass)
- New lessons: vu0_02 SVG geometry (true Bézier bounding box, viewBox meet/slice/none mapping, path data minifier), vu3_02 clean and accessible SVG (accessible icon-only toolbar), vu4_02 colour systems (OKLCH tonal scale with gamut mapping and contrast, light/dark token audit).
- Note: @@lang is per lesson (the last one wins), so Python and HTML exercises live in separate lessons.
- Totals now: 8 lessons, 16 exercises, 37 quizzes, about 13k words. Verifies with 16 checks, 0 problems.

### Rust UI (done in this pass)
- New lessons: ru0_02 app architecture (Elm-style update with snapshot undo/redo and capped history; background search over a channel with stale-result handling), ru2_02 input and gestures (click/double-click/drag recogniser; kinetic scrolling with velocity window and friction), ru4_02 text wrapping and virtual lists (greedy wrap and truncation by chars; fixed and variable-height visible ranges).
- Totals now: 8 lessons, 19 exercises, about 16k words. Verifies with 19 checks, 0 problems.

### GIMP (done in this pass)
- New lessons: gm0_02 image data (bit depth, compression, formats, EXIF privacy; size and RLE maths, orientation and metadata cleaning), gm1_02 transforms and resampling (nearest vs bilinear with pixel centres, aliasing, rotated canvas and crop with aspect), gm2_02 blur, sharpening and noise (Gaussian kernel and unsharp mask halos, mean vs median on salt-and-pepper).
- Totals now: 7 lessons, 17 exercises, about 13k words. Verifies with 17 checks, 0 problems.

### DaVinci Resolve (done in this pass)
- New lessons: dr1_02 sync, multicam and pacing (cross-correlation audio sync with sound-travel distance; ASL, shot histogram and snapping cuts to the beat), dr3_02 balancing and skin tones (chart gains and stops; Rec.709 vectorscope hue against the skin line), dr6_02 frame rates and streaming (nearest-frame mapping and 3:2 pulldown; bpp-based ladders by codec and content).
- Totals now: 11 lessons, 22 exercises, about 19k words. Verifies with 22 checks, 0 problems.

### Blender (done in this pass)
- New lessons: bl0_02 3D maths (TRS matrices and a parent hierarchy; quaternions, slerp vs Euler lerp, gimbal lock), bl1_02 mesh data (Newell normals and flipped faces; manifold, loose, duplicate and Euler checks with subdivision counts), bl2_02 cameras, light and render budgets (FOV and depth of field; samples, time and deadlines).
- Totals now: 12 lessons, 24 exercises, about 22k words. Verifies with 24 checks, 0 problems.

### GUI Design (done in this pass)
- New lessons: gu0_02 layout engines (flex line resolution with freezing; grid track sizing, 1fr vs minmax(0, 1fr)), gu2_02 responsive interfaces (mobile-first dashboard with clamp and pointer queries; container-query card), gu3_02 accessible widgets (native modal dialog with focus return; APG tabs with roving tabindex).
- Totals now: 12 lessons, 26 exercises, about 30k words. Verifies with 28 checks, 0 problems.

### PMP, second pass
- New module 2 · Quality, Change and the Exam: pm2_01 quality and integrated change control (control chart with rule of seven; Pareto vital few and change-request routing), pm2_02 exam strategy (pacing analysis from timings; study hours by domain weight and gap).
- Totals now: 8 lessons, 10 exercises, about 12k words. Verifies with 14 checks, 0 problems.

### IT career in Japan, second pass
- New lessons: jc1_02 Highly Skilled Professional points (simplified calculator with salary age limits; planning to 80 points as age changes), jc3_02 take-home pay (social insurance, income tax and resident tax model; resident tax start schedule). Figures are labelled as 2026 example values to check against official sources.
- Totals now: 9 lessons, 8 exercises, about 13k words. Verifies with 12 checks, 0 problems.

### Japanese, second pass
- New lessons: jp0_02 kana in depth (Hepburn converter with small kana, っ and ん'; NFKC form normalisation and validation), jp3_02 障害報告 (numbered incident report generator; set phrases and casual-to-polite rewrites).
- Totals now: 10 lessons, 10 exercises, about 14k words. Verifies with 14 checks, 0 problems.

### CISSP, second pass
- New lessons: cs0_03 asset security (NIST 800-88 sanitisation choices; retention with legal holds and over-retention), cs1_03 network and identity in depth (Kerberos service-ticket checks with skew and replay; separation-of-duties, privilege and leaver review).
- Totals now: 9 lessons, 10 exercises, about 15k words. Verifies with 14 checks, 0 problems.

### ITIL, second pass

- it0_02_value_offerings: utility vs warranty classifier (flags services missing either), service offerings composed of goods, access and actions with monthly cost and imposed risk.
- it2_04_config_events_knowledge: CMDB impact analysis (reverse dependency walk, shared CIs), event filtering with thresholds, deduplication and host-down correlation; release vs deployment and knowledge management in the text.

### Terraform, second pass

- tf2_02_validation_tests: validation/precondition/postcondition/check/test comparison; variable validation with can(), a mini terraform test runner with expect_failures.
- tf2_03_pipelines_drift: saved plans, OIDC, plan risk table, drift and -refresh-only, locking; pipeline gate from plan JSON, drift classification with ignore_changes.

### IPA FE, second pass

- fe2_02_structures_hands_on: 1-based heap index rules, hashing with linear probing, BST shape, binary search counts; heap trace exercise, hash table + binary search comparisons.
- fe3_03_db_network_hands_on: functional dependencies, closures, candidate keys, 2NF/3NF, SQL NULL rules, subnetting; normal-form analyser and a bitwise subnet calculator.

### CCNP, second pass

- cp0_03_overlays_tunnels: encapsulation overhead table, PMTUD black holes, GRE/IPsec/VXLAN/LISP; tunnel MTU and MSS sizing, LISP map-cache lookup with priority/weight and negative replies.
- cp1_07_ipv6_multicast: IPv6 address types, EUI-64, solicited-node, multicast MAC mapping, IGMP, PIM and RPF; EUI-64/solicited-node/classifier exercise, multicast MAC collisions and RPF checks.

### GCP, second pass

- gc2_03_migration_dr: migration phases and strategies, waves, DR patterns and data-service RPOs; wave planner with chatty pairs and strategy hints, cheapest DR pattern per workload with cost.
- gc2_04_load_balancing_scaling: four-question load balancer choice, MIG autoscaling formula and controls; LB chooser, autoscaler simulation with stabilisation and VM-hours.

### PMP, third pass

- pm1_04_scope_procurement: RTM, 100% rule, contract types and incentive maths; WBS rollup with traceability gaps and gold plating, CPFF/CPIF/FPIF payments and PTA.
- pm1_05_resources_communication: leveling vs smoothing, float, channels, push/pull/interactive; histogram smoothing within float, communication planning.

### UI animation, third pass

- ua1_02_bones_kinematics: bones, FK, two-bone IK, skeleton deformation weights, render settings; FK keyframe interpolation, IK with both bend directions and clamping.

### Japan career, third pass

- jc0_02_listings_cities: 募集要項 fields, SES signals, holiday terms, city comparison; listing screener with SES/contract warnings, after-rent city comparison.
- jc3_03_leave_resigning: statutory leave table, carry-over and 5-day rule, notice, bonus timing, between-jobs admin; leave balance tracker, last-day planner.

### GIMP, third pass

- gm3_02_effects_legibility: shadows, outlines, glows from alpha, non-destructive effects, text contrast and scrims; dilation/offset-blur compositing, text colour and scrim opacity for AAA.

### Linux desktop, third pass

- le0_02_disks_boot: device names, UUIDs, GPT/ESP, file systems, fstab, UEFI → GRUB → systemd; safe USB-stick finder from lsblk -P, fstab vs blkid checker.
- le2_02_packages_deep: Debian version rules, holds, ownership and cleanup commands across apt/dnf/pacman, repo trust; dpkg -S ownership report, sort -V outdated check and biggest packages.

### Rust flashcards

- Rust had 40 flashcards across 106 lessons. Added flashcard decks (4–9 cards each) to 76 concept lessons from m0 to m11; projects and the guided g* builds left as they are.

### Parity floor

Target for every non-language track: at least 12 lessons, 20 checked exercises, 60 quiz questions and 20k words. Language tracks are measured against Rust separately.

### Vector UI, to the floor

- vu1_02_type_spacing: modular type scale, rem, line height, 4-point grid; scale generator, spacing audit with proximity check.
- vu1_03_components_layout: variant matrices, hug/fill/fixed as flexbox; gap and orphan finder, flex row sizing with min/max clamping.
- vu2_02_form_controls (HTML): labelled text field with helper and error states, native checkbox restyle and role="switch".
- vu3_03_icon_geometry: keylines, optical correction, pixel snapping, scaling icon sets; crisp-stroke checker, per-size scaling report.

### UI animation, to the floor

- ua0_03_timing_choreography: durations, distance scaling, staggers, overlap; duration model, choreographed timeline with capped stagger.
- ua1_03_motion_paths: Bézier t vs distance, arc-length tables, tangents and auto-orient, arcs and throws; constant-speed sampling, path orientation and parabola.
- ua2_03_flip_view_transitions (HTML): WAAPI, FLIP list reorder, View Transitions gallery with fallback and reduced motion.
- ua3_03_loading_feedback: response-time thresholds, spinner delay/minimum, skeletons, honest progress, optimistic UI; flicker-free spinner plan, EMA progress estimate.

### Japanese, to the floor

- New exercises appended (at the end, existing order untouched) to the four lessons that had none: kana conversion (jp0_01), 自己紹介 builder and politeness check (jp1_01), uchi/soto references (jp2_01), business email builder and checker (jp3_01).
- jp1_04_particles: core particles, は vs が, に vs で; particle-by-role sentence builder, は/が rules.
- jp2_03_meetings_phone: meeting flow and phrases, 根回し, 議事録, phone handling; minutes generator, phone message memo.
- jp3_03_it_kanji: on/kun readings, sound changes, IT compounds, Leitner boxes; longest-match furigana, Leitner review simulation.

## True parity pass (floor: 12 lessons, 20 exercises, 60 quizzes, 20k words)

Order: rustui → gimp → cissp, itil → pmp, japan-career → linuxenv, terraform, ipa-fe → azure, ccnp, gcp, resolve → cloud, cka, blender; then language tracks against Rust.

### rustui (now 12 lessons, 27 exercises, 60 quizzes, ~23.5k words)
- ru0_03_accessibility_keyboard: AccessKit tree, roles and names, roving focus; tree audit, grid keyboard navigation.
- ru1_02_colour_maths: HSL, linear-light mixing, compositing; RGB↔HSL with lighten, linear mix and source-over.
- ru3_02_vector_paths: adaptive Bézier flattening, non-zero vs even-odd; flattening, point-in-polygon for both rules.
- ru4_03_dpi_responsive: logical vs physical pixels, snapping, size classes with hysteresis; pixel snapping, breakpoint state machine.

### gimp (now 12 lessons, 27 exercises, 64 quizzes, ~22.4k words)
- gm0_03_precision_paths: guides, grids, Measure/Straighten, alignment, paths; measure and straighten with crop, align and distribute.
- gm1_03_cutouts_alpha: halos, Color to Alpha, Foreground Select, chroma key, spill; Color to Alpha, soft chroma key with despill.
- gm2_03_white_balance_colour_management: white balance, ICC assign vs convert, intents, soft proofing, TAC; grey world vs white patch, CMYK with an ink limit.
- gm3_03_web_indexed_dither: indexed colour, Floyd–Steinberg vs ordered, sprite sheets, srcset; dithering, export widths and sheet layout.

### cissp (now 13 lessons, 20 exercises, 79 quizzes)
- cs0_04_law_ethics_privacy: canons, law types, IP, GDPR/APPI, Wassenaar; breach deadline calculator, ethics by canon.
- cs1_04_secure_design_physical: design principles, zero trust, shared responsibility, ICS, CPTED, fire, power; responsibility matrix review, DC environment check.
- cs2_03_software_security_deep: SDLC controls, SAST/DAST/IAST/SCA, SBOM and supply chain, APIs; pipeline gate, SBOM vs advisories.
- cs2_04_assessment_audit_deep: test types, SOC 1/2/3 and Type I/II, sampling, KPIs/KRIs; control sample evaluation, vulnerability metrics with coverage.
- Exercises appended to cs0_01 (think like a manager) and cs1_01 (key counts, brute-force time).

### itil (now 13 lessons, 20 exercises, 78 quizzes, ~25.6k words)
- it1_03_continual_improvement: CI model, CIR, PDCA/Lean/Kaizen, baselines; register prioritisation, progress vs baseline with paired metrics.
- it1_04_customer_journey_experience: DSV journey, touchpoints, moments of truth, NPS/CSAT/CES, XLAs; survey scoring, watermelon finder.
- it2_05_requests_service_desk: service requests, catalogue, desk structures, shift-left, desk metrics; request routing, desk dashboard.
- it2_06_assets_finance_suppliers: ITAM/SAM, CapEx/OpEx, showback/chargeback, suppliers and underpinning contracts; licence position, showback.
- Exercises appended to it0_01 (outputs/outcomes/costs/risks) and it1_01 (which guiding principle).

### pmp (now 13 lessons, 20 exercises, 74 quizzes, ~22.8k words)
- pm0_03_motivation_leadership: Maslow, Herzberg, X/Y/Z, McClelland, Vroom, situational leadership, power, distributed teams; style chooser, time-zone overlap rotation.
- pm1_06_flow_forecasting: Kanban, WIP limits, Little's law, CFDs, Monte Carlo; flow metrics from a board, seeded Monte Carlo forecast.
- pm2_03_benefits_change: NPV/IRR/payback/BCR, opportunity and sunk cost, benefits ownership, ADKAR/Kotter/Satir/Bridges, transition; project ranking, benefits and ADKAR gaps.

### japan-career (now 14 lessons, 20 exercises, 71 quizzes, ~23.5k words)
- jc1_03_visa_changes_pr: 14-day notifications, renewal and 特例期間, re-entry, permanent residence routes; deadline planner, PR eligibility.
- jc2_04_interviews_tests: 選考 flow, SPI and coding tests, common questions, manners, 候補日; slot formatter, answer checker.
- jc3_04_housing_contracts: listings, 初期費用, lease types, 更新料, 原状回復, moving admin; listing parser, initial and two-year costs.
- Exercises appended to jc0_01 (employer types) and jc1_01 (does the offer fit 技人国).

### linuxenv (now 12 lessons, 26 exercises, 63 quizzes, ~20.2k words)
- le1_03_users_security: UIDs, groups, sudo and visudo, ACLs, setuid, capabilities, ufw, SSH keys, LUKS, AppArmor/SELinux; account audit, sshd_config check.
- le4_03_display_audio: Wayland vs X11, portals, logical pixels and scaling, PipeWire/WirePlumber, Bluetooth profiles; monitor layout, no-sound triage.

### terraform (now 12 lessons, 20 exercises, 62 quizzes, ~22.5k words)
- tf3_02_secrets_security: sensitive propagation, state as a secret, ephemeral resources and write-only args, OIDC, scanners; state secret finder, output sensitivity check.
- tf3_03_providers_multi_env: required_providers and the lock file, constraints, aliases and module providers maps, environment layouts, Infracost; provider resolution, plan cost estimate.
- Exercises appended to tf0_01 (command sequence check), tf1_01 (workspace state paths) and tf2_01 (dynamic block expansion).

### ipa-fe (now 14 lessons, 20 exercises, 84 quizzes, ~21.9k words)
- fe2_03_graphs_rpn: stacks, queues, 逆ポーランド記法, traversal orders, adjacency matrices, BFS, Dijkstra; shunting-yard and RPN evaluation, BFS and Dijkstra.
- fe3_04_security_hands_on: salts and stretching, key roles, HMAC vs signatures, access control, log patterns; PBKDF2 storage, login attack classifier.
- Exercises appended to fe0_01 (pass estimate) and fe4_01 (arrow diagram critical path).

### azure (now 12 lessons, 20 exercises, 69 quizzes, ~23.7k words)
- az2_06_apps_keyvault: App Service tiers and slots, Functions hosting plans, ACI/Container Apps/AKS/ACR, managed identities, Key Vault RBAC, soft delete and purge protection; tier choice and slot swap, Functions cost, Key Vault access and recovery.
- Exercises appended to az0_01 (composite SLA), az1_01 (database choice), az1_02 (Conditional Access), az1_04 (budget forecast alerts) and az2_05 (stateful metric alert).
