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
                "n0_01_welcome.md", "n0_02_errors.md"),
            module!(Novice, "1 · Fundamentals", "Variables, math, text, decisions, loops and functions.", "python";
                "n1_01_variables.md", "n1_02_numbers.md", "n1_03_strings.md", "n1_04_input.md", "n1_05_conditions.md",
                "n1_06_loops.md", "n1_07_functions.md"),
            module!(Novice, "2 · Collections", "Lists, dictionaries, tuples and sets, and a first project.", "python";
                "n2_01_lists.md", "n2_02_dicts.md", "n2_03_tuples_sets.md", "n2_04_project_inventory.md"),
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
                "p1_01_variables.md", "p1_02_strings.md", "p1_03_conditions.md", "p1_04_loops.md", "p1_05_functions.md"),
            module!(Intermediate, "2 · Collections and Objects", "Arrays, hashtables, object pipelines and regex.", "powershell";
                "p2_01_collections.md", "p2_02_objects.md", "p2_03_text.md"),
            module!(Intermediate, "3 · Files and Windows", "Files, CSV, JSON, errors, and managing a Windows machine.", "powershell";
                "p3_01_files.md", "p3_02_errors.md", "p3_03_windows_admin.md"),
            module!(Advanced, "4 · Admin Automation", "Advanced functions, modules, scheduled tasks, remoting and AD.", "powershell";
                "p4_01_advanced_functions.md", "p4_02_scripts_modules.md", "p4_03_remoting_ad.md"),
            module!(Expert, "5 · Professional PowerShell", "Classes, .NET, testing, style and security.", "powershell";
                "p5_01_classes.md", "p5_02_professional.md"),
            module!(Expert, "6 · Building Tools and Apps", "Tool design, REST APIs, secrets, logging, desktop GUIs, CI and publishing.", "powershell";
                "p6_01_tool_design.md", "p6_02_apis.md", "p6_03_gui_shipping.md"),
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
                "l1_01_variables.md", "l1_02_conditions.md", "l1_03_loops.md", "l1_04_functions.md"),
            module!(Intermediate, "2 · Text Power Tools", "Pipes, redirection, grep, sed, awk, find and archives.", "bash";
                "l2_01_pipes.md", "l2_02_grep_sed_awk.md", "l2_03_find.md"),
            module!(Intermediate, "3 · Linux Administration", "Permissions, processes, systemd, logs and networking.", "bash";
                "l3_01_permissions.md", "l3_02_processes.md", "l3_03_networking.md"),
            module!(Advanced, "4 · Automation and Servers", "Robust scripts, scheduling, packages, disks and hardening.", "bash";
                "l4_01_robust_scripts.md", "l4_02_cron.md", "l4_03_packages_storage.md"),
            module!(Expert, "5 · Containers and Expert Bash", "Docker, associative arrays, parallel jobs and Git.", "bash";
                "l5_01_containers.md", "l5_02_professional.md"),
            build!(Intermediate, "📊 Access Log Analyzer", "Who, what and when on a web server: a report from nginx logs.", "bash";
                "m1_01_log_analyzer.md"),
            build!(Advanced, "🛡 Server Security Audit", "Check SSH, accounts and sudo rules, and score a Linux server.", "bash";
                "m2_01_server_audit.md"),
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
                "j0_01_welcome.md"),
            module!(Novice, "1 · Fundamentals", "Values, comparisons, decisions, loops and functions.", "javascript";
                "j1_01_values.md", "j1_02_control.md", "j1_03_functions.md"),
            module!(Intermediate, "2 · Working with Data", "Arrays, objects, JSON and regular expressions.", "javascript";
                "j2_01_arrays.md", "j2_02_objects.md", "j2_03_regex.md"),
            module!(Intermediate, "3 · Real-World JavaScript", "Errors, classes, async code, files and npm.", "javascript";
                "j3_01_errors_classes.md", "j3_02_async.md", "j3_03_node.md"),
            module!(Advanced, "4 · The Web", "HTML, the DOM, HTTP and REST APIs.", "javascript";
                "j4_01_web.md", "j4_02_http.md"),
            module!(Expert, "5 · Professional JavaScript", "Testing, TypeScript and security.", "javascript";
                "j5_01_professional.md"),
            module!(Expert, "6 · Full-Stack Applications", "Your own web server, front-end state and rendering, auth, config and deployment.", "javascript";
                "j6_01_server.md", "j6_02_frontend.md", "j6_03_production.md"),
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
                "t0_01_welcome.md"),
            module!(Intermediate, "1 · The Type System", "Arrays, unions, literal types and object types.", "typescript";
                "t1_01_types.md"),
            module!(Intermediate, "2 · Functions and Narrowing", "Generics, discriminated unions and type guards.", "typescript";
                "t2_01_functions_generics.md", "t2_02_narrowing.md"),
            module!(Advanced, "3 · Real-World TypeScript", "Classes, utility types, async code, APIs and modules.", "typescript";
                "t3_01_classes_utility.md", "t3_02_async_modules.md"),
            module!(Expert, "4 · Professional TypeScript", "tsconfig, testing, declarations, migration and runtime validation.", "typescript";
                "t4_01_professional.md"),
            module!(Expert, "5 · Application Architecture", "Domain modeling with types, ports and adapters, events, and full-stack TypeScript.", "typescript";
                "t5_01_domain.md", "t5_02_architecture.md", "t5_03_fullstack.md"),
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
            module!(Novice, "1 · HTML Content", "Links, images, lists, tables and forms.", "html";
                "h1_01_content.md", "h1_02_forms.md"),
            module!(Intermediate, "2 · CSS", "Selectors, the box model, flexbox and grid.", "html";
                "h2_01_css_basics.md", "h2_02_layout.md"),
            module!(Advanced, "3 · Modern Web Pages", "Responsive design, dark mode, semantics and accessibility.", "html";
                "h3_01_responsive.md", "h3_02_accessibility.md"),
            module!(Expert, "4 · Professional Front End", "Web security (XSS, CSP), performance and publishing.", "html";
                "h4_01_professional.md"),
            build!(Intermediate, "🪪 Portfolio and Résumé Site", "Your own responsive site with dark mode and a print-ready résumé.", "html";
                "i1_01_portfolio.md"),
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
                "g0_01_welcome.md"),
            module!(Novice, "1 · Fundamentals", "Types, strings, control flow, functions and errors.", "go";
                "g1_01_basics.md", "g1_02_functions_errors.md"),
            module!(Intermediate, "2 · Data", "Slices, maps, structs, methods and interfaces.", "go";
                "g2_01_slices_maps.md", "g2_02_structs_interfaces.md"),
            module!(Advanced, "3 · Concurrency and Networks", "Goroutines, channels, worker pools, HTTP and JSON.", "go";
                "g3_01_concurrency.md", "g3_02_http_json.md"),
            module!(Expert, "4 · Professional Go", "Modules, table-driven tests, generics and shipping binaries.", "go";
                "g4_01_professional.md"),
            build!(Advanced, "🩺 Service Health Prober", "Check every service at once: TCP and HTTP, a worker pool, timeouts, percentiles.", "go";
                "h1_01_health_prober.md"),
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
                "c0_01_welcome.md"),
            module!(Novice, "1 · Fundamentals", "Types, text, decisions, loops and methods.", "csharp";
                "c1_01_types.md", "c1_02_control.md", "c1_03_methods.md"),
            module!(Intermediate, "2 · Data and Objects", "Lists, dictionaries, LINQ, classes, records and interfaces.", "csharp";
                "c2_01_collections.md", "c2_02_classes.md"),
            module!(Advanced, "3 · Real-World C#", "Exceptions, files, JSON, async code and HTTP.", "csharp";
                "c3_01_errors_files.md", "c3_02_async.md"),
            module!(Expert, "4 · Professional .NET", "Projects, NuGet, testing, dependency injection and Web APIs.", "csharp";
                "c4_01_professional.md"),
            module!(Expert, "5 · Application Development", "Clean architecture, EF Core and ASP.NET Core, MVVM desktop apps and shipping.", "csharp";
                "c5_01_architecture.md", "c5_02_data_web.md", "c5_03_desktop.md"),
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
                "x0_01_welcome.md"),
            module!(Novice, "1 · Fundamentals", "Types, bits, strings, input, decisions, loops and functions.", "cpp";
                "x1_01_types.md", "x1_02_control_functions.md"),
            module!(Intermediate, "2 · The Standard Library", "Vectors, maps, algorithms, lambdas, classes and RAII.", "cpp";
                "x2_01_containers.md", "x2_02_classes.md"),
            module!(Advanced, "3 · Memory and Errors", "Pointers, smart pointers, move semantics, exceptions and files.", "cpp";
                "x3_01_memory.md", "x3_02_errors_files.md"),
            module!(Expert, "4 · Expert C++", "Templates, concepts, builds, threads and writing safe C++.", "cpp";
                "x4_01_templates.md", "x4_02_professional.md"),
            module!(Expert, "5 · Systems Applications", "Designing, measuring and shipping real C++ programs: interfaces, performance, undo, GUIs and networking.", "cpp";
                "x5_01_design.md", "x5_02_performance.md", "x5_03_apps.md"),
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
            module!(Novice, "1 · Asking Questions", "Filtering, sorting, calculating and grouping.", "sql";
                "s1_01_where.md", "s1_02_sorting.md", "s1_03_aggregates.md"),
            module!(Intermediate, "2 · Relationships and Changes", "Joins, changing data, transactions and table design.", "sql";
                "s2_01_joins.md", "s2_02_modify.md", "s2_03_create.md"),
            module!(Advanced, "3 · Powerful Queries", "Subqueries, CTEs, text, dates and CASE.", "sql";
                "s3_01_subqueries.md", "s3_02_functions.md"),
            module!(Advanced, "4 · Analytics", "Window functions: rankings, running totals, top N per group.", "sql";
                "s4_01_window.md"),
            module!(Expert, "5 · Professional Databases", "SQL injection, permissions, normalization and DBA basics.", "sql";
                "s5_01_professional.md"),
            build!(Intermediate, "🗃 IT Asset Inventory", "A real inventory database: schema, reports and a safe Python CLI.", "sql";
                "t1_01_asset_inventory.md"),
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
                module!(Novice, "0 · Computers", "Hardware, binary, operating systems and booting.", "foundations";
                    "f0_01_hardware.md", "f0_02_os.md"),
                module!(Novice, "1 · Networks", "IP addresses, DNS, DHCP, ports and the OSI model.", "foundations";
                    "f1_01_networks.md"),
                module!(Novice, "2 · Working in IT", "Troubleshooting, tickets, security, virtualization and cloud.", "foundations";
                    "f2_01_troubleshooting.md", "f2_02_security_cloud.md"),
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
                module!(Novice, "1 · Core 1 (220-1201)", "Hardware, mobile devices, networking, printers, virtualization.", "aplus";
                    "a1_01_hardware.md", "a1_02_networking.md", "a1_03_printers_troubleshooting.md", "a1_04_virtualization_cloud.md"),
                module!(Intermediate, "2 · Core 2 (220-1202)", "Windows, other OSs, security and operational procedures.", "aplus";
                    "a2_01_windows.md", "a2_02_other_os.md", "a2_03_security.md", "a2_04_operations.md"),
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
                module!(Intermediate, "1 · Concepts", "OSI, ports and protocols, subnetting and IPv6.", "networkplus";
                    "n1_01_osi_protocols.md", "n1_02_subnetting.md"),
                module!(Intermediate, "2 · Implementation", "Switching, VLANs, routing, wireless, WAN and cloud networks.", "networkplus";
                    "n2_01_switching_routing.md", "n2_02_wireless_wan_cloud.md"),
                module!(Advanced, "3 · Operations, Security, Troubleshooting", "Services, monitoring, attacks, defences and tools.", "networkplus";
                    "n3_01_services_security.md", "n3_02_troubleshooting.md"),
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
                module!(Intermediate, "1 · Storage, Users and Services", "Partitions, LVM, mounts, accounts, packages, systemd and logs.", "linux";
                    "lx1_01_storage.md", "lx1_02_users_services.md"),
                module!(Advanced, "2 · Security and Automation", "Permissions, SELinux, firewalls, SSH, networking, containers, Ansible.", "linux";
                    "lx2_01_security.md", "lx2_02_networking_automation.md"),
                module!(Advanced, "3 · Troubleshooting", "Performance, disks, services, networking and boot problems.", "linux";
                    "lx3_01_troubleshooting.md"),
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
                module!(Intermediate, "1 · General Security Concepts", "Controls, zero trust, cryptography and PKI.", "securityplus";
                    "sp0_01_concepts.md", "sp1_01_crypto.md"),
                module!(Intermediate, "2 · Threats and Vulnerabilities", "Threat actors, attacks, malware and indicators.", "securityplus";
                    "sp2_01_threats.md"),
                module!(Advanced, "3 · Architecture and Operations", "Secure design, data protection, IAM, monitoring and incident response.", "securityplus";
                    "sp3_01_architecture.md", "sp4_01_operations.md"),
                module!(Advanced, "4 · Program Management", "Governance, risk, third parties, compliance and audits.", "securityplus";
                    "sp5_01_governance.md"),
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
                module!(Intermediate, "0 · The Exam and IOS", "CCNA 200-301, the IOS command line and basic device setup.", "ccna";
                    "c0_01_ios.md"),
                module!(Intermediate, "1 · Switching and Routing", "VLANs, trunks, STP, EtherChannel, static routes and OSPF.", "ccna";
                    "c1_01_switching.md", "c1_02_routing.md"),
                module!(Advanced, "2 · Services, Security, Automation", "DHCP, NAT, ACLs, port security, wireless, SDN and APIs.", "ccna";
                    "c2_01_services_security.md", "c2_02_wireless_automation.md"),
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
                module!(Intermediate, "1 · Core Services", "EC2, S3, EBS, VPC, Route 53, RDS, DynamoDB and more.", "aws";
                    "aw1_01_compute_storage.md", "aw1_02_network_databases.md"),
                module!(Advanced, "2 · Security, Billing, Architecture", "IAM, security services, pricing, support and resilient designs.", "aws";
                    "aw2_01_security_billing.md", "aw2_02_architecture.md"),
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
                module!(Advanced, "0 · Architecture", "The exams, campus and data-centre design, wireless, SD-Access, SD-WAN, virtualization.", "ccnp";
                    "cp0_01_encor_architecture.md"),
                module!(Expert, "1 · Routing, Services, Security", "EIGRP, OSPF and BGP in depth, route maps, STP, QoS, AAA, NETCONF/RESTCONF.", "ccnp";
                    "cp1_01_routing.md", "cp1_02_services_security_automation.md"),
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
                module!(Advanced, "0 · Foundations and Services", "The exam, hierarchy, networking, compute, storage, databases, data.", "gcp";
                    "gc0_01_gcp_foundations.md"),
                module!(Expert, "1 · Security and Reliability", "IAM, keys, VPC Service Controls, load balancing, SLOs, DR, migration, cost.", "gcp";
                    "gc1_01_security_reliability.md"),
                module!(Expert, "2 · Design and Case Studies", "Reading requirements, mapping them to services, worked designs.", "gcp";
                    "gc2_01_design_case_studies.md"),
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
                module!(Advanced, "1 · Workloads, Networking, Storage", "Resources and probes, scheduling, Services, NetworkPolicy, volumes.", "cka";
                    "ck1_01_workloads_networking_storage.md"),
                module!(Expert, "2 · Administration and Troubleshooting", "kubeadm upgrades, etcd backup, RBAC, Helm, and a method for fixing anything.", "cka";
                    "ck2_01_admin_troubleshooting.md"),
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
                    "tf0_01_iac_workflow.md"),
                module!(Advanced, "1 · State, Modules, HCP Terraform", "Remote state and locking, import and moved blocks, modules, workspaces, policy.", "terraform";
                    "tf1_01_state_modules.md"),
                module!(Advanced, "2 · Testing, CI/CD, Advanced HCL", "dynamic blocks, terraform test, pipelines with OIDC, drift, structure.", "terraform";
                    "tf2_01_testing_ci.md"),
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
                module!(Advanced, "0 · Governance, Risk and Assets", "The exam, thinking like a manager, risk math, continuity, law, data handling.", "cissp";
                    "cs0_01_risk_governance.md"),
                module!(Expert, "1 · Architecture, Crypto, Networks, Identity", "Security models, cryptography, physical and network security, IAM.", "cissp";
                    "cs1_01_architecture_network_iam.md"),
                module!(Expert, "2 · Assessment, Operations, Software", "Audits and testing, incidents and forensics, backups, secure development.", "cissp";
                    "cs2_01_assessment_operations_software.md"),
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
                module!(Advanced, "0 · The Exam and People", "PMP mindset, servant leadership, teams, conflict and stakeholders.", "pmp";
                    "pm0_01_pmp_people.md"),
                module!(Advanced, "1 · Process", "Scope, critical path, earned value, risk, quality, change and agile frameworks.", "pmp";
                    "pm1_01_process.md"),
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
                module!(Intermediate, "1 · Services and Governance", "Compute, networking, storage, Entra ID, RBAC, Policy and cost.", "azure";
                    "az1_01_services.md", "az1_02_identity_governance.md"),
                module!(Advanced, "2 · Administration (AZ-104)", "Az PowerShell, CLI, Bicep, networking, backup and monitoring.", "azure";
                    "az2_01_admin.md"),
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
            blurb: "Virtualization, cloud models, Docker, Kubernetes and cloud-native design, for any provider.",
            modules: vec![
                module!(Novice, "0 · Virtualization and Cloud", "Hypervisors, VMs vs containers, cloud models, migration and FinOps.", "cloud";
                    "cl0_01_virtualization.md"),
                module!(Intermediate, "1 · Containers", "Images, Dockerfiles, volumes, networks, Compose and container security.", "cloud";
                    "cl1_01_containers.md"),
                module!(Advanced, "2 · Orchestration and Cloud-Native", "Kubernetes, twelve-factor apps, resilience patterns and serverless.", "cloud";
                    "cl2_01_kubernetes.md", "cl2_02_cloud_native.md"),
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
            blurb: "Git, CI/CD, infrastructure as code, observability, SLOs and incident response.",
            modules: vec![
                module!(Novice, "0 · Git", "Commits, branches, merges, pull requests and team workflows.", "devops";
                    "d0_01_git.md"),
                module!(Intermediate, "1 · Automate Delivery", "CI/CD pipelines, deployment strategies, Terraform and Ansible.", "devops";
                    "d1_01_cicd.md", "d1_02_iac.md"),
                module!(Advanced, "2 · Run It", "Metrics, logs, traces, SLOs, error budgets, on-call and postmortems.", "devops";
                    "d2_01_observability.md"),
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
                module!(Novice, "0 · Key Concepts", "Services, value, outcomes, utility and warranty.", "itil";
                    "it0_01_concepts.md"),
                module!(Intermediate, "1 · The Service Value System", "Four dimensions, seven guiding principles, the value chain.", "itil";
                    "it1_01_svs.md"),
                module!(Intermediate, "2 · Practices", "Incident, problem, request, change, service desk, SLM and more.", "itil";
                    "it2_01_practices.md"),
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
                module!(Intermediate, "1 · 基礎理論 and 疑似言語", "Numbers, bits, logic, and reading the exam's pseudo-code.", "ipa-fe";
                    "fe1_01_numbers.md", "fe1_02_pseudocode.md"),
                module!(Intermediate, "2 · Algorithms", "Data structures, search, sort, recursion and complexity.", "ipa-fe";
                    "fe2_01_algorithms.md"),
                module!(Advanced, "3 · Technology, Management, Strategy", "Availability, DB, networks, security, projects, business and law.", "ipa-fe";
                    "fe3_01_technology.md", "fe4_01_management.md"),
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
                module!(Novice, "0 · Kana", "Hiragana, katakana, sounds, katakana IT words and the JLPT.", "japanese";
                    "jp0_01_kana.md"),
                module!(Novice, "1 · Polite Basics", "Particles, です/ます, self-introduction, numbers and money.", "japanese";
                    "jp1_01_basics.md"),
                module!(Intermediate, "2 · 敬語 and Workplace Culture", "Respectful and humble language, daily phrases, 報連相.", "japanese";
                    "jp2_01_keigo.md"),
                module!(Intermediate, "3 · IT Communication", "Business email, incident reports, chat, phone and help desk Japanese.", "japanese";
                    "jp3_01_it_communication.md"),
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
                module!(Novice, "0 · The Market", "SIer, SES, in-house, 社内SE and 外資系; roles and salaries.", "japan-career";
                    "jc0_01_market.md"),
                module!(Intermediate, "1 · Visas", "技術・人文知識・国際業務, Highly Skilled Professional, and staying legal.", "japan-career";
                    "jc1_01_visa.md"),
                module!(Intermediate, "2 · Job Hunting", "Sites, recruiters, 履歴書, 職務経歴書, interviews and reading offers.", "japan-career";
                    "jc2_01_job_hunting.md"),
                module!(Intermediate, "3 · Life Admin", "Registration, insurance, pension, tax, housing, leave and resigning.", "japan-career";
                    "jc3_01_life_admin.md"),
                build!(Intermediate, "🎒 Application Kit", "Résumé templates, an application tracker and an honest offer checker.", "japan-career";
                    "jc9_01_application_kit.md"),
                // generated project ideas japan-career
                build!(Intermediate, "🧩 Bilingual Résumé (履歴書 and 職務経歴書)", "Write and print a Japanese-format résumé and career history from one data file.", "japan-career"; "zp_01_resume_site.md"),
                build!(Intermediate, "🧩 Japan Job Application Tracker", "Track applications, interviews and offers, and compare take-home pay.", "japan-career"; "zp_02_job_tracker.md"),
            ],
        },
    ]
}
