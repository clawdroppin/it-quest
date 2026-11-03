# Course parity plan

Goal: every programming and scripting track reaches the depth of the Rust track. Modules that go beyond IT work carry the label **🎓 Beyond IT** in the module title and a note at the top of each lesson.

## Where things stand (November 2026)

| Track | Lessons | Exercises | Quizzes | Words | vs Rust (words) |
|---|---:|---:|---:|---:|---:|
| Rust | 106 | 179 | 254 | 175k | 100% |
| Python | 84 | 155 | 216 | 103k | 59% (done, see below) |
| PowerShell | 24 | 46 | 53 | 28k | 16% |
| Bash | 23 | 41 | 53 | 27k | 15% |
| JavaScript | 55 | 106 | 153 | 83k | 47% (done, see below) |
| SQL | 16 | 33 | 35 | 23k | 13% |
| C# | 14 | 24 | 39 | 22k | 12% |
| C++ | 14 | 23 | 40 | 24k | 14% |
| Go | 13 | 22 | 38 | 26k | 15% |
| HTML/CSS | 13 | 19 | 43 | 21k | 12% |
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
