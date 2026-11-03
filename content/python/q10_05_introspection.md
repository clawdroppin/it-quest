@@lesson q10-introspection Introspection: Code That Examines Code
@@concepts objects, functions, modules
@@text
Python can look at its own objects while running: list an object's attributes, read a function's parameters, load a module by name, or call a method chosen from a string. Test frameworks, CLI libraries, web frameworks and plugin systems are built on this.

## Asking objects about themselves

```python
import inspect

class Backup:
    """Copies files to the backup share."""
    retention_days = 30
    def run(self, path: str, dry_run: bool = False) -> int:
        """Run a backup and return the number of files."""
        return 0

b = Backup()
print(type(b).__name__, isinstance(b, Backup))
print(hasattr(b, "run"), getattr(b, "retention_days"), getattr(b, "missing", "default"))
print(vars(Backup).keys() - {"__dict__", "__weakref__", "__module__", "__qualname__", "__firstlineno__", "__static_attributes__"})
print(inspect.getdoc(Backup.run))
print(inspect.signature(Backup.run))
print(callable(b.run), inspect.ismethod(b.run))
```

| Function | Answers |
|---|---|
| `type(x)`, `isinstance(x, T)` | what is it? |
| `dir(x)`, `vars(x)` | which attributes / its `__dict__` |
| `getattr(x, name, default)`, `setattr`, `hasattr`, `delattr` | attribute access by **string** name |
| `callable(x)` | can it be called? |
| `inspect.signature(f)` | parameters, defaults, annotations |
| `inspect.getsource(f)`, `getdoc`, `getfile` | source code, docstring, file |
| `typing.get_type_hints(f)` | resolved type annotations |

## Dispatching by name

`getattr` turns strings (from a command line, config file or chat message) into method calls, without a long `if/elif` chain:

```python
class Tasks:
    def cmd_backup(self):
        return "backing up"
    def cmd_cleanup(self):
        return "cleaning"

def dispatch(obj, name):
    handler = getattr(obj, f"cmd_{name}", None)
    if handler is None:
        commands = sorted(n[4:] for n in dir(obj) if n.startswith("cmd_"))
        return f"unknown command; try: {', '.join(commands)}"
    return handler()

print(dispatch(Tasks(), "backup"))
print(dispatch(Tasks(), "reboot"))
```

The `cmd_` prefix matters: never let user input pick **any** attribute (`__class__`, `__init__`...). Restrict names to an explicit prefix or allow-list.

## Signatures: binding arguments

`inspect.signature(f).bind(*args, **kwargs)` checks arguments against a function **without calling it**, and tells you which value goes to which parameter. Libraries like Typer and FastAPI use signatures and type hints to turn functions directly into CLI commands or HTTP endpoints:

```python
import inspect

def create_user(name: str, quota: int = 10, admin: bool = False): ...

sig = inspect.signature(create_user)
for p in sig.parameters.values():
    print(p.name, p.annotation.__name__, "required" if p.default is p.empty else f"default={p.default}")

bound = sig.bind("maya", admin=True)
bound.apply_defaults()
print(bound.arguments)
```

## Importing by name: plugins

```python
import importlib

mod = importlib.import_module("json")          # same as `import json`, but the name is a string
print(mod.dumps({"ok": True}))

func = getattr(importlib.import_module("os.path"), "basename")
print(func("/var/log/syslog"))
```

A plugin system can read `"mycompany.checks.disk:DiskCheck"` from a config file, import the module part and `getattr` the class. Packages can also advertise plugins through **entry points** (`importlib.metadata.entry_points(group="myapp.plugins")`), which is how pytest finds its plugins.

## The call stack and frames

`inspect.stack()` and `sys._getframe()` reveal who called you (logging libraries use this to print file and line numbers). `traceback.format_exc()` turns the current exception into text for logs.

## Introspection and dataclasses

```python
from dataclasses import dataclass, fields, asdict

@dataclass
class Server:
    name: str
    port: int = 22

print([(f.name, f.type.__name__, f.default) for f in fields(Server)])
print(asdict(Server("web01")))
```

## Use it sparingly

Introspection is powerful, but code that builds attribute names from strings is harder to read, search and type-check. Use it at the **edges** (CLIs, plugins, serialisation, frameworks) and keep business logic plain.

@@quiz
What does `getattr(obj, "run", None)` return if obj has no `run`?
- Raises AttributeError
* None
- An empty function
- False
? The third argument is the default.

@@quiz
Why use a prefix like `cmd_` when dispatching user input with getattr?
* So users can't call arbitrary attributes like `__init__` or internal methods
- getattr requires it
- For speed
- To support uppercase
? Always restrict reflective calls to an allow-list.

@@quiz
What does `inspect.signature(f).bind(...)` do?
- Calls f
* Matches arguments to parameters and raises TypeError if they don't fit, without calling f
- Changes the function's defaults
- Compiles f
? Frameworks use it to validate calls.

@@quiz
How do you import a module whose name is in a string?
- `import name`
- `eval("import " + name)`
* `importlib.import_module(name)`
- `__import__` is forbidden
? Avoid eval; importlib is the proper API.

@@flashcards
Introspection :: examining objects at runtime
`getattr(obj, name, default)` :: attribute by string name
`vars(obj)` :: an object's `__dict__`
`inspect.signature(f)` :: parameters, defaults and annotations of a callable
`Signature.bind` :: map arguments to parameters without calling
`importlib.import_module` :: import a module by name
Entry points :: package metadata advertising plugins
`dataclasses.fields` :: list a dataclass's fields

@@exercise A command dispatcher
Write `class Shell` with methods `do_ping(host)`, `do_add(a, b)` (adds two ints, converting from strings) and `do_help()`, and a method `run(line)`:

- split the line; find `do_<word>` with `getattr`; unknown → `"unknown command: <word>"`
- before calling, check the arguments with `inspect.signature(method).bind(*args)`; a `TypeError` → `"usage: <word> <param names separated by spaces>"`
- `do_help()` returns `"commands: "` plus the sorted command names (without `do_`) joined by `", "`, found with `dir`
@@starter
import inspect


class Shell:
    pass


sh = Shell()
for line in ["ping web01", "add 2 3", "add 2", "help", "reboot now"]:
    print(sh.run(line))
@@expected
pong from web01
5
usage: add a b
commands: add, help, ping
unknown command: reboot
@@tests
sh = Shell()
check(sh.run("ping"), "usage: ping host", "missing arg")
check(sh.run("help extra"), "usage: help", "too many args")
check(sh.run("__init__"), "unknown command: __init__", "no dunder access")
@@requires
inspect.signature
@@hint
`word, *args = line.split()`; `method = getattr(self, f"do_{word}", None)`. Then `try: inspect.signature(method).bind(*args) except TypeError: params = " ".join(inspect.signature(method).parameters); return f"usage: {word} {params}".strip()`. For a bound method the signature doesn't include `self`.
@@solution
import inspect


class Shell:
    def do_ping(self, host):
        return f"pong from {host}"

    def do_add(self, a, b):
        return int(a) + int(b)

    def do_help(self):
        names = sorted(n[3:] for n in dir(self) if n.startswith("do_"))
        return "commands: " + ", ".join(names)

    def run(self, line):
        word, *args = line.split()
        method = getattr(self, f"do_{word}", None)
        if method is None:
            return f"unknown command: {word}"
        sig = inspect.signature(method)
        try:
            sig.bind(*args)
        except TypeError:
            return f"usage: {word} {' '.join(sig.parameters)}".strip()
        return method(*args)


sh = Shell()
for line in ["ping web01", "add 2 3", "add 2", "help", "reboot now"]:
    print(sh.run(line))

@@exercise Turn a function into a CLI
Write `call_from_args(func, argv)` that calls `func` using command-line style arguments, like Typer does:

- positional strings fill required parameters in order
- `--name value` sets a parameter by name (dashes become underscores)
- `--flag` with no value sets a `bool` parameter to True
- each value is converted with the parameter's annotation (`int`, `float`, `str`; for `bool` only the flag form is used)
- an unknown option raises `ValueError("unknown option --x")`; missing required parameters raise `TypeError` (from `bind`)
@@starter
import inspect


def call_from_args(func, argv):
    pass


def create_user(name: str, quota: int = 10, admin: bool = False, ratio: float = 1.0):
    return f"{name} quota={quota} admin={admin} ratio={ratio}"


print(call_from_args(create_user, ["maya", "--quota", "50", "--admin"]))
@@tests
check(call_from_args(create_user, ["li"]), "li quota=10 admin=False ratio=1.0", "defaults")
check(call_from_args(create_user, ["--ratio", "0.5", "sam"]), "sam quota=10 admin=False ratio=0.5", "option first")
def copy_files(src: str, dst: str, dry_run: bool = False):
    return (src, dst, dry_run)
check(call_from_args(copy_files, ["a", "b", "--dry-run"]), ("a", "b", True), "dashes to underscores")
try:
    call_from_args(create_user, ["x", "--color", "red"])
    check("ok", "ValueError", "unknown option")
except ValueError as e:
    check(str(e), "unknown option --color", "message")
try:
    call_from_args(copy_files, ["a"])
    check("ok", "TypeError", "missing")
except TypeError:
    pass
@@hint
`params = inspect.signature(func).parameters`. Walk argv with an index. For `--x`: `name = arg[2:].replace("-", "_")`; if not in params raise; if the annotation is `bool`, set True; else take the next item and convert with `params[name].annotation(value)`. Positionals: collect them, then fill in order the parameter names not given as options, converting each. Finally `bound = inspect.signature(func).bind(*positional_values, **kwargs)` and `return func(*bound.args, **bound.kwargs)`.
@@solution
import inspect


def call_from_args(func, argv):
    sig = inspect.signature(func)
    params = sig.parameters
    kwargs, positional = {}, []
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg.startswith("--"):
            name = arg[2:].replace("-", "_")
            if name not in params:
                raise ValueError(f"unknown option {arg}")
            kind = params[name].annotation
            if kind is bool:
                kwargs[name] = True
            else:
                i += 1
                kwargs[name] = kind(argv[i])
        else:
            positional.append(arg)
        i += 1
    free = [n for n in params if n not in kwargs]
    values = [params[n].annotation(v) for n, v in zip(free, positional)]
    bound = sig.bind(*values, **kwargs)
    return func(*bound.args, **bound.kwargs)


def create_user(name: str, quota: int = 10, admin: bool = False, ratio: float = 1.0):
    return f"{name} quota={quota} admin={admin} ratio={ratio}"


print(call_from_args(create_user, ["maya", "--quota", "50", "--admin"]))
