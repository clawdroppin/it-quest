"""stub.py SRC DST name1 name2 ... : copy SRC to DST with the named top-level functions'
bodies replaced by their docstring + extra hint comments + raise NotImplementedError.
Hints: a comment block directly ABOVE a def, starting with '# HINT', is moved inside."""
import ast
import sys

src_path, dst_path, *names = sys.argv[1:]
src = open(src_path, encoding="utf-8").read()
lines = src.splitlines(keepends=True)
tree = ast.parse(src)
edits = []
for node in tree.body:
    if isinstance(node, ast.FunctionDef) and node.name in names:
        body = node.body
        indent = " " * body[0].col_offset
        has_doc = isinstance(body[0], ast.Expr) and isinstance(getattr(body[0], "value", None), ast.Constant) \
            and isinstance(body[0].value.value, str)
        start = body[1].lineno - 1 if has_doc and len(body) > 1 else body[0].lineno - 1
        if has_doc and len(body) == 1:
            start = body[0].end_lineno
        end = node.end_lineno
        edits.append((start, end, f"{indent}raise NotImplementedError\n"))
        names.remove(node.name)
if names:
    sys.exit(f"not found: {names}")
for start, end, text in sorted(edits, reverse=True):
    lines[start:end] = [text]
out = "".join(lines)
open(dst_path, "w", encoding="utf-8", newline="\n").write(out)
print("wrote", dst_path)
