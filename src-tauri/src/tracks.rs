//! Every track and module, in teaching order. New lesson files must be listed here.

use crate::course::{Category, Level, Module, Track, build, lessons, module, parse_lesson};
use crate::runner::Lang;

pub fn rust() -> Track {
    Track {
        id: "rust",
        name: "Rust",
        icon: "🦀",
        category: Category::Programming,
        lang: Some(Lang::Rust),
        blurb: "Fast, safe systems programming. The full Rust Quest course: from zero to a desktop app with a GUI.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "What programs are, and your first one.", "rust";
                "m0_01_welcome.md", "m0_02_what_is_a_program.md", "m0_03_hello_world.md", "m0_04_compiler_errors.md"),
            module!(Novice, "1 · Programming Fundamentals", "The building blocks every program is made of.", "rust";
                "m1_01_variables.md", "m1_02_numbers.md", "m1_03_bool_char.md", "m1_04_operators.md", "m1_05_text_formatting.md",
                "m1_06_functions.md", "m1_07_if_else.md", "m1_08_loops.md", "m1_09_comments_style.md", "m1_10_thinking.md"),
            module!(Novice, "2 · Collections of Data", "Grouping many values together.", "rust";
                "m2_01_tuples.md", "m2_02_arrays.md", "m2_03_vectors.md", "m2_04_iterating.md", "m2_05_strings_deep.md",
                "m2_06_hashmap.md", "m2_07_grids.md", "m2_08_project_gradebook.md"),
            module!(Intermediate, "3 · Ownership & Borrowing", "How Rust manages memory without a garbage collector.", "rust";
                "m3_01_memory.md", "m3_02_ownership.md", "m3_03_copy_clone.md", "m3_04_references.md", "m3_05_mut_references.md",
                "m3_06_slices.md", "m3_07_lifetimes.md"),
            module!(Intermediate, "4 · Modeling Data", "Structs, enums and pattern matching.", "rust";
                "m4_01_structs.md", "m4_02_methods.md", "m4_03_enums.md", "m4_04_match.md", "m4_05_option.md", "m4_06_if_let.md",
                "m4_07_project_inventory.md"),
            module!(Intermediate, "5 · Handling Errors & Input", "Programs that deal with the messy real world.", "rust";
                "m5_01_panic.md", "m5_02_result.md", "m5_03_question_mark.md", "m5_04_input.md", "m5_05_custom_errors.md",
                "m5_06_project_calculator.md"),
            module!(Advanced, "6 · Organizing Code", "Cargo, modules, crates and tests.", "rust";
                "m6_01_cargo.md", "m6_02_modules.md", "m6_03_modules_files.md", "m6_04_crates.md", "m6_05_testing.md",
                "m6_06_docs_tools.md", "m6_07_project_hangman.md"),
            module!(Advanced, "7 · Abstraction", "Traits, generics, closures and iterators.", "rust";
                "m7_01_traits.md", "m7_02_generics.md", "m7_03_trait_bounds.md", "m7_04_derive.md", "m7_05_closures.md",
                "m7_06_iterators.md", "m7_07_smart_pointers.md", "m7_08_project_movies.md"),
            module!(Expert, "8 · Real Programs", "Files, data formats and program architecture.", "rust";
                "m8_01_files.md", "m8_02_serde.md", "m8_03_structure.md", "m8_04_debugging.md", "m8_06_collections.md",
                "m8_07_time_threads.md", "m8_05_project_todo.md"),
            module!(Expert, "9 · GUI Capstone", "Build a real desktop app with a GUI.", "rust";
                "m9_01_how_guis_work.md", "m9_02_egui_hello.md", "m9_03_widgets_layout.md", "m9_04_app_state.md",
                "m9_04b_background.md", "m9_05_ledger_model.md", "m9_06_ledger_gui.md", "m9_07_ledger_save.md", "m9_08_shipping.md"),
            module!(Expert, "10 · Advanced Rust", "Advanced traits, dispatch, lifetimes, concurrency, async, macros, unsafe and performance.", "rust";
                "m10_01_advanced_traits.md", "m10_02_dispatch.md", "m10_03_lifetimes_advanced.md", "m10_04_concurrency.md",
                "m10_05_async.md", "m10_06_macros.md", "m10_07_unsafe_ffi.md", "m10_08_performance.md", "m10_09_project_threadpool.md"),
            module!(Expert, "11 · Rust in the Real World", "Command-line tools, networking, web services, databases, the Cargo ecosystem, testing, observability and WebAssembly.", "rust";
                "m11_01_cli.md", "m11_02_networking.md", "m11_03_web.md", "m11_04_databases.md", "m11_05_cargo_ecosystem.md",
                "m11_06_testing_advanced.md", "m11_07_observability_errors.md", "m11_08_wasm.md", "m11_09_project_kvstore.md"),
        module!(Expert, "12 · 🎓 Beyond IT: Embedded and Game Development", "Optional, beyond IT scope: no_std firmware on microcontrollers, and game development with ECS and Bevy.", "rust";
                "m12_01_embedded.md", "m12_02_gamedev.md"),
            build!(Novice, "Start Here", "How the builds work, and setting up an AI helper.", "rust"; "g0_start.md"),
            build!(Advanced, "🍅 Focus Timer", "A Pomodoro timer with a progress ring, task log and weekly stats.", "rust";
                "g1_01_timer_model.md", "g1_02_timer_window.md", "g1_03_timer_stats.md"),
            build!(Advanced, "📝 Note Vault", "A Markdown notes app with search, #tags, live preview and autosave.", "rust";
                "g2_01_vault_model.md", "g2_02_vault_window.md", "g2_03_vault_files.md"),
            build!(Expert, "🗂 File Organizer", "Tidy a messy folder into neat subfolders, with a preview and undo.", "rust";
                "g3_01_organizer_plan.md", "g3_02_organizer_window.md", "g3_03_organizer_apply.md"),
            build!(Expert, "💬 Local AI Chat", "Your own desktop chat app for AI models running on your computer.", "rust";
                "g4_01_chat_api.md", "g4_02_chat_window.md", "g4_03_chat_conversations.md"),
        ],
    }
}

pub fn python() -> Track {
    Track {
        id: "python",
        name: "Python",
        icon: "🐍",
        category: Category::Scripting,
        lang: Some(Lang::Python),
        blurb: "The most useful first language in IT: automation, scripting, data, security tools and the cloud.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Your first programs, and how to read errors.", "python";
                "n0_01_welcome.md", "n0_02_errors.md", "n0_03_how_python_runs.md"),
            module!(Novice, "1 · Fundamentals", "Variables, math, text, decisions, loops and functions.", "python";
                "n1_01_variables.md", "n1_02_numbers.md", "n1_03_strings.md", "n1_04_input.md", "n1_05_conditions.md",
                "n1_06_loops.md", "n1_07_functions.md", "n1_08_booleans_none.md", "n1_09_scope_arguments.md", "n1_10_loop_patterns.md"),
            module!(Novice, "2 · Collections", "Lists, dictionaries, tuples and sets, and a first project.", "python";
                "n2_01_lists.md", "n2_02_dicts.md", "n2_03_tuples_sets.md", "n2_05_comprehensions.md", "n2_06_collections_sorting.md",
                "n2_04_project_inventory.md"),
            module!(Intermediate, "3 · Real-World Input", "Errors, files, the standard library and regular expressions.", "python";
                "i3_01_exceptions.md", "i3_02_files.md", "i3_03_stdlib.md", "i3_04_regex.md"),
            module!(Intermediate, "4 · Data and Objects", "JSON, CSV, classes, and a log analyzer.", "python";
                "i4_01_json_csv.md", "i4_02_classes.md", "i4_03_project_logs.md"),
            module!(Advanced, "5 · Automation Tools", "Run commands, build CLI tools, write tests.", "python";
                "a5_01_system.md", "a5_02_cli_tools.md", "a5_03_testing.md"),
            module!(Advanced, "6 · Networks and APIs", "IP addresses, subnets, sockets, HTTP and REST APIs.", "python";
                "a6_01_networking.md", "a6_02_http.md"),
            module!(Expert, "7 · Professional Python", "Generators, decorators, types, concurrency and real projects.", "python";
                "e7_01_generators.md", "e7_02_decorators.md", "e7_03_typing.md", "e7_04_concurrency.md", "e7_05_professional.md"),
            module!(Expert, "8 · Application Development", "Architecture, databases and migrations, web APIs, GUIs and shipping real apps.", "python";
                "f8_01_architecture.md", "f8_02_data.md", "f8_03_interfaces.md"),
            module!(Advanced, "9 · The Python Object Model", "References and mutability, special methods, inheritance, properties, protocols, enums, pattern matching and functional tools.", "python";
                "p9_01_references.md", "p9_02_dunder_methods.md", "p9_03_inheritance.md", "p9_04_properties_classmethods.md",
                "p9_05_abc_protocols.md", "p9_06_enums_matching.md", "p9_07_functional.md", "p9_08_project_calculator.md"),
            module!(Expert, "10 · Advanced Python", "Descriptors and metaclasses, asyncio in depth, threads and processes, performance, introspection, advanced typing, binary data and C/Rust interop, memory.", "python";
                "q10_01_descriptors_metaclasses.md", "q10_02_asyncio.md", "q10_03_parallelism.md", "q10_04_performance.md", "q10_05_introspection.md",
                "q10_06_typing_advanced.md", "q10_07_binary_interop.md", "q10_08_memory.md", "q10_09_project_scheduler.md"),
            module!(Expert, "11 · Python in the Real World", "FastAPI and Pydantic, ORMs and query performance, packaging, pytest in depth, logging, configuration and secrets, security, dates and times.", "python";
                "r11_01_fastapi.md", "r11_02_orm.md", "r11_03_packaging.md", "r11_04_pytest.md", "r11_05_logging.md",
                "r11_06_config_secrets.md", "r11_07_security.md", "r11_08_dates_times.md", "r11_09_project_relay.md"),
        module!(Expert, "12 · GUI Capstone: Desktop Apps with Tkinter",
                "Tkinter and ttk widgets, layout, events and validation, MVP and undo, tables and themes, menus and files, canvas charts, background threads, and packaging with PyInstaller.",
                "python"; "s12_01_tkinter_basics.md", "s12_02_layout.md", "s12_03_events_variables.md", "s12_04_mvp.md",
                "s12_05_tables_themes.md", "s12_06_menus_dialogs_files.md", "s12_07_canvas_charts.md",
                "s12_08_background_work.md", "s12_09_project_packaging.md"),
        module!(Expert, "13 · 🎓 Beyond IT: Data Science, Machine Learning and Games",
                "Optional, beyond IT scope: data analysis, machine learning from scratch, NumPy and numerical methods, and game development with Pygame.",
                "python"; "t13_01_data_analysis.md", "t13_02_machine_learning.md", "t13_03_scientific_computing.md",
                "t13_04_game_development.md"),
            build!(Intermediate, "🔎 Log Sentinel", "A security tool that finds brute-force attacks and break-ins in SSH logs.", "python";
                "b1_01_sentinel_parse.md", "b1_02_sentinel_rules.md"),
            build!(Advanced, "🌐 Network Toolkit", "Subnet calculator, VLSM planner and a threaded port scanner in one CLI.", "python";
                "b2_01_netkit_subnets.md", "b2_02_netkit_scanner.md"),
            build!(Advanced, "💾 Backup Rotator", "Zip backups with checksums, a retention policy and a dry-run mode.", "python";
                "b3_01_backup_rotator.md"),
            build!(Expert, "🗓 Team Task Board", "A full app: SQLite with migrations, business rules, a JSON API, a web page and a CLI.", "python";
                "b4_01_task_board.md"),
            // generated project ideas python
            build!(Intermediate, "🧩 Downloads Folder Organizer", "Sort a messy folder into subfolders by file type and date.", "python"; "zp_01_folder_organizer.md"),
            build!(Advanced, "🧩 Certification Study Planner", "Plan your study hours to an exam date, stored in SQLite.", "python"; "zp_02_study_planner.md"),
        ],
    }
}

pub fn powershell() -> Track {
    Track {
        id: "powershell",
        name: "PowerShell",
        icon: "💠",
        category: Category::Scripting,
        lang: Some(Lang::PowerShell),
        blurb: "Automate Windows, Active Directory, Microsoft 365 and Azure. A must for Windows admins and help desk.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Cmdlets, help, and the object pipeline.", "powershell";
                "p0_01_welcome.md", "p0_02_pipeline.md"),
            module!(Novice, "1 · Fundamentals", "Variables, text, decisions, loops and functions.", "powershell";
                "p1_01_variables.md", "p1_02_strings.md", "p1_03_conditions.md", "p1_04_loops.md", "p1_05_functions.md", "p1_06_operators.md", "p1_07_scriptblocks.md"),
            module!(Intermediate, "2 · Collections and Objects", "Arrays, hashtables, object pipelines and regex.", "powershell";
                "p2_01_collections.md", "p2_02_objects.md", "p2_03_text.md", "p2_04_pipeline_deep.md"),
            module!(Intermediate, "3 · Files and Windows", "Files, CSV, JSON, errors, and managing a Windows machine.", "powershell";
                "p3_01_files.md", "p3_02_errors.md", "p3_03_windows_admin.md", "p3_04_dates_culture.md"),
            module!(Advanced, "4 · Admin Automation", "Advanced functions, modules, scheduled tasks, remoting and AD.", "powershell";
                "p4_01_advanced_functions.md", "p4_02_scripts_modules.md", "p4_03_remoting_ad.md"),
            module!(Expert, "5 · Professional PowerShell", "Classes, .NET, testing, style and security.", "powershell";
                "p5_01_classes.md", "p5_02_professional.md"),
            module!(Expert, "6 · Building Tools and Apps", "Tool design, REST APIs, secrets, logging, desktop GUIs, CI and publishing.", "powershell";
                "p6_01_tool_design.md", "p6_02_apis.md", "p6_03_gui_shipping.md"),
            module!(Expert, "7 · PowerShell in Depth", "Streams and unrolling, .NET collections, error handling, runspaces, classes, and a configuration drift project.", "powershell";
                "e7_01_output_streams.md", "e7_02_dotnet_collections.md", "e7_03_errors_deep.md",
                "e7_04_parallel.md", "e7_05_classes_deep.md", "e7_06_project.md"),
            module!(Expert, "8 · Production PowerShell", "Pester and mocks, performance, security, PowerShell 7 and cross-platform, structured logging, and an access review project.", "powershell";
                "n8_01_testing.md", "n8_02_performance.md", "n8_03_security.md", "n8_04_ps7_crossplatform.md",
                "n8_05_logging.md", "n8_06_project.md"),
            module!(Expert, "9 · WPF Apps with MVVM", "XAML layout and events, data binding, view models and commands, and a help desk app project.", "powershell";
                "g9_01_wpf_xaml.md", "g9_02_binding_mvvm.md", "g9_03_project.md"),
            module!(Expert, "10 · 🎓 Beyond IT: Puzzles, Graphics and Sound", "Optional, beyond IT scope: backtracking puzzle solvers, generative bitmaps and SVG, and synthesising WAV audio.", "powershell";
                "b10_01_puzzles.md", "b10_02_graphics.md", "b10_03_sound.md"),
            build!(Intermediate, "🩺 System Health Monitor", "Check disks, services and uptime; write an HTML report with exit codes.", "powershell";
                "q1_01_health_monitor.md"),
            build!(Advanced, "👥 User Onboarding", "Turn HR's CSV into accounts, groups, passwords and welcome letters.", "powershell";
                "q2_01_onboarding.md"),
            build!(Expert, "🧰 IT Asset Tracker", "A tested PowerShell module and a WinForms app: who has which laptop, and when warranties end.", "powershell";
                "q3_01_asset_tracker.md"),
            // generated project ideas powershell
            build!(Intermediate, "🧩 Software Inventory Report", "List installed programs on a PC and export a tidy report.", "powershell"; "zp_01_software_inventory.md"),
            build!(Advanced, "🧩 Scheduled Disk Cleanup Task", "A safe cleanup script that registers itself as a scheduled task.", "powershell"; "zp_02_cleanup_task.md"),
        ],
    }
}

pub fn bash() -> Track {
    Track {
        id: "bash",
        name: "Bash & Linux",
        icon: "🐧",
        category: Category::Scripting,
        lang: Some(Lang::Bash),
        blurb: "The Linux command line and shell scripting: servers, cloud machines, containers and DevOps.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "The shell, commands, help and the Linux file system.", "bash";
                "l0_01_welcome.md", "l0_02_filesystem.md"),
            module!(Novice, "1 · Scripting Fundamentals", "Variables, quoting, tests, loops and functions.", "bash";
                "l1_01_variables.md", "l1_02_conditions.md", "l1_03_loops.md", "l1_04_functions.md", "l1_05_expansion.md", "l1_06_arrays.md"),
            module!(Intermediate, "2 · Text Power Tools", "Pipes, redirection, grep, sed, awk, find and archives.", "bash";
                "l2_01_pipes.md", "l2_02_grep_sed_awk.md", "l2_03_find.md", "l2_04_redirection_deep.md", "l2_05_regex.md"),
            module!(Intermediate, "3 · Linux Administration", "Permissions, processes, systemd, logs and networking.", "bash";
                "l3_01_permissions.md", "l3_02_processes.md", "l3_03_networking.md"),
            module!(Advanced, "4 · Automation and Servers", "Robust scripts, scheduling, packages, disks and hardening.", "bash";
                "l4_01_robust_scripts.md", "l4_02_cron.md", "l4_03_packages_storage.md"),
            module!(Expert, "5 · Containers and Expert Bash", "Docker, associative arrays, parallel jobs and Git.", "bash";
                "l5_01_containers.md", "l5_02_professional.md"),
            module!(Expert, "6 · Building CLI Tools", "Command-line conventions, getopts, subcommands, tests, ShellCheck, packaging and completion.", "bash";
                "l6_01_cli_design.md", "l6_02_quality.md", "l6_03_shipping.md"),
            module!(Expert, "7 · Bash in Depth", "printf and read, functions and scope, signals and jobs, subshells and eval, awk programs, and a task runner project.", "bash";
                "e7_01_printf_read.md", "e7_02_functions_scope.md", "e7_03_signals_jobs.md", "e7_04_subshells_eval.md", "e7_05_awk_programs.md", "e7_06_project.md"),
            module!(Expert, "8 · Production Bash", "HTTP APIs and JSON, portability, performance, testing, security, and a release script project.", "bash";
                "n8_01_http_json.md", "n8_02_portability.md", "n8_03_performance.md",
                "n8_04_testing.md", "n8_05_security.md", "n8_06_project.md"),
            module!(Expert, "9 · Terminal UIs", "Colours and cursor control, menus and key handling, whiptail, and a full-screen dashboard project.", "bash";
                "g9_01_ansi_tput.md", "g9_02_menus.md", "g9_03_project.md"),
            module!(Expert, "10 · 🎓 Beyond IT: Games, Generative Art and Interpreters", "Optional, beyond IT scope: terminal games, fixed-point fractals and mazes, and language interpreters in pure Bash.", "bash";
                "b10_01_games.md", "b10_02_generative.md", "b10_03_interpreters.md"),
            build!(Intermediate, "📊 Access Log Analyzer", "Who, what and when on a web server: a report from nginx logs.", "bash";
                "m1_01_log_analyzer.md"),
            build!(Advanced, "🛡 Server Security Audit", "Check SSH, accounts and sudo rules, and score a Linux server.", "bash";
                "m2_01_server_audit.md"),
            build!(Expert, "🎫 tix: Ticket Tracker CLI", "A git-style command-line app: subcommands, options, safe updates, tests, completion and make install.", "bash";
                "m3_01_tix.md"),
            // generated project ideas bash
            build!(Intermediate, "🧩 Dotfiles Installer", "Keep your shell setup in Git and install it on any Linux machine.", "bash"; "zp_01_dotfiles.md"),
            build!(Advanced, "🧩 Server Login Dashboard", "A message of the day showing a server's health at login.", "bash"; "zp_02_motd.md"),
        ],
    }
}

pub fn javascript() -> Track {
    Track {
        id: "javascript",
        name: "JavaScript",
        icon: "🟨",
        category: Category::Programming,
        lang: Some(Lang::JavaScript),
        blurb: "The language of the web, and with Node.js, of servers and tools too.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Your first JavaScript, run with Node.js.", "javascript";
                "j0_01_welcome.md", "j0_02_how_js_runs.md"),
            module!(Novice, "1 · Fundamentals", "Values, comparisons, decisions, loops and functions.", "javascript";
                "j1_01_values.md", "j1_02_control.md", "j1_03_functions.md",
                "j1_04_types_coercion.md", "j1_05_scope_hoisting.md", "j1_06_loops_iteration.md"),
            module!(Intermediate, "2 · Working with Data", "Arrays, objects, JSON and regular expressions.", "javascript";
                "j2_01_arrays.md", "j2_02_objects.md", "j2_03_regex.md",
                "j2_04_destructuring_copying.md", "j2_05_maps_sets.md", "j2_06_dates_intl.md"),
            module!(Intermediate, "3 · Real-World JavaScript", "Errors, classes, async code, files and npm.", "javascript";
                "j3_01_errors_classes.md", "j3_02_async.md", "j3_03_node.md"),
            module!(Advanced, "4 · The Web", "HTML, the DOM, HTTP and REST APIs.", "javascript";
                "j4_01_web.md", "j4_02_http.md"),
            module!(Expert, "5 · Professional JavaScript", "Testing, TypeScript and security.", "javascript";
                "j5_01_professional.md"),
            module!(Expert, "6 · Full-Stack Applications", "Your own web server, front-end state and rendering, auth, config and deployment.", "javascript";
                "j6_01_server.md", "j6_02_frontend.md", "j6_03_production.md"),
            module!(Advanced, "7 · JavaScript in Depth",
                "this and binding, prototypes and classes, modules, iterators and generators, the event loop, metaprogramming, error handling and functional patterns.",
                "javascript"; "l7_01_this_binding.md", "l7_02_prototypes_classes.md", "l7_03_modules.md",
                "l7_04_iterators_generators.md", "l7_05_event_loop.md", "l7_06_metaprogramming.md",
                "l7_07_errors_deep.md", "l7_08_functional.md", "l7_09_project_task_queue.md"),
            module!(Expert, "8 · Advanced Node.js",
                "Streams, worker threads and child processes, testing with node:test, binary data and crypto, performance and memory, security, observability and databases.",
                "javascript"; "m8_01_streams.md", "m8_02_workers_processes.md", "m8_03_testing.md",
                "m8_04_binary_crypto.md", "m8_05_performance_memory.md", "m8_06_security.md",
                "m8_07_observability.md", "m8_08_databases.md", "m8_09_project_backup.md"),
            module!(Expert, "9 · GUI Capstone: React and Desktop Apps",
                "Components and the virtual DOM, React state and reducers, hooks and effects, forms and schemas, routing and server state, accessibility, and desktop apps with Electron and Tauri.",
                "javascript"; "u9_01_components_vdom.md", "u9_02_react_basics.md", "u9_03_hooks_effects.md",
                "u9_04_forms_validation.md", "u9_05_routing_data.md", "u9_06_accessibility_styling.md",
                "u9_07_desktop_apps.md", "u9_08_project_asset_app.md"),
            module!(Expert, "10 · 🎓 Beyond IT: Games, 3D and Creative Coding",
                "Optional, beyond IT scope: canvas games, 3D graphics with three.js, and generative art and sound.",
                "javascript"; "g10_01_canvas_games.md", "g10_02_3d_graphics.md", "g10_03_creative_coding.md"),
            build!(Advanced, "📡 Uptime Monitor", "Parallel site checks, alerts, history and a status page.", "javascript";
                "k1_01_uptime_monitor.md"),
            build!(Expert, "🎫 Help Desk Ticket API", "A REST API with SLAs, validation and tests: the core of an ITSM tool.", "javascript";
                "k2_01_ticket_api.md"),
            build!(Expert, "🔖 Link Vault", "A full-stack app with no dependencies: your own router, safe file storage and a reactive front end.", "javascript";
                "k3_01_link_vault.md"),
            // generated project ideas javascript
            build!(Advanced, "🧩 Website Uptime Checker CLI", "Check a list of URLs in parallel with fetch and report response times.", "javascript"; "zp_01_uptime_cli.md"),
            build!(Intermediate, "🧩 Kanban Board in the Browser", "A drag-and-drop task board saved in localStorage.", "javascript"; "zp_02_kanban.md"),
        ],
    }
}

pub fn typescript() -> Track {
    Track {
        id: "typescript",
        name: "TypeScript",
        icon: "🔷",
        category: Category::Programming,
        lang: Some(Lang::TypeScript),
        blurb: "JavaScript with types: the standard for serious web, Node.js and cloud tooling code.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Types, inference, and how TypeScript runs.", "typescript";
                "t0_01_welcome.md", "t0_02_type_checking.md"),
            module!(Intermediate, "1 · The Type System", "Arrays, unions, literal types and object types.", "typescript";
                "t1_01_types.md", "t1_02_objects_interfaces.md", "t1_03_tuples_literals.md"),
            module!(Intermediate, "2 · Functions and Narrowing", "Generics, discriminated unions and type guards.", "typescript";
                "t2_01_functions_generics.md", "t2_02_narrowing.md", "t2_03_function_types.md"),
            module!(Advanced, "3 · Real-World TypeScript", "Classes, utility types, async code, APIs and modules.", "typescript";
                "t3_01_classes_utility.md", "t3_02_async_modules.md"),
            module!(Expert, "4 · Professional TypeScript", "tsconfig, testing, declarations, migration and runtime validation.", "typescript";
                "t4_01_professional.md"),
            module!(Expert, "5 · Application Architecture", "Domain modeling with types, ports and adapters, events, and full-stack TypeScript.", "typescript";
                "t5_01_domain.md", "t5_02_architecture.md", "t5_03_fullstack.md"),
            module!(Expert, "6 · The Type System in Depth", "Generics, mapped, conditional and template literal types, variance, brands and type-level programming.", "typescript";
                "t6_01_generics_deep.md", "t6_02_mapped_types.md", "t6_03_conditional_infer.md", "t6_04_template_literal_types.md",
                "t6_05_variance_branded.md", "t6_06_recursive_types.md", "t6_07_project_query_builder.md"),
            module!(Expert, "7 · TypeScript in the Real World", "Declaration files, decorators, tooling and packages, testing, runtime validation, typed errors and typed APIs.", "typescript";
                "t7_01_declaration_files.md", "t7_02_decorators.md", "t7_03_tooling_packages.md", "t7_04_testing.md",
                "t7_05_runtime_validation.md", "t7_06_typed_errors.md", "t7_07_typed_api.md", "t7_08_project_incidents.md"),
            module!(Expert, "8 · GUI Capstone: React and Tauri Desktop Apps", "Typed components and props, state stores, data fetching and forms, Tauri commands and events, and a keyboard-first desktop app core.", "typescript";
                "t8_01_react_typescript.md", "t8_02_state_stores.md", "t8_03_data_forms.md", "t8_04_tauri_ipc.md", "t8_05_project_asset_desk.md"),
            module!(Expert, "9 · 🎓 Beyond IT: Interpreters, Game Engines and Generative Art", "Optional, beyond IT scope: a parser and interpreter, a typed ECS game engine, and seeded simulations.", "typescript";
                "x9_01_interpreter.md", "x9_02_game_ecs.md", "x9_03_simulation_generative.md"),
            build!(Advanced, "🚨 Alert Router", "Route monitoring alerts to on-call teams: matchers, silences, de-duplication.", "typescript";
                "u1_01_alert_router.md"),
            build!(Expert, "🏢 Meeting Room Booking", "Holds that expire, confirmations and smart room search, with ports and adapters.", "typescript";
                "u2_01_room_booking.md"),
            // generated project ideas typescript
            build!(Intermediate, "🧩 Type-safe CSV Importer", "Parse CSV into typed records with validation errors per row.", "typescript"; "zp_01_csv_typed.md"),
            build!(Advanced, "🧩 Typed Inventory REST API", "A small JSON API for IT assets with node:http and TypeScript.", "typescript"; "zp_02_inventory_api.md"),
        ],
    }
}

pub fn html() -> Track {
    Track {
        id: "html",
        name: "HTML & CSS",
        icon: "🎨",
        category: Category::Programming,
        lang: Some(Lang::Html),
        blurb: "Build web pages: structure with HTML, style and layout with CSS. Every IT person edits a page sooner or later.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Elements, tags and the skeleton of every page.", "html";
                "h0_01_welcome.md"),
            module!(Novice, "1 · HTML Content", "Links, images, lists, tables, forms, text semantics, media and validation.", "html";
                "h1_01_content.md", "h1_02_forms.md", "h1_03_text_semantics.md", "h1_04_media.md", "h1_05_forms_deep.md"),
            module!(Intermediate, "2 · CSS", "Selectors, the box model, flexbox and grid, the cascade, units, typography and colour.", "html";
                "h2_01_css_basics.md", "h2_02_layout.md", "h2_03_cascade.md", "h2_04_selectors.md", "h2_05_typography.md"),
            module!(Advanced, "3 · Modern Web Pages", "Responsive design, dark mode, semantics, accessibility, positioning and stacking.", "html";
                "h3_01_responsive.md", "h3_02_accessibility.md", "h3_03_positioning.md"),
            module!(Expert, "4 · Professional Front End", "Web security (XSS, CSP), performance and publishing.", "html";
                "h4_01_professional.md"),
            module!(Expert, "5 · Web Applications", "The DOM, events, state and render, fetch, storage, routing, components, accessibility and PWAs.", "html";
                "h5_01_dom_state.md", "h5_02_data.md", "h5_03_apps.md"),
            module!(Expert, "6 · CSS in Depth", "Grid areas and subgrid, transitions and keyframes, container queries and logical properties, tokens, BEM and cascade layers, SVG icons, and a dashboard.", "html";
                "e6_01_grid_deep.md", "e6_02_animation.md", "e6_03_modern_responsive.md", "e6_04_architecture.md", "e6_05_svg_icons.md", "e6_06_project.md"),
            module!(Expert, "7 · Production Front End", "Accessibility patterns and focus, Core Web Vitals and loading, form validation UX, testing and audits, SEO, i18n and Japanese typography, and an accessible request form.", "html";
                "n7_01_a11y_deep.md", "n7_02_performance.md", "n7_03_forms_ux.md", "n7_04_testing.md", "n7_05_seo_i18n.md", "n7_06_project.md"),
            module!(Expert, "8 · Web Components and Tooling", "Custom elements, shadow DOM and slots, ES modules and import maps, npm and Vite, and a component-based status board.", "html";
                "g8_01_web_components.md", "g8_02_modules_tooling.md", "g8_03_project.md"),
            module!(Expert, "9 · 🎓 Beyond IT: Canvas, CSS Art and Games", "Optional, beyond IT scope: generative art with canvas, illustrations in pure CSS, and an original arcade game.", "html";
                "b9_01_canvas_art.md", "b9_02_css_art.md", "b9_03_game.md"),
            build!(Intermediate, "🪪 Portfolio and Résumé Site", "Your own responsive site with dark mode and a print-ready résumé.", "html";
                "i1_01_portfolio.md"),
            build!(Expert, "📅 Habit Tracker PWA", "An installable, offline web app with tested logic, local storage, import/export and full keyboard support.", "html";
                "i2_01_habits.md"),
            // generated project ideas html
            build!(Advanced, "🧩 Searchable IT Handbook", "A multi-page help site for staff, with a search box and print styles.", "html"; "zp_01_it_handbook.md"),
            build!(Intermediate, "🧩 Service Status Page", "A public status page for your company's IT services.", "html"; "zp_02_status_page.md"),
        ],
    }
}

pub fn go() -> Track {
    Track {
        id: "go",
        name: "Go",
        icon: "🐹",
        category: Category::Programming,
        lang: Some(Lang::Go),
        blurb: "The language of cloud infrastructure: Docker, Kubernetes and Terraform are written in Go.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Your first Go program, variables and Go's strict compiler.", "go";
                "g0_01_welcome.md", "g0_02_toolchain_constants.md"),
            module!(Novice, "1 · Fundamentals", "Types, strings, control flow, functions and errors.", "go";
                "g1_01_basics.md", "g1_02_functions_errors.md", "g1_03_pointers_values.md", "g1_04_strings_runes.md"),
            module!(Intermediate, "2 · Data", "Slices, maps, structs, methods and interfaces.", "go";
                "g2_01_slices_maps.md", "g2_02_structs_interfaces.md", "g2_03_slices_deep.md"),
            module!(Advanced, "3 · Concurrency and Networks", "Goroutines, channels, worker pools, HTTP and JSON.", "go";
                "g3_01_concurrency.md", "g3_02_http_json.md"),
            module!(Expert, "4 · Professional Go", "Modules, table-driven tests, generics and shipping binaries.", "go";
                "g4_01_professional.md"),
            module!(Expert, "5 · Building Services", "Go 1.22 routing, JSON, middleware, project structure, errors, databases, logging, timeouts and containers.", "go";
                "g5_01_services.md", "g5_02_structure.md", "g5_03_operations.md"),
            module!(Expert, "6 · Go in Depth", "Interfaces, generics, iterators, errors, concurrency patterns, context and sync, reflection and unsafe.", "go";
                "d6_01_interfaces_deep.md", "d6_02_generics_deep.md", "d6_03_iterators.md", "d6_04_errors_deep.md", "d6_05_concurrency_patterns.md", "d6_06_context_sync.md", "d6_07_reflection_unsafe.md", "d6_08_project.md"),
            module!(Expert, "7 · Production Go", "Testing, benchmarks and fuzzing, CLIs, databases, protobuf and gRPC, performance, structured logging and a production HTTP service.", "go";
                "p7_01_testing.md", "p7_02_cli.md", "p7_03_database.md", "p7_04_grpc_protobuf.md", "p7_05_performance.md", "p7_06_slog.md", "p7_07_project.md"),
            module!(Expert, "8 · GUI Capstone: Desktop Apps with Fyne", "Fyne windows, widgets and layouts, data binding and view-models, background work on the UI thread, and a server dashboard app.", "go";
                "f8_01_fyne_basics.md", "f8_02_binding_mvvm.md", "f8_03_ui_concurrency.md", "f8_04_project.md"),
            module!(Expert, "9 · 🎓 Beyond IT: Embedded, Games and Generative Code", "Optional, beyond IT scope: TinyGo on microcontrollers, 2D games with Ebitengine, and procedural generation.", "go";
                "b9_01_tinygo.md", "b9_02_ebitengine.md", "b9_03_generative.md"),
            build!(Advanced, "🩺 Service Health Prober", "Check every service at once: TCP and HTTP, a worker pool, timeouts, percentiles.", "go";
                "h1_01_health_prober.md"),
            build!(Expert, "📇 Device Inventory REST API", "A standard-library Go service: routing, validation, error mapping, middleware, safe storage and tests.", "go";
                "h2_01_device_api.md"),
            // generated project ideas go
            build!(Intermediate, "🧩 URL Shortener Service", "A small HTTP service with JSON, a map behind a mutex, and tests.", "go"; "zp_01_url_shortener.md"),
            build!(Advanced, "🧩 Multi-file Log Follower", "tail -f for many files at once, with filters and colours.", "go"; "zp_02_log_tail.md"),
        ],
    }
}

pub fn csharp() -> Track {
    Track {
        id: "csharp",
        name: "C#",
        icon: "🟪",
        category: Category::Programming,
        lang: Some(Lang::CSharp),
        blurb: "Microsoft's .NET language: business systems, web APIs, Azure, Windows tools and Unity games.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Your first C# programs, variables and compiler errors.", "csharp";
                "c0_01_welcome.md", "c0_02_dotnet_toolchain.md"),
            module!(Novice, "1 · Fundamentals", "Types, text, decisions, loops and methods.", "csharp";
                "c1_01_types.md", "c1_02_control.md", "c1_03_methods.md", "c1_04_strings_deep.md", "c1_05_value_reference.md"),
            module!(Intermediate, "2 · Data and Objects", "Lists, dictionaries, LINQ, classes, records and interfaces.", "csharp";
                "c2_01_collections.md", "c2_02_classes.md", "c2_03_generics_collections.md"),
            module!(Advanced, "3 · Real-World C#", "Exceptions, files, JSON, async code and HTTP.", "csharp";
                "c3_01_errors_files.md", "c3_02_async.md"),
            module!(Expert, "4 · Professional .NET", "Projects, NuGet, testing, dependency injection and Web APIs.", "csharp";
                "c4_01_professional.md"),
            module!(Expert, "5 · Application Development", "Clean architecture, EF Core and ASP.NET Core, MVVM desktop apps and shipping.", "csharp";
                "c5_01_architecture.md", "c5_02_data_web.md", "c5_03_desktop.md"),
            module!(Expert, "6 · C# in Depth", "OOP in depth, delegates and events, LINQ internals, pattern matching, nullable references, async in depth, reflection and attributes.", "csharp";
                "e6_01_oop_deep.md", "e6_02_delegates_events.md", "e6_03_linq_deep.md", "e6_04_pattern_matching.md", "e6_05_nullable.md", "e6_06_async_deep.md", "e6_07_reflection_attributes.md", "e6_08_project.md"),
            module!(Expert, "7 · Production .NET", "Testing, dependency injection and configuration, EF Core, ASP.NET Core, Blazor, performance, and a hosted background service.", "csharp";
                "n7_01_testing.md", "n7_02_di_config.md", "n7_03_efcore.md", "n7_04_minimal_apis.md", "n7_05_blazor.md", "n7_06_performance.md", "n7_07_project.md"),
            module!(Expert, "8 · Desktop Apps in Depth", "XAML data binding, async commands and navigation, live collections and the UI thread, and an asset desk app with validation and undo.", "csharp";
                "g8_01_xaml_binding.md", "g8_02_commands_navigation.md", "g8_03_collections_threading.md", "g8_04_project.md"),
            module!(Expert, "9 · 🎓 Beyond IT: Game Development", "Optional, beyond IT scope: Unity's lifecycle and fixed timestep, coroutines and AI state machines, Godot, MonoGame and procedural generation.", "csharp";
                "b9_01_unity.md", "b9_02_coroutines_ai.md", "b9_03_procgen.md"),
            build!(Advanced, "🛡 Patch Compliance Reporter", "Check every machine against the security baseline: builds, patches, encryption, AV.", "csharp";
                "d1_01_patch_report.md"),
            build!(Expert, "🧾 Invoice Manager", "Japan's qualified-invoice tax rules, numbering, payments and an aging report.", "csharp";
                "d2_01_invoices.md"),
            // generated project ideas csharp
            build!(Advanced, "🧩 Log Analyzer with LINQ", "Parse application logs and answer questions with LINQ queries.", "csharp"; "zp_01_event_log_parser.md"),
            build!(Intermediate, "🧩 Password Generator and Strength Meter", "Generate strong passwords and passphrases, and rate existing ones.", "csharp"; "zp_02_password_tool.md"),
        ],
    }
}

pub fn cpp() -> Track {
    Track {
        id: "cpp",
        name: "C++",
        icon: "⚙",
        category: Category::Programming,
        lang: Some(Lang::Cpp),
        blurb: "Close to the machine: performance, embedded systems, games, and the code under most other software.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Your first C++ program, compiling, and variables.", "cpp";
                "x0_01_welcome.md", "x0_02_toolchain.md"),
            module!(Novice, "1 · Fundamentals", "Types, bits, strings, input, decisions, loops and functions.", "cpp";
                "x1_01_types.md", "x1_02_control_functions.md", "x1_03_references_pointers.md", "x1_04_text.md"),
            module!(Intermediate, "2 · The Standard Library", "Vectors, maps, algorithms, lambdas, classes and RAII.", "cpp";
                "x2_01_containers.md", "x2_02_classes.md", "x2_03_stl_deep.md"),
            module!(Advanced, "3 · Memory and Errors", "Pointers, smart pointers, move semantics, exceptions and files.", "cpp";
                "x3_01_memory.md", "x3_02_errors_files.md"),
            module!(Expert, "4 · Expert C++", "Templates, concepts, builds, threads and writing safe C++.", "cpp";
                "x4_01_templates.md", "x4_02_professional.md"),
            module!(Expert, "5 · Systems Applications", "Designing, measuring and shipping real C++ programs: interfaces, performance, undo, GUIs and networking.", "cpp";
                "x5_01_design.md", "x5_02_performance.md", "x5_03_apps.md"),
            module!(Expert, "6 · C++ in Depth", "Special members and the rule of five, polymorphism and variant, move semantics, templates and constexpr, exception safety, concurrency, ranges and coroutines.", "cpp";
                "e6_01_classes_deep.md", "e6_02_polymorphism.md", "e6_03_move_semantics.md", "e6_04_templates_deep.md", "e6_05_errors_deep.md", "e6_06_concurrency_deep.md", "e6_07_ranges_coroutines.md", "e6_08_project.md"),
            module!(Expert, "7 · Production C++", "CMake and dependency resolution, test frameworks and property tests, undefined behaviour and sanitizers, network protocols, allocators and data layout, C interop, and a log shipping agent.", "cpp";
                "n7_01_builds_dependencies.md", "n7_02_testing.md", "n7_03_ub_safety.md", "n7_04_networking.md", "n7_05_memory_layout.md", "n7_06_interop.md", "n7_07_project.md"),
            module!(Expert, "8 · Desktop GUIs: Qt and Dear ImGui", "Signals and slots, the object tree, model/view with proxies, QML-style bindings, immediate-mode UIs, and a server monitor with an event loop and settings.", "cpp";
                "g8_01_qt_basics.md", "g8_02_qt_models.md", "g8_03_imgui.md", "g8_04_project.md"),
            module!(Expert, "9 · 🎓 Beyond IT: Games, Embedded and SIMD", "Optional, beyond IT scope: an ECS and collisions with raylib/SDL, microcontroller registers and interrupts, and SIMD/SWAR bit tricks.", "cpp";
                "b9_01_games.md", "b9_02_embedded.md", "b9_03_simd.md"),
            build!(Advanced, "🧱 Firewall Rule Engine", "A first-match ACL engine with CIDR matching and shadowed-rule detection.", "cpp";
                "y1_01_firewall.md"),
            build!(Expert, "🗄 Mini Key-Value Database", "An in-memory store with a write-ahead log, crash-safe replay and compaction.", "cpp";
                "y2_01_kvstore.md"),
            // generated project ideas cpp
            build!(Intermediate, "🧩 Network Packet Header Parser", "Decode Ethernet, IPv4 and TCP headers from raw bytes.", "cpp"; "zp_01_packet_parser.md"),
            build!(Advanced, "🧩 Ring Buffer Logger", "A fixed-size, fast in-memory log that never allocates after start.", "cpp"; "zp_02_ring_logger.md"),
        ],
    }
}

pub fn sql() -> Track {
    Track {
        id: "sql",
        name: "SQL & Databases",
        icon: "🗄",
        category: Category::Data,
        lang: Some(Lang::Sql),
        blurb: "Ask questions of data. Every business runs on databases, and SQL is how you talk to them.",
        modules: vec![
            module!(Novice, "0 · Getting Started", "Tables, rows, columns and your first SELECT.", "sql";
                "s0_01_welcome.md"),
            module!(Novice, "1 · Asking Questions", "Filtering, sorting, calculating and grouping, NULL logic and data types.", "sql";
                "s1_01_where.md", "s1_02_sorting.md", "s1_03_aggregates.md", "s1_04_null.md", "s1_05_types.md"),
            module!(Intermediate, "2 · Relationships and Changes", "Joins (self, outer, semi and anti), set operations, changing data, transactions and table design.", "sql";
                "s2_01_joins.md", "s2_02_modify.md", "s2_03_create.md", "s2_04_joins_deep.md", "s2_05_sets.md"),
            module!(Advanced, "3 · Powerful Queries", "Subqueries, CTEs and recursive CTEs, views, text, dates and CASE.", "sql";
                "s3_01_subqueries.md", "s3_02_functions.md", "s3_03_recursive.md", "s3_04_views.md"),
            module!(Advanced, "4 · Analytics", "Window functions: rankings, running totals, top N per group.", "sql";
                "s4_01_window.md"),
            module!(Expert, "5 · Professional Databases", "SQL injection, permissions, normalization and DBA basics.", "sql";
                "s5_01_professional.md"),
            module!(Expert, "6 · Databases for Applications", "Schema design and constraints, indexes and query plans, pagination, transactions, UPSERT, triggers and migrations.", "sql";
                "s6_01_schema.md", "s6_02_performance.md", "s6_03_app_patterns.md"),
            module!(Expert, "7 · SQL in Depth", "Window frames and gaps and islands, JSON, full-text search, dates and time zones, constraints and triggers, and an SLA report.", "sql";
                "e7_01_windows_deep.md", "e7_02_json.md", "e7_03_text_search.md", "e7_04_dates.md", "e7_05_triggers_constraints.md", "e7_06_project.md"),
            module!(Expert, "8 · Production Databases", "Isolation and locking, query tuning, database security, PostgreSQL and other dialects, backups, replication and migrations.", "sql";
                "n8_01_transactions.md", "n8_02_tuning.md", "n8_03_security.md", "n8_04_postgres.md", "n8_05_operations.md", "n8_06_project.md"),
            module!(Expert, "9 · Analytics and Data Warehousing", "Star schemas, ETL and slowly changing dimensions, data quality, funnels, cohorts and retention.", "sql";
                "g9_01_star_schema.md", "g9_02_etl.md", "g9_03_project.md"),
            module!(Expert, "10 · 🎓 Beyond IT: Fractals, Graphs and Statistics", "Optional, beyond IT scope: fractals and cellular automata in recursive SQL, graph algorithms and the Game of Life, and statistics.", "sql";
                "b10_01_fractals.md", "b10_02_graphs.md", "b10_03_statistics.md"),
            build!(Intermediate, "🗃 IT Asset Inventory", "A real inventory database: schema, reports and a safe Python CLI.", "sql";
                "t1_01_asset_inventory.md"),
            build!(Expert, "🛒 Shop Order Database", "Write the SQL behind an online shop: no overselling, optimistic locking, triggers, keyset search and rankings.", "sql";
                "t2_01_shop.md"),
            // generated project ideas sql
            build!(Advanced, "🧩 Sales Analytics with Window Functions", "Answer business questions with CTEs and window functions.", "sql"; "zp_01_sales_analytics.md"),
            build!(Intermediate, "🧩 Help Desk Ticket Database", "Design a ticketing database and write the reports a manager asks for.", "sql"; "zp_02_helpdesk_db.md"),
        ]
        .into_iter()
        .map(|mut m| {
            m.lessons = m.lessons.into_iter().map(|l| l.with_setup(COMPANY_DB)).collect();
            m
        })
        .collect(),
    }
}

/// Sakura Systems K.K.: the sample database every SQL lesson starts with.
const COMPANY_DB: &str = include_str!("../../content/sql/company_db.sql");

/// IT foundations, certifications and the Japan career track: knowledge more than code.
pub fn gui() -> Track {
    Track {
        id: "gui",
        name: "GUI Design & Creation",
        icon: "🖌",
        category: Category::Extra,
        lang: Some(Lang::Html),
        blurb: "Build windows and apps people enjoy using: GUIs in many languages, colour and type, interactive controls, cute animations and your own icons.",
        modules: vec![
            module!(Novice, "0 · GUI Foundations", "Windows, widgets, the event loop, how layout engines size flex and grid, and keeping logic out of the window.", "gui";
                "gu0_01_foundations.md", "gu0_02_layout_engines.md"),
            module!(Intermediate, "1 · Desktop GUIs in Every Language", "WinForms, WPF and XAML binding, PowerShell windows, Electron and Tauri, Qt and Fyne.", "gui";
                "gu1_01_desktop_languages.md"),
            module!(Intermediate, "2 · Making It Pretty", "Colour, spacing scales, typography, depth, light and dark themes, fluid sizes and container queries.", "gui";
                "gu2_01_pretty.md", "gu2_02_responsive.md"),
            module!(Intermediate, "3 · Interactive Controls", "Button states, toggles, sliders, toasts, spinners, keyboard access, accessible dialogs and tabs.", "gui";
                "gu3_01_interactive.md", "gu3_02_accessible_widgets.md"),
            module!(Advanced, "4 · Cute Animations", "Transitions, keyframes, easing, micro-interactions and animating desktop GUIs.", "gui";
                "gu4_01_css_animation.md", "gu4_02_animation_code.md"),
            module!(Advanced, "5 · Icon Creation", "Hand-written SVG icons, a consistent set, animated icons, and exporting app icons.", "gui";
                "gu5_01_svg_icons.md", "gu5_02_icon_export.md"),
            build!(Advanced, "🌱 Plant Buddy App", "A polished plant-care app with a hand-drawn icon set, light and dark themes, toasts and watering animations.", "gui";
                "gu9_01_plant_buddy.md"),
        ],
    }
}

/// Living in Linux on your own computer: the desktop side, next to the Linux admin certification track.
pub fn linux_env() -> Track {
    Track {
        id: "linuxenv",
        name: "Linux Desktop & Environment",
        icon: "🐧",
        category: Category::Extra,
        lang: Some(Lang::Bash),
        blurb: "Use Linux as your everyday system: choose and install a distro, the terminal and files, software and Flatpak, customising, drivers, gaming, backups and fixing problems.",
        modules: vec![
            module!(Novice, "0 · Choosing and Installing", "Distros and families, desktop environments, Wayland, live USBs, disks and partitions, fstab, UEFI and GRUB, dual boot and the first hour.", "linuxenv";
                "le0_01_choose_install.md", "le0_02_disks_boot.md"),
            module!(Novice, "1 · The Terminal and Files", "The shell, the filesystem tree, finding things, pipes, permissions, grep, sed and awk, users and sudo, firewalls, SSH keys and encryption.", "linuxenv";
                "le1_01_shell_files.md", "le1_02_text_tools.md", "le1_03_users_security.md"),
            module!(Intermediate, "2 · Installing Software", "apt, dnf, pacman and zypper, Flatpak, Snap, AppImage, versions, file ownership, cleaning up and keeping it updated.", "linuxenv";
                "le2_01_software.md", "le2_02_packages_deep.md"),
            module!(Intermediate, "3 · Making It Yours", "Themes, fonts and Japanese input, launchers, autostart and user services, .bashrc, dotfiles and reliable shell scripts.", "linuxenv";
                "le3_01_customise.md", "le3_02_scripting.md"),
            module!(Advanced, "4 · Hardware, Gaming and Fixing Things", "Drivers, audio, Steam and Proton, backups, logs, systemd services and timers, network diagnosis and a method for fixing anything, Wayland, monitors and scaling, PipeWire and Bluetooth audio.", "linuxenv";
                "le4_01_hardware_fix.md", "le4_02_systemd_network.md", "le4_03_display_audio.md"),
        ],
    }
}

/// Image editing with GIMP; the exercises rebuild what its tools do to pixels.
pub fn gimp() -> Track {
    Track {
        id: "gimp",
        name: "GIMP Image Editing",
        icon: "🎨",
        category: Category::Extra,
        lang: Some(Lang::Python),
        blurb: "Edit photos and make graphics with GIMP 3: layers, selections and masks, colour correction, retouching, text, filters and automation with Python.",
        modules: vec![
            module!(Novice, "0 · GIMP Basics", "The interface and tools, pixels and print size, image modes, bit depth, compression, formats and metadata, saving XCF and exporting, guides, measuring, alignment and paths.", "gimp";
                "gm0_01_basics.md", "gm0_02_formats_compression.md", "gm0_03_precision_paths.md"),
            module!(Intermediate, "1 · Layers, Selections and Masks", "The layer stack, blend modes, every selection tool and mode, layer masks, transforms and resampling, cutouts, Color to Alpha and chroma keying.", "gimp";
                "gm1_01_layers_selections.md", "gm1_02_transforms_resampling.md", "gm1_03_cutouts_alpha.md"),
            module!(Intermediate, "2 · Colour and Retouching", "Histograms, Levels and Curves, black and white, hue and saturation, healing and cloning, sharpening and noise reduction, white balance, ICC profiles and print.", "gimp";
                "gm2_01_colour_retouch.md", "gm2_02_sharpen_denoise.md", "gm2_03_white_balance_colour_management.md"),
            module!(Advanced, "3 · Text, Filters and Automation", "Text and painting, how filters work, shadows, outlines and glows, legible text and scrims, exporting, batch jobs and Python plug-ins, indexed colour, dithering, sprite sheets and responsive exports.", "gimp";
                "gm3_01_filters_automation.md", "gm3_02_effects_legibility.md", "gm3_03_web_indexed_dither.md"),
        ],
    }
}

/// Beautiful Rust desktop apps: themes, animated buttons, icons and polish (egui, iced, Slint).
pub fn rust_ui() -> Track {
    Track {
        id: "rustui",
        name: "Rust UI & Theme Design",
        icon: "🦀",
        category: Category::Extra,
        lang: Some(Lang::Rust),
        blurb: "Make Rust apps beautiful: egui, iced and Slint, pretty light and dark themes, interactive animated buttons, icons and app icons, layout and polish.",
        modules: vec![
            module!(Intermediate, "0 · Rust GUI Toolkits", "egui, iced, Slint and Tauri, immediate vs retained mode, widget states, hit testing, the Elm architecture, undo and background work, AccessKit and keyboard navigation.", "rustui";
                "ru0_01_landscape.md", "ru0_02_app_architecture.md", "ru0_03_accessibility_keyboard.md"),
            module!(Intermediate, "1 · Pretty Themes", "Palettes and design tokens, contrast, derived state colours, fonts, egui Visuals and iced themes, HSL and linear-light colour maths.", "rustui";
                "ru1_01_themes.md", "ru1_02_colour_maths.md"),
            module!(Advanced, "2 · Interactive Animated Buttons", "Easing, interruptible animations, springs, a custom animated egui button, gestures and kinetic scrolling.", "rustui";
                "ru2_01_animated_buttons.md", "ru2_02_input_gestures.md"),
            module!(Advanced, "3 · Icons and App Icons", "Icon fonts, SVG icons, icons generated and painted in code, and the executable's icon, curve flattening, fill rules and hit testing.", "rustui";
                "ru3_01_icons.md", "ru3_02_vector_paths.md"),
            module!(Advanced, "4 · Layout and Polish", "Flex layout, text that fits, word wrapping, virtual lists, keyboard focus, accessibility, DPI scaling, breakpoints and the polish checklist.", "rustui";
                "ru4_01_layout_polish.md", "ru4_02_text_virtual_lists.md", "ru4_03_dpi_responsive.md"),
        ],
    }
}

/// Vector UI design with Penpot and Inkscape: icons, buttons, components and themes.
pub fn vector_ui() -> Track {
    Track {
        id: "vectorui",
        name: "Vector UI Design: Penpot & Inkscape",
        icon: "✒",
        category: Category::Extra,
        lang: Some(Lang::Html),
        blurb: "Design app screens, buttons, icons and themes with Penpot (the open-source Figma) and Inkscape (the open-source Illustrator), and turn them into real SVG and CSS.",
        modules: vec![
            module!(Novice, "0 · Vector Basics", "Vector vs raster, SVG anatomy, path commands and Béziers, viewBox and transforms, setting up Penpot (or self-hosting it) and Inkscape.", "vectorui";
                "vu0_01_vector_basics.md", "vu0_02_paths_coordinates.md"),
            module!(Intermediate, "1 · Penpot", "Boards, flex and grid layout, type and spacing scales, variant matrices, hug and fill sizing, tokens, prototypes and Inspect.", "vectorui";
                "vu1_01_penpot.md", "vu1_02_type_spacing.md", "vu1_03_components_layout.md"),
            module!(Intermediate, "2 · A Button System", "Variants, sizes, states and micro-interactions, then text fields, checkboxes and switches, from Penpot components to CSS.", "vectorui";
                "vu2_01_button_system.md", "vu2_02_form_controls.md"),
            module!(Advanced, "3 · Inkscape Icons", "Paths and nodes, boolean operations, a consistent icon grid, keylines and pixel snapping, clean SVG export, optimisation, sprites and accessible icons.", "vectorui";
                "vu3_01_inkscape.md", "vu3_02_clean_accessible_svg.md", "vu3_03_icon_geometry.md"),
            module!(Advanced, "4 · Themes and Design Systems", "Three layers of design tokens, tonal scales and contrast, light and dark themes, exporting tokens to CSS and Rust.", "vectorui";
                "vu4_01_themes_tokens.md", "vu4_02_colour_systems.md"),
        ],
    }
}

pub fn ui_anim() -> Track {
    Track {
        id: "uianim",
        name: "UI Animation: Synfig & Glaxnimate",
        icon: "✨",
        category: Category::Extra,
        lang: Some(Lang::Python),
        blurb: "Animated icons, buttons and micro-interactions: motion principles, Synfig Studio's tweening and bones, Glaxnimate and the Lottie format, and playing animations in web, Tauri and Rust apps.",
        modules: vec![
            module!(Novice, "0 · Motion for Interfaces", "Animation principles for UI, keyframes and interpolation, easing and springs, timing and choreography, colour interpolation, formats, and planning a micro-interaction.", "uianim";
                "ua0_01_motion.md", "ua0_02_easing_springs.md", "ua0_03_timing_choreography.md"),
            module!(Intermediate, "1 · Synfig Studio", "Layers, waypoints and interpolation, path morphing, bones with forward and inverse kinematics, motion paths at constant speed, linked parameters and rendering.", "uianim";
                "ua1_01_synfig.md", "ua1_02_bones_kinematics.md", "ua1_03_motion_paths.md"),
            module!(Intermediate, "2 · Glaxnimate and Lottie", "Keyframes, easing, trim paths and precomps, the Lottie JSON format inside out, native CSS and SVG animation, FLIP and view transitions.", "uianim";
                "ua2_01_glaxnimate.md", "ua2_02_css_svg_animation.md", "ua2_03_flip_view_transitions.md"),
            module!(Advanced, "3 · Animations in Real Apps", "dotlottie-web, lottie-web, Rust players, state-machine buttons, sprite sheets, frame budgets, compositing, reduced motion, loading states and progress.", "uianim";
                "ua3_01_lottie_in_apps.md", "ua3_02_performance_a11y.md", "ua3_03_loading_feedback.md"),
        ],
    }
}

pub fn blender() -> Track {
    Track {
        id: "blender",
        name: "Blender: 3D, VFX & Motion Graphics",
        icon: "🧊",
        category: Category::Extra,
        lang: Some(Lang::Python),
        blurb: "The full Blender suite: modeling, shading and rendering, animation and rigging, Geometry Nodes, smoke, fire, fluid and physics simulations, Grease Pencil, motion tracking, the node Compositor, the Video Sequence Editor and Python scripting.",
        modules: vec![
            module!(Novice, "0 · Interface and 3D Space", "Editors and workspaces, navigation, transforms, parenting, collections, the coordinate system, matrices and quaternions.", "blender";
                "bl0_01_interface.md", "bl0_02_3d_maths.md"),
            module!(Novice, "1 · Modeling", "Edit Mode tools, clean topology, mesh data and normals, manifold checks, the modifier stack, hard surface, sculpting, UVs, curves and text.", "blender";
                "bl1_01_modeling.md", "bl1_02_mesh_data.md"),
            module!(Intermediate, "2 · Shading and Rendering", "Principled BSDF, shader nodes, procedural materials, lighting, lenses and depth of field, EEVEE, Cycles and render budgets.", "blender";
                "bl2_01_shading_rendering.md", "bl2_02_camera_render_maths.md"),
            module!(Intermediate, "3 · Animation and Rigging", "Keyframes, the Graph Editor, drivers, constraints, armatures, FK/IK, Rigify and shape keys.", "blender";
                "bl3_01_animation_rigging.md"),
            module!(Intermediate, "4 · Geometry Nodes", "Fields, attributes, instances, scattering, simulation zones and procedural motion graphics.", "blender";
                "bl4_01_geometry_nodes.md"),
            module!(Advanced, "5 · Simulations and VFX", "Particles, Mantaflow fire, smoke and liquids, rigid body destruction, cloth and caching.", "blender";
                "bl5_01_simulations.md"),
            module!(Advanced, "6 · Grease Pencil", "2D animation in 3D: strokes, layers, onion skins, interpolation, modifiers and Line Art.", "blender";
                "bl6_01_grease_pencil.md"),
            module!(Advanced, "7 · Tracking and Compositing", "Camera and plane tracking, solving, the node Compositor, keying, Alpha Over and integration.", "blender";
                "bl7_01_tracking_compositing.md"),
            module!(Expert, "8 · VSE, Projects and Scripting", "The Video Sequence Editor, a motion graphics pipeline, bpy scripting and command-line rendering.", "blender";
                "bl8_01_vse_scripting.md"),
        ],
    }
}

pub fn resolve() -> Track {
    Track {
        id: "resolve",
        name: "DaVinci Resolve: Edit, Color, Fusion & Audio",
        icon: "🎬",
        category: Category::Extra,
        lang: Some(Lang::Python),
        blurb: "Post-production from first import to final delivery: editing on the Cut and Edit pages, Fusion VFX and motion graphics, colour grading and colour management, Fairlight audio, codecs and export, and scripting Resolve with Python.",
        modules: vec![
            module!(Novice, "0 · Pages, Projects and Timecode", "The pages, project settings, media management, proxies, frame rates and timecode.", "resolve";
                "dr0_01_basics.md"),
            module!(Novice, "1 · Editing", "The Cut and Edit pages, essential edits, trimming, storytelling, titles, speed, EDLs, sync, multicam and pacing.", "resolve";
                "dr1_01_editing.md", "dr1_02_sync_pacing.md"),
            module!(Intermediate, "2 · Fusion", "Node-based compositing, keyframes and modifiers, tracking, keying, macros and motion graphics.", "resolve";
                "dr2_01_fusion.md"),
            module!(Intermediate, "3 · Color Grading", "Nodes, lift/gamma/gain, secondaries, chart balancing, shot matching, skin tones, groups and LUTs.", "resolve";
                "dr3_01_color.md", "dr3_02_balance_matching.md"),
            module!(Advanced, "4 · Scopes and Colour Management", "Waveforms and vectorscopes, log footage, DaVinci Wide Gamut, ACES and HDR.", "resolve";
                "dr4_01_color_management.md"),
            module!(Advanced, "5 · Fairlight Audio", "Editing and mixing dialogue, music and effects, EQ, compression, ducking and loudness.", "resolve";
                "dr5_01_fairlight.md"),
            module!(Advanced, "6 · Deliver", "Codecs, bitrates and file sizes, export recipes, alpha, subtitles, frame-rate conversion, streaming ladders and quality checks.", "resolve";
                "dr6_01_deliver.md", "dr6_02_framerates_streaming.md"),
            module!(Expert, "7 · Scripting and Workflow", "The Python API, automation, collaboration, project structure and a full creative pipeline.", "resolve";
                "dr7_01_scripting_workflow.md"),
        ],
    }
}

pub fn knowledge() -> Vec<Track> {
    vec![
        Track {
            id: "foundations",
            name: "IT Foundations",
            icon: "🧭",
            category: Category::Foundations,
            lang: None,
            blurb: "Start here: how computers, operating systems and networks work, troubleshooting, security and the cloud.",
            modules: vec![
                module!(Novice, "0 · Computers", "Hardware, CPUs and memory, ports and mobile devices, binary, text encodings and files, operating systems and booting.", "foundations";
                    "f0_01_hardware.md", "f0_02_os.md", "f0_03_cpu_memory.md", "f0_04_devices.md", "f0_05_data.md"),
                module!(Novice, "1 · Networks", "IP addresses and subnets, IPv6, DNS, DHCP, ports, the OSI model, switches, routers, Wi-Fi, the web and email.", "foundations";
                    "f1_01_networks.md", "f1_02_ip_subnets.md", "f1_03_network_gear.md", "f1_04_web_email.md"),
                module!(Novice, "2 · Working in IT", "Troubleshooting, tickets, security, virtualization and cloud, accounts and permissions, the command line, backups, service management and careers.", "foundations";
                    "f2_01_troubleshooting.md", "f2_02_security_cloud.md", "f2_03_accounts.md", "f2_04_command_line.md", "f2_05_backup.md", "f2_06_itsm.md", "f2_07_careers.md"),
                module!(Novice, "3 · Data, Software and Automation", "Databases and data formats, how software is built, versioned and licensed, and scripting everyday IT tasks.", "foundations";
                    "f3_01_databases.md", "f3_02_software.md", "f3_03_automation.md"),
                build!(Novice, "🧪 Home Lab", "A virtual company network to practise everything on, with a lab notebook.", "foundations";
                    "f9_01_home_lab.md"),
                // generated project ideas foundations
                build!(Novice, "🧩 Home Lab Inventory and Network Map", "Document your own computers and home network like an IT admin.", "foundations"; "zp_01_home_lab.md"),
                build!(Novice, "🧩 PC Build Compatibility Checker", "Check a list of PC parts for compatibility and power needs.", "foundations"; "zp_02_pc_build.md"),
            ],
        },
        Track {
            id: "aplus",
            name: "CompTIA A+",
            icon: "🛠",
            category: Category::Cert,
            lang: None,
            blurb: "The entry-level IT support certification: hardware, Windows, networking, security and troubleshooting.",
            modules: vec![
                module!(Novice, "0 · The Exam", "What A+ covers and how to pass it.", "aplus"; "a0_01_about.md"),
                module!(Novice, "1 · Core 1 (220-1201)", "Hardware, motherboards and power, storage and RAID, mobile devices, networking, ports and DNS, servers and IoT, printers, virtualization and troubleshooting.", "aplus";
                    "a1_01_hardware.md", "a1_02_networking.md", "a1_03_printers_troubleshooting.md", "a1_04_virtualization_cloud.md", "a1_05_motherboard_power.md", "a1_06_storage_raid.md", "a1_07_mobile.md", "a1_08_network_config.md", "a1_09_network_services.md", "a1_10_troubleshooting.md"),
                module!(Intermediate, "2 · Core 2 (220-1202)", "Windows, other OSs, security and operational procedures: admin tools, installs and upgrades, networking and sharing, hardening, software and malware troubleshooting, scripting and remote access.", "aplus";
                    "a2_01_windows.md", "a2_02_other_os.md", "a2_03_security.md", "a2_04_operations.md",
                    "a2_05_windows_admin.md", "a2_06_os_install.md", "a2_07_windows_networking.md", "a2_08_security_deep.md",
                    "a2_09_software_troubleshooting.md", "a2_10_scripting_remote.md"),
                build!(Intermediate, "🩹 Help Desk Triage", "Prioritize calls, suggest likely causes and write ticket notes, in PowerShell.", "aplus";
                    "a9_01_triage.md"),
                // generated project ideas aplus
                build!(Intermediate, "🧩 PC Refresh Plan", "Plan replacing ten old office PCs: specs, data migration and disposal.", "aplus"; "zp_01_pc_refresh.md"),
                build!(Intermediate, "🧩 Help Desk Ticket Triage", "Sort incoming tickets by category and priority, with SLA deadlines.", "aplus"; "zp_02_ticket_triage.md"),
            ],
        },
        Track {
            id: "networkplus",
            name: "CompTIA Network+",
            icon: "🌐",
            category: Category::Cert,
            lang: None,
            blurb: "Networks in depth: protocols, subnetting, switching, routing, wireless, security and troubleshooting.",
            modules: vec![
                module!(Novice, "0 · The Exam", "What Network+ covers.", "networkplus"; "n0_01_about.md"),
                module!(Intermediate, "1 · Concepts", "OSI, ports and protocols, subnetting and IPv6, cabling and transceivers, DNS/DHCP/NTP in depth, data centre and cloud networking.", "networkplus";
                    "n1_01_osi_protocols.md", "n1_02_subnetting.md", "n1_03_cabling_media.md", "n1_04_dns_dhcp_time.md",
                    "n1_05_modern_networks.md"),
                module!(Intermediate, "2 · Implementation", "Switching, VLANs, routing, wireless, WAN and cloud networks, then routing protocols and NAT, trunks and spanning tree, and Wi-Fi design.", "networkplus";
                    "n2_01_switching_routing.md", "n2_02_wireless_wan_cloud.md", "n2_03_routing_nat.md", "n2_04_switching_deep.md",
                    "n2_05_wireless_deep.md"),
                module!(Advanced, "3 · Operations, Security, Troubleshooting", "Services, monitoring, attacks, defences and tools, plus metrics and logs, high availability and DR, ACLs and remote access.", "networkplus";
                    "n3_01_services_security.md", "n3_02_troubleshooting.md", "n3_03_monitoring.md", "n3_04_availability_dr.md",
                    "n3_05_network_security.md"),
                module!(Advanced, "4 · Troubleshooting in Depth", "The troubleshooting method, interface counters and optics, addressing, routing, service and performance problems.", "networkplus";
                    "n4_01_physical_troubleshooting.md", "n4_02_service_troubleshooting.md"),
                build!(Advanced, "🏢 Office Network Planner", "Check a VLAN plan, plan subnets with VLSM, and generate switch config.", "networkplus";
                    "n9_01_office_network.md"),
                // generated project ideas networkplus
                build!(Advanced, "🧩 Wi-Fi Channel Survey", "Parse nearby networks and recommend the least crowded channels.", "networkplus"; "zp_01_wifi_survey.md"),
                build!(Intermediate, "🧩 VLSM Subnet Planner", "Split an address block into right-sized subnets for each department.", "networkplus"; "zp_02_vlsm_planner.md"),
            ],
        },
        Track {
            id: "linux",
            name: "Linux+ (Linux Admin)",
            icon: "🐧",
            category: Category::Cert,
            lang: None,
            blurb: "Administer Linux servers: boot, storage, users, services, security, containers and troubleshooting.",
            modules: vec![
                module!(Intermediate, "0 · System Management", "The exam, boot process, kernel and file system layout.", "linux";
                    "lx0_01_boot.md"),
                module!(Intermediate, "1 · Storage, Users and Services", "Partitions, LVM, mounts, accounts, packages, systemd and logs, plus text processing, processes and scheduling, packages and the kernel, and storage operations.", "linux";
                    "lx1_01_storage.md", "lx1_02_users_services.md", "lx1_03_text_processing.md", "lx1_04_processes_scheduling.md",
                    "lx1_05_packages_kernel.md", "lx1_06_storage_ops.md"),
                module!(Advanced, "2 · Security and Automation", "Permissions, SELinux, firewalls, SSH, networking, containers, Ansible, then octal permissions, ACLs and sudo, authentication and hardening, and network configuration.", "linux";
                    "lx2_01_security.md", "lx2_02_networking_automation.md", "lx2_03_permissions_sudo.md", "lx2_04_auth_hardening.md",
                    "lx2_05_network_config.md"),
                module!(Advanced, "3 · Troubleshooting", "Performance, disks, services, networking and boot problems, with CPU/memory/I/O diagnosis and boot, service, access and SELinux fixes.", "linux";
                    "lx3_01_troubleshooting.md", "lx3_02_performance.md", "lx3_03_boot_services.md"),
                module!(Advanced, "4 · Scripting, Containers and Automation", "Bash scripting, Podman and Docker, Git, Ansible, infrastructure as code and CI/CD.", "linux";
                    "lx4_01_bash_scripting.md", "lx4_02_containers.md", "lx4_03_git_automation.md"),
                build!(Advanced, "🖥 Hardened Web Server", "Turn a fresh Ubuntu VM into a secure web server, by hand and with a script.", "linux";
                    "lx9_01_server_build.md"),
                // generated project ideas linux
                build!(Advanced, "🧩 Log Rotation and Disk Alerting", "Write logrotate rules and a disk-space alert that emails or logs.", "linux"; "zp_01_log_rotation.md"),
                build!(Intermediate, "🧩 Self-hosted Web Server Lab", "Set up and secure a Linux web server in a VM, documented as you go.", "linux"; "zp_02_lamp_lab.md"),
            ],
        },
        Track {
            id: "securityplus",
            name: "CompTIA Security+",
            icon: "🔐",
            category: Category::Cert,
            lang: None,
            blurb: "The baseline security certification: threats, cryptography, architecture, operations and governance.",
            modules: vec![
                module!(Intermediate, "1 · General Security Concepts", "Controls, zero trust, cryptography and PKI, the control matrix, physical security, change management, and applied cryptography (TLS, signatures, password storage, certificates).", "securityplus";
                    "sp0_01_concepts.md", "sp1_01_crypto.md", "sp1_02_controls_change.md", "sp1_03_crypto_applied.md"),
                module!(Intermediate, "2 · Threats and Vulnerabilities", "Threat actors, attacks, malware and indicators, social engineering and email security, application vulnerabilities and secure coding.", "securityplus";
                    "sp2_01_threats.md", "sp2_02_social_engineering.md", "sp2_03_app_vulnerabilities.md"),
                module!(Advanced, "3 · Architecture and Operations", "Secure design, data protection, IAM, monitoring and incident response, in depth: network architecture, cloud and OT, vulnerability management, identity, forensics and SOC tooling.", "securityplus";
                    "sp3_01_architecture.md", "sp3_02_secure_network.md", "sp3_03_cloud_iot_ot.md", "sp4_01_operations.md",
                    "sp4_02_vuln_management.md", "sp4_03_iam_deep.md", "sp4_04_incident_forensics.md", "sp4_05_secops_tools.md"),
                module!(Advanced, "4 · Program Management", "Governance, risk, third parties, compliance and audits, with SLE/ALE risk calculations, risk registers, BIA, vendor agreements and privacy roles.", "securityplus";
                    "sp5_01_governance.md", "sp5_02_risk_analysis.md", "sp5_03_third_party_compliance.md"),
                build!(Advanced, "🛡 Security Guard", "A file integrity monitor and a breach-aware password auditor.", "securityplus";
                    "sp9_01_guard.md"),
                // generated project ideas securityplus
                build!(Advanced, "🧩 Suspicious Email Header Analyzer", "Read an email's headers and explain the warning signs to a user.", "securityplus"; "zp_01_email_headers.md"),
                build!(Intermediate, "🧩 File Integrity Monitor", "Detect changed, added and deleted files with SHA-256 baselines.", "securityplus"; "zp_02_file_integrity.md"),
            ],
        },
        Track {
            id: "ccna",
            name: "Cisco CCNA",
            icon: "📡",
            category: Category::Cert,
            lang: None,
            blurb: "Configure Cisco routers and switches: IOS, VLANs, STP, OSPF, services, security, wireless and automation.",
            modules: vec![
                module!(Intermediate, "0 · The Exam and Fundamentals", "CCNA 200-301, the IOS command line, subnetting, IPv6 and how switches forward frames.", "ccna";
                    "c0_01_ios.md", "c0_02_subnetting_wildcards.md", "c0_03_ipv6.md", "c0_04_switching_fundamentals.md"),
                module!(Intermediate, "1 · Switching and Routing", "VLANs, trunks, STP elections, EtherChannel, route selection and OSPF in depth.", "ccna";
                    "c1_01_switching.md", "c1_02_routing.md", "c1_03_stp_etherchannel.md", "c1_04_ospf_route_selection.md"),
                module!(Advanced, "2 · Services, Security, Automation", "NAT, DHCP relay, QoS, syslog, ACLs, layer 2 security, AAA, WLC configuration, REST APIs and Ansible.", "ccna";
                    "c2_01_services_security.md", "c2_02_wireless_automation.md", "c2_03_ip_services_deep.md", "c2_04_security_deep.md",
                    "c2_05_wireless_deep.md", "c2_06_automation_deep.md"),
                module!(Advanced, "3 · Troubleshooting", "Find faults layer by layer with show commands, ping and traceroute.", "ccna";
                    "c3_01_troubleshooting.md"),
                build!(Advanced, "🏫 Campus Lab", "Build a two-VLAN campus in Packet Tracer, then parse its show output with Python.", "ccna";
                    "c9_01_campus_lab.md"),
                // generated project ideas ccna
                build!(Advanced, "🧩 Branch Router Config Generator", "Generate consistent configs for many sites from one template.", "ccna"; "zp_01_config_generator.md"),
                build!(Intermediate, "🧩 Router Config Backup and Diff", "Keep config backups and see exactly what changed between them.", "ccna"; "zp_02_config_diff.md"),
            ],
        },
        Track {
            id: "aws",
            name: "AWS Cloud",
            icon: "☁",
            category: Category::Cert,
            lang: None,
            blurb: "Cloud Practitioner to Solutions Architect: compute, storage, networking, databases, IAM, billing and design.",
            modules: vec![
                module!(Novice, "0 · Cloud Concepts", "The exams, global infrastructure, shared responsibility, Well-Architected.", "aws";
                    "aw0_01_cloud_concepts.md"),
                module!(Intermediate, "1 · Core Services", "EC2 purchasing and EBS, S3 classes and lifecycle, VPC routing and firewalls, RDS, Aurora and DynamoDB.", "aws";
                    "aw1_01_compute_storage.md", "aw1_02_network_databases.md", "aw1_03_ec2_deep.md", "aw1_04_s3_deep.md",
                    "aw1_05_vpc_deep.md", "aw1_06_databases_deep.md"),
                module!(Advanced, "2 · Security, Billing, Architecture", "IAM policies and Organizations, monitoring and operations, cost management, serverless and containers.", "aws";
                    "aw2_01_security_billing.md", "aw2_02_architecture.md", "aw2_03_iam_deep.md", "aw2_04_monitoring_ops.md",
                    "aw2_05_cost.md", "aw2_06_serverless_containers.md"),
                module!(Advanced, "3 · Solutions Architect Associate", "The SAA-C03 domains, scenario keywords, DR strategies and Route 53 routing.", "aws";
                    "aw3_01_saa_scenarios.md", "aw3_02_resilience_dr.md"),
                build!(Advanced, "📝 Serverless IT Request Form", "API Gateway + Lambda + DynamoDB with SAM, tested offline first.", "aws";
                    "aw9_01_serverless_form.md"),
                // generated project ideas aws
                build!(Advanced, "🧩 IAM Policy Linter", "Find wildcards and risky permissions in AWS IAM policy documents.", "aws"; "zp_01_iam_policy_lint.md"),
                build!(Intermediate, "🧩 AWS Cost and Tagging Report", "Find where the money goes, and which resources nobody owns.", "aws"; "zp_02_cost_report.md"),
            ],
        },
        Track {
            id: "ccnp",
            name: "Cisco CCNP Enterprise",
            icon: "🛰",
            category: Category::Cert,
            lang: None,
            blurb: "ENCOR and ENARSI: enterprise design, SD-WAN, advanced OSPF and BGP, services, security and automation.",
            modules: vec![
                module!(Advanced, "0 · Architecture", "The exams, campus and data-centre design, QoS, wireless and RF math, SD-Access, SD-WAN, virtualization, tunnels, VXLAN and LISP.", "ccnp";
                    "cp0_01_encor_architecture.md", "cp0_02_qos_wireless_deep.md", "cp0_03_overlays_tunnels.md"),
                module!(Expert, "1 · Routing, Services, Security", "EIGRP, OSPF and BGP in depth, prefix lists, route maps, redistribution, STP, AAA, IPv6, multicast, automation and assurance.", "ccnp";
                    "cp1_01_routing.md", "cp1_02_services_security_automation.md", "cp1_03_eigrp_ospf_deep.md",
                    "cp1_04_route_control_deep.md", "cp1_05_automation_assurance_deep.md", "cp1_06_switching_security_deep.md", "cp1_07_ipv6_multicast.md"),
                build!(Expert, "🧭 BGP Decision Simulator", "Route maps and the full best-path algorithm, in Python.", "ccnp";
                    "cp9_01_bgp_simulator.md"),
                // generated project ideas ccnp
                build!(Expert, "🧩 QoS Policy Planner", "Design queues for a WAN link and generate the MQC configuration.", "ccnp"; "zp_01_qos_planner.md"),
                build!(Expert, "🧩 SD-WAN Path Selection Simulator", "Choose a WAN link per application from latency, loss and jitter.", "ccnp"; "zp_02_sdwan_policy.md"),
            ],
        },
        Track {
            id: "gcp",
            name: "Google Cloud Architect",
            icon: "🌥",
            category: Category::Cert,
            lang: None,
            blurb: "Professional Cloud Architect: resource hierarchy, global VPCs, choosing services, IAM, security and reliable design.",
            modules: vec![
                module!(Advanced, "0 · Foundations and Services", "The exam and hierarchy; compute, VPC networking, storage and databases in depth.", "gcp";
                    "gc0_01_gcp_foundations.md", "gc0_02_compute_deep.md", "gc0_03_networking_deep.md", "gc0_04_storage_databases.md"),
                module!(Expert, "1 · Security and Reliability", "IAM, deny and org policies, audit logs, VPC Service Controls, SLOs, error budgets, DR and cost.", "gcp";
                    "gc1_01_security_reliability.md", "gc1_02_iam_deep.md", "gc1_03_operations_sre.md"),
                module!(Expert, "2 · Design and Case Studies", "Reading requirements, worked designs, data pipelines, migration waves, DR patterns, load balancers and autoscaling.", "gcp";
                    "gc2_01_design_case_studies.md", "gc2_02_data_pipelines.md", "gc2_03_migration_dr.md", "gc2_04_load_balancing_scaling.md"),
                build!(Expert, "🔎 IAM Policy Auditor", "Find public access, basic roles and outsiders in exported IAM policies.", "gcp";
                    "gc9_01_iam_auditor.md"),
                // generated project ideas gcp
                build!(Advanced, "🧩 Architecture Availability Calculator", "Compute the composite SLA of a design with series and parallel parts.", "gcp"; "zp_01_sla_calculator.md"),
                build!(Advanced, "🧩 Deploy a Service to Cloud Run", "Build, deploy and secure a container on Cloud Run, with a budget alert.", "gcp"; "zp_02_cloud_run_service.md"),
            ],
        },
        Track {
            id: "cka",
            name: "Kubernetes Administrator (CKA)",
            icon: "☸",
            category: Category::Cert,
            lang: None,
            blurb: "The hands-on CKA: architecture, kubectl speed, scheduling, networking, storage, upgrades, etcd, RBAC, troubleshooting.",
            modules: vec![
                module!(Advanced, "0 · Architecture and kubectl", "The exam, control plane and nodes, kubectl at speed, the core objects.", "cka";
                    "ck0_01_architecture_kubectl.md"),
                module!(Advanced, "1 · Workloads, Networking, Storage", "Rollouts, QoS and autoscaling, scheduling rules, Services, DNS, Gateway API, PVs and StorageClasses.", "cka";
                    "ck1_01_workloads_networking_storage.md", "ck1_02_workloads_deep.md", "ck1_03_scheduling_deep.md",
                    "ck1_04_services_gateway.md", "ck1_05_storage_deep.md"),
                module!(Expert, "2 · Administration and Troubleshooting", "kubeadm upgrades, etcd backup, RBAC, Helm, and a method for fixing anything.", "cka";
                    "ck2_01_admin_troubleshooting.md", "ck2_02_rbac_security.md", "ck2_03_maintenance_troubleshooting.md"),
                module!(Expert, "3 · Exam Tasks, Hands On", "The task types on the exam, fastest commands, and the rules behind scheduling, Services, RBAC and NetworkPolicy.", "cka";
                    "ck3_01_exam_tasks.md"),
                build!(Expert, "🧹 Kubernetes Manifest Linter", "Catch latest tags, missing limits, privileged pods and broken Services before deploy.", "cka";
                    "ck9_01_manifest_linter.md"),
                // generated project ideas cka
                build!(Expert, "🧩 RBAC Permission Matrix", "Turn Roles and RoleBindings into a who-can-do-what table.", "cka"; "zp_01_rbac_matrix.md"),
                build!(Advanced, "🧩 Three-tier App on a Local Cluster", "Run a web app, an API and a database on kind with everything the CKA asks for.", "cka"; "zp_02_three_tier_kind.md"),
            ],
        },
        Track {
            id: "terraform",
            name: "Terraform Associate",
            icon: "🏗",
            category: Category::Cert,
            lang: None,
            blurb: "Infrastructure as code: HCL, the plan/apply workflow, state, modules, imports and HCP Terraform.",
            modules: vec![
                module!(Intermediate, "0 · IaC and the Workflow", "The exam, HCL, providers, variables, count and for_each, init/plan/apply.", "terraform";
                    "tf0_01_iac_workflow.md", "tf0_02_config_deep.md"),
                module!(Advanced, "1 · State, Modules, HCP Terraform", "Remote state and locking, import and moved blocks, modules, workspaces, policy.", "terraform";
                    "tf1_01_state_modules.md", "tf1_02_state_plan_deep.md", "tf1_03_modules_hcp_deep.md"),
                module!(Advanced, "2 · Testing, CI/CD, Advanced HCL", "dynamic blocks, validation and conditions, terraform test, pipeline gates, drift.", "terraform";
                    "tf2_01_testing_ci.md", "tf2_02_validation_tests.md", "tf2_03_pipelines_drift.md"),
                module!(Advanced, "3 · Hands-on HCL", "Types, count vs for_each, expressions, functions, refactoring with moved/import, exam checklist, secrets and state security, OIDC, provider aliases, the lock file, environments and cost.", "terraform";
                    "tf3_01_hcl_hands_on.md", "tf3_02_secrets_security.md", "tf3_03_providers_multi_env.md"),
                build!(Advanced, "🧾 Terraform Plan Reviewer", "Read plan JSON and block data loss and open ports before apply.", "terraform";
                    "tf9_01_plan_reviewer.md"),
                // generated project ideas terraform
                build!(Intermediate, "🧩 Tag Policy Checker for .tf Files", "Scan Terraform code for resources without required tags.", "terraform"; "zp_01_tag_checker.md"),
                build!(Advanced, "🧩 Reusable Terraform Module", "Write, document and test a module others can use.", "terraform"; "zp_02_static_site_module.md"),
            ],
        },
        Track {
            id: "cissp",
            name: "CISSP",
            icon: "🏛",
            category: Category::Cert,
            lang: None,
            blurb: "Security leadership across all eight domains: risk, assets, architecture, networks, identity, testing, operations, software.",
            modules: vec![
                module!(Advanced, "0 · Governance, Risk and Assets", "The exam, thinking like a manager, risk math, continuity, law, data roles and classification, retention and sanitisation, ethics canons, law, IP and breach notification.", "cissp";
                    "cs0_01_risk_governance.md", "cs0_02_risk_bcp_deep.md", "cs0_03_asset_security.md", "cs0_04_law_ethics_privacy.md"),
                module!(Expert, "1 · Architecture, Crypto, Networks, Identity", "Security models, cryptography, physical and network security, IPsec and zero trust, Kerberos, federation and the identity lifecycle, secure design, cloud responsibility and physical security.", "cissp";
                    "cs1_01_architecture_network_iam.md", "cs1_02_models_crypto_deep.md", "cs1_03_network_iam_deep.md", "cs1_04_secure_design_physical.md"),
                module!(Expert, "2 · Assessment, Operations, Software", "Audits and testing, incidents and forensics, backups, secure development, SOC reports, sampling, metrics, SAST/DAST/SCA and SBOMs.", "cissp";
                    "cs2_01_assessment_operations_software.md", "cs2_02_operations_deep.md", "cs2_03_software_security_deep.md", "cs2_04_assessment_audit_deep.md"),
                build!(Expert, "📊 Quantitative Risk Register", "ALE, safeguard value, treatments and BIA checks, in yen.", "cissp";
                    "cs9_01_risk_register.md"),
                // generated project ideas cissp
                build!(Advanced, "🧩 Incident Response Tabletop Exercise", "Run a ransomware tabletop exercise with timed injects and scoring.", "cissp"; "zp_01_tabletop.md"),
                build!(Expert, "🧩 Joiner-Mover-Leaver Access Review", "Find privilege creep and orphaned accounts by comparing HR data with system access.", "cissp"; "zp_02_access_review.md"),
            ],
        },
        Track {
            id: "pmp",
            name: "PMP (Project Management)",
            icon: "📅",
            category: Category::Cert,
            lang: None,
            blurb: "Lead IT projects: predictive and agile, people and stakeholders, scope, schedule, cost, risk and quality.",
            modules: vec![
                module!(Advanced, "0 · The Exam and People", "PMP mindset, servant leadership, teams, conflict and stakeholders, motivation theories, situational leadership, power and distributed teams.", "pmp";
                    "pm0_01_pmp_people.md", "pm0_02_people_deep.md", "pm0_03_motivation_leadership.md"),
                module!(Advanced, "1 · Process", "Scope and traceability, estimating, critical path and compression, EVM forecasts, risk decisions, contract math, resources, communication, agile and business value, Kanban flow, Little's law and Monte Carlo forecasts.", "pmp";
                    "pm1_01_process.md", "pm1_02_schedule_cost_deep.md", "pm1_03_agile_risk_business.md", "pm1_04_scope_procurement.md", "pm1_05_resources_communication.md", "pm1_06_flow_forecasting.md"),
                module!(Advanced, "2 · Quality, Change and the Exam", "Control charts and Pareto, cost of quality, integrated change control, closing, situational question rules, pacing and a study plan, NPV and IRR, benefits realisation and change models.", "pmp";
                    "pm2_01_quality_change.md", "pm2_02_exam_strategy.md", "pm2_03_benefits_change.md"),
                build!(Advanced, "📅 Office Move: Critical Path and EVM", "The forward/backward pass and earned value formulas, in Python.", "pmp";
                    "pm9_01_schedule_evm.md"),
                // generated project ideas pmp
                build!(Intermediate, "🧩 Sprint Metrics Dashboard", "Velocity, burndown and forecast from a team's sprint data.", "pmp"; "zp_01_sprint_metrics.md"),
                build!(Advanced, "🧩 Plan a Real IT Project", "Write the charter, WBS, risk register and schedule for a project you know.", "pmp"; "zp_02_project_plan.md"),
            ],
        },
        Track {
            id: "azure",
            name: "Microsoft Azure",
            icon: "🔷",
            category: Category::Cert,
            lang: None,
            blurb: "AZ-900 to AZ-104: architecture, core services, Entra ID, governance, cost and hands-on administration.",
            modules: vec![
                module!(Novice, "0 · Concepts and Architecture", "The exams, regions, zones and the resource hierarchy.", "azure";
                    "az0_01_concepts.md"),
                module!(Intermediate, "1 · Services and Governance", "Compute, networking, storage, Entra ID, RBAC inheritance, Policy effects, locks and cost.", "azure";
                    "az1_01_services.md", "az1_02_identity_governance.md", "az1_03_entra_rbac.md", "az1_04_policy_cost.md"),
                module!(Advanced, "2 · Administration (AZ-104)", "Az PowerShell, CLI and Bicep; storage, compute, networking, monitoring and backup in depth, plus App Service, Functions, containers and Key Vault.", "azure";
                    "az2_01_admin.md", "az2_02_storage_deep.md", "az2_03_compute_deep.md", "az2_04_networking_deep.md",
                    "az2_05_monitor_backup.md", "az2_06_apps_keyvault.md"),
                build!(Advanced, "🧾 Governance Audit", "Find untagged spend, open ports and public data with PowerShell, then enforce it with Policy.", "azure";
                    "az9_01_governance_lab.md"),
                // generated project ideas azure
                build!(Advanced, "🧩 VM Rightsizing Advisor", "Recommend smaller (or bigger) VM sizes from CPU and memory metrics.", "azure"; "zp_01_rightsizing.md"),
                build!(Intermediate, "🧩 Azure Naming and Tagging Audit", "Check resources against the Cloud Adoption Framework naming convention.", "azure"; "zp_02_naming_audit.md"),
            ],
        },
        Track {
            id: "cloud",
            name: "Cloud and Containers",
            icon: "📦",
            category: Category::Cert,
            lang: None,
            blurb: "Virtualization, Docker, Kubernetes, cloud-native design, and CloudOps: landing zones, DR, cost, networking and Python automation.",
            modules: vec![
                module!(Novice, "0 · Virtualization and Cloud", "Hypervisors, VMs vs containers, cloud models, migration and FinOps.", "cloud";
                    "cl0_01_virtualization.md", "cl0_02_virtualization_deep.md"),
                module!(Intermediate, "1 · Containers", "Images, Dockerfiles, volumes, networks, Compose and container security.", "cloud";
                    "cl1_01_containers.md", "cl1_02_containers_deep.md"),
                module!(Advanced, "2 · Orchestration and Cloud-Native", "Kubernetes, twelve-factor apps, resilience patterns and serverless.", "cloud";
                    "cl2_01_kubernetes.md", "cl2_02_cloud_native.md"),
                module!(Advanced, "3 · CloudOps", "Landing zones, identity, tagging, monitoring, patching, backup and DR, cost, networking, security and compliance.", "cloud";
                    "cl3_01_cloudops.md", "cl3_02_cloud_networking.md", "cl3_03_cloud_security.md"),
                module!(Advanced, "4 · Cloud Automation with Python", "SDKs, pagination, retries and backoff, safe cleanup scripts, events and cost reports.", "cloud";
                    "cl4_01_cloud_automation.md"),
                build!(Advanced, "📦 Containerize a Status Board", "A Python service, its Docker image, a Compose stack and a Kubernetes deployment.", "cloud";
                    "cl9_01_containerize.md"),
                // generated project ideas cloud
                build!(Advanced, "🧩 Cloud Provider Cost Comparison", "Price the same small architecture on AWS, Azure and Google Cloud.", "cloud"; "zp_01_cloud_compare.md"),
                build!(Intermediate, "🧩 Containerize a Web App", "Package a Python web app with Docker and docker compose, the right way.", "cloud"; "zp_02_containerize.md"),
            ],
        },
        Track {
            id: "devops",
            name: "DevOps and SRE",
            icon: "🚀",
            category: Category::Cert,
            lang: None,
            blurb: "Git, GitHub Actions, Docker, infrastructure as code, Ansible and GitOps, Prometheus and Grafana, DevSecOps, SLOs and incident response.",
            modules: vec![
                module!(Novice, "0 · Git", "Commits, branches, merges, pull requests and team workflows.", "devops";
                    "d0_01_git.md", "d0_02_git_internals.md"),
                module!(Intermediate, "1 · Automate Delivery", "CI/CD pipelines, deployment strategies, feature flags, DORA metrics, semantic releases, Terraform and Ansible.", "devops";
                    "d1_01_cicd.md", "d1_02_iac.md", "d1_03_deployment_strategies.md", "d1_04_release_engineering.md"),
                module!(Advanced, "2 · Run It", "Metrics, logs, traces, SLOs, error budgets, on-call and postmortems.", "devops";
                    "d2_01_observability.md", "d2_02_logs_traces_incidents.md"),
                module!(Intermediate, "3 · Git and GitHub in Practice", "Hands-on branches, conflicts, rebase, squash, revert and the pull request workflow.", "devops";
                    "d3_01_git_practice.md"),
                module!(Advanced, "4 · GitHub Actions", "Workflows, jobs, matrices, secrets, OIDC to the cloud, delivery pipelines and pipeline security.", "devops";
                    "d4_01_github_actions.md"),
                module!(Advanced, "5 · Docker in Depth", "Layers and caching, multi-stage builds, registries, Compose, debugging and container security.", "devops";
                    "d5_01_docker.md"),
                module!(Advanced, "6 · Prometheus and Grafana", "Metrics, exporters, PromQL, alerting, Alertmanager, dashboards and logs with Loki.", "devops";
                    "d6_01_prometheus_grafana.md"),
                module!(Expert, "7 · Configuration and GitOps", "Ansible, Helm, Kustomize, Argo CD and progressive delivery.", "devops";
                    "d7_01_gitops.md"),
                module!(Expert, "8 · DevSecOps and Platforms", "Secrets, pipeline security scans, supply chain, policy as code and platform engineering.", "devops";
                    "d8_01_devsecops.md"),
                build!(Expert, "🚀 Ship a Service End to End", "An instrumented API with its image, CI pipeline, Kubernetes manifests, Prometheus alerts and Grafana dashboard.", "devops";
                    "d9_02_ship_service.md"),
                build!(Advanced, "🚦 SLO Report with CI", "Error budgets from an access log, gated by a GitHub Actions pipeline.", "devops";
                    "d9_01_ci_pipeline.md"),
                // generated project ideas devops
                build!(Advanced, "🧩 CI/CD Pipeline for a Project", "Lint, test, build and release a project automatically with GitHub Actions.", "devops"; "zp_01_ci_pipeline.md"),
                build!(Advanced, "🧩 SLO and Error Budget Tracker", "Turn request logs into SLIs, SLO compliance and burn-rate alerts.", "devops"; "zp_02_slo_budget.md"),
            ],
        },
        Track {
            id: "itil",
            name: "ITIL 4 Foundation",
            icon: "📋",
            category: Category::Cert,
            lang: None,
            blurb: "IT service management: value, the service value system, guiding principles and the daily practices.",
            modules: vec![
                module!(Novice, "0 · Key Concepts", "Services, value, outcomes, utility and warranty, offerings and relationships.", "itil";
                    "it0_01_concepts.md", "it0_02_value_offerings.md"),
                module!(Intermediate, "1 · The Service Value System", "Four dimensions, seven guiding principles, the value chain, continual improvement, the customer journey and experience.", "itil";
                    "it1_01_svs.md", "it1_02_principles_value_streams.md", "it1_03_continual_improvement.md", "it1_04_customer_journey_experience.md"),
                module!(Intermediate, "2 · Practices", "Incident, problem, change, SLM, configuration, events, releases and knowledge, requests and the service desk, assets, money and suppliers.", "itil";
                    "it2_01_practices.md", "it2_02_incident_problem_change.md", "it2_03_service_levels_measurement.md", "it2_04_config_events_knowledge.md", "it2_05_requests_service_desk.md", "it2_06_assets_finance_suppliers.md"),
                build!(Intermediate, "⏱ Service Desk Kit", "An SLA clock with Japanese business hours and holidays, plus the process documents.", "itil";
                    "it9_01_service_desk_kit.md"),
                // generated project ideas itil
                build!(Advanced, "🧩 IT Service Catalog Site", "Publish a service catalog with SLAs, owners and request forms.", "itil"; "zp_01_service_catalog.md"),
                build!(Intermediate, "🧩 Change Calendar Conflict Checker", "Spot risky changes: overlaps, freeze periods and missing approvals.", "itil"; "zp_02_change_calendar.md"),
            ],
        },
        Track {
            id: "ipa-fe",
            name: "基本情報技術者 (IPA FE)",
            icon: "🎌",
            category: Category::Cert,
            lang: None,
            blurb: "Japan's national IT exam: theory, pseudo-code algorithms, systems, security, management and law.",
            modules: vec![
                module!(Novice, "0 · The Exam", "科目A and 科目B, the IPA exam ladder, how to study.", "ipa-fe";
                    "fe0_01_about.md"),
                module!(Intermediate, "1 · 基礎理論 and 疑似言語", "Numbers, two's complement, shifts, floating point, errors, logic, and the exam's pseudo-code.", "ipa-fe";
                    "fe1_01_numbers.md", "fe1_02_pseudocode.md", "fe1_03_numbers_deep.md"),
                module!(Intermediate, "2 · Algorithms", "Data structures, heaps and hash tables in 1-based arrays, search, sort, recursion and complexity, stacks and reverse Polish notation, BFS and Dijkstra.", "ipa-fe";
                    "fe2_01_algorithms.md", "fe2_02_structures_hands_on.md", "fe2_03_graphs_rpn.md"),
                module!(Advanced, "3 · Technology, Management, Strategy", "Availability, CPU/disk/network calculations, normalisation, subnetting, security, projects, arrow diagrams, break-even, inventory, decisions and law, salted hashes, signatures and log analysis.", "ipa-fe";
                    "fe3_01_technology.md", "fe3_02_system_calculations.md", "fe3_03_db_network_hands_on.md", "fe3_04_security_hands_on.md", "fe4_01_management.md", "fe4_02_strategy_calculations.md"),
                build!(Advanced, "🧮 科目B Drills", "Ten classic exam algorithms, plus a random tracing quiz.", "ipa-fe";
                    "fe9_01_drills.md"),
                // generated project ideas ipa-fe
                build!(Novice, "🧩 Number Systems and Logic Trainer", "Drill binary, hex, two's complement and logic gates like the exam.", "ipa-fe"; "zp_01_number_systems.md"),
                build!(Intermediate, "🧩 Algorithm Trace Trainer", "Practise the FE exam's trace tables by having a program check yours.", "ipa-fe"; "zp_02_algorithm_tracer.md"),
            ],
        },
        Track {
            id: "japanese",
            name: "Japanese for IT Work",
            icon: "🗾",
            category: Category::Career,
            lang: None,
            blurb: "Kana, polite basics, keigo, and the Japanese of tickets, emails, chat, phone and the help desk.",
            modules: vec![
                module!(Novice, "0 · Kana", "Hiragana, katakana, sounds, romaji, small kana and long vowels, kana as data, katakana IT words and the JLPT.", "japanese";
                    "jp0_01_kana.md", "jp0_02_kana_systems.md"),
                module!(Novice, "1 · Polite Basics", "Particles in depth, です/ます, self-introduction, verb forms, numbers, money, dates and times.", "japanese";
                    "jp1_01_basics.md", "jp1_02_verbs.md", "jp1_03_numbers_dates.md", "jp1_04_particles.md"),
                module!(Intermediate, "2 · 敬語 and Workplace Culture", "Respectful and humble language, special keigo verbs, uchi and soto, common mistakes, daily phrases, 報連相, meetings, 議事録 and phone calls.", "japanese";
                    "jp2_01_keigo.md", "jp2_02_keigo_forms.md", "jp2_03_meetings_phone.md"),
                module!(Intermediate, "3 · IT Communication", "Business email, 障害報告 structure and phrases, status updates, chat, phone and help desk Japanese, IT kanji and a study system.", "japanese";
                    "jp3_01_it_communication.md", "jp3_02_incident_reports.md", "jp3_03_it_kanji.md"),
                build!(Intermediate, "🈁 Japanese Trainer", "A spaced-repetition trainer with a kana-to-romaji engine, plus an interview kit.", "japanese";
                    "jp9_01_trainer.md"),
                // generated project ideas japanese
                build!(Intermediate, "🧩 Business Email Template Builder", "Assemble polite Japanese work emails from phrase blocks.", "japanese"; "zp_01_keigo_mail.md"),
                build!(Novice, "🧩 Kana and IT Vocabulary Quiz", "A terminal quiz with spaced repetition for kana and IT words.", "japanese"; "zp_02_kana_quiz.md"),
            ],
        },
        Track {
            id: "japan-career",
            name: "IT Career in Japan",
            icon: "🗼",
            category: Category::Career,
            lang: None,
            blurb: "The job market, visas, documents, job hunting, reading offers, and the admin of living in Japan.",
            modules: vec![
                module!(Novice, "0 · The Market", "SIer, SES, in-house, 社内SE and 外資系; roles and salaries; reading listings and comparing cities.", "japan-career";
                    "jc0_01_market.md", "jc0_02_listings_cities.md"),
                module!(Intermediate, "1 · Visas", "技術・人文知識・国際業務, Highly Skilled Professional points and planning, staying legal, job changes, renewals, re-entry and permanent residence.", "japan-career";
                    "jc1_01_visa.md", "jc1_02_hsp_points.md", "jc1_03_visa_changes_pr.md"),
                module!(Intermediate, "2 · Job Hunting", "Sites, recruiters, 履歴書 and 和暦, 職務経歴書, interviews, offers, 固定残業代 and real hourly pay, the selection flow, SPI, common questions and 候補日.", "japan-career";
                    "jc2_01_job_hunting.md", "jc2_02_offers_pay.md", "jc2_03_resume_dates.md", "jc2_04_interviews_tests.md"),
                module!(Intermediate, "3 · Life Admin", "Registration, insurance, pension, tax, take-home pay and resident tax timing, housing, paid leave balances, resigning and timing your last day, listings, 初期費用, leases and 原状回復.", "japan-career";
                    "jc3_01_life_admin.md", "jc3_02_take_home_pay.md", "jc3_03_leave_resigning.md", "jc3_04_housing_contracts.md"),
                build!(Intermediate, "🎒 Application Kit", "Résumé templates, an application tracker and an honest offer checker.", "japan-career";
                    "jc9_01_application_kit.md"),
                // generated project ideas japan-career
                build!(Intermediate, "🧩 Bilingual Résumé (履歴書 and 職務経歴書)", "Write and print a Japanese-format résumé and career history from one data file.", "japan-career"; "zp_01_resume_site.md"),
                build!(Intermediate, "🧩 Japan Job Application Tracker", "Track applications, interviews and offers, and compare take-home pay.", "japan-career"; "zp_02_job_tracker.md"),
            ],
        },
    ]
}
