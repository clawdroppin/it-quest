"""stub_braces.py SRC DST STATEMENT name1 name2 ...

The stub.py of brace languages (JavaScript, TypeScript, C#, C++, Go, PowerShell): copies SRC to DST
with the body of each named function or method replaced by STATEMENT, for example
    'throw new Error("TODO: not done yet");'          JavaScript / TypeScript
    'throw new NotImplementedException();'            C#
    'throw std::logic_error("TODO: not done yet");'   C++

A function is found by its first line: the name followed by "(" with only keywords, a type or
modifiers before it, and the line ending in "{". Its body ends at the first later line that is the
same indentation followed by "}" (so the code must be formatted, which ours always is).
Comments above the function stay, so they can explain what to write.
"""
import re
import sys

src, dst, statement, *names = sys.argv[1:]
lines = open(src, encoding="utf-8").read().split("\n")
out, i, done = [], 0, set()
while i < len(lines):
    line = lines[i]
    m = re.match(r"^(\s*)(.*?)\b(\w+)\s*(<[^>]*>)?\s*\(", line)
    if m and m.group(3) in names and line.rstrip().endswith("{") and not re.match(r"\s*(if|for|while|switch|return)\b", line):
        indent = m.group(1)
        end = i + 1
        while not re.match(re.escape(indent) + r"\}", lines[end]) or lines[end][len(indent):].startswith(" "):
            end += 1
        out += [line, f"{indent}  {statement}" if "\t" not in indent else f"{indent}\t{statement}", lines[end]]
        done.add(m.group(3))
        i = end + 1
        continue
    out.append(line)
    i += 1
missing = set(names) - done
if missing:
    sys.exit(f"not found: {sorted(missing)}")
open(dst, "w", encoding="utf-8", newline="\n").write("\n".join(out))
print("wrote", dst)
