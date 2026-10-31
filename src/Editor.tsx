// CodeMirror 6 editor, for the learner's code and (read-only) for lesson examples.
import { useEffect, useRef } from "react";
import { EditorView, basicSetup } from "codemirror";
import { EditorState, Extension, Prec } from "@codemirror/state";
import { keymap } from "@codemirror/view";
import { indentWithTab } from "@codemirror/commands";
import { StreamLanguage, indentUnit } from "@codemirror/language";
import { oneDark } from "@codemirror/theme-one-dark";
import { python } from "@codemirror/lang-python";
import { javascript } from "@codemirror/lang-javascript";
import { rust } from "@codemirror/lang-rust";
import { sql } from "@codemirror/lang-sql";
import { powerShell } from "@codemirror/legacy-modes/mode/powershell";
import { shell } from "@codemirror/legacy-modes/mode/shell";
import { go } from "@codemirror/legacy-modes/mode/go";
import { cpp, csharp } from "@codemirror/legacy-modes/mode/clike";
import { html } from "@codemirror/legacy-modes/mode/xml";
import { Lang } from "./api";

function language(lang: Lang | null): Extension {
  switch (lang) {
    case "python":
      return python();
    case "javascript":
      return javascript();
    case "rust":
      return rust();
    case "sql":
      return sql();
    case "powershell":
      return StreamLanguage.define(powerShell);
    case "bash":
      return StreamLanguage.define(shell);
    case "typescript":
      return javascript({ typescript: true });
    case "go":
      return StreamLanguage.define(go);
    case "csharp":
      return StreamLanguage.define(csharp);
    case "cpp":
      return StreamLanguage.define(cpp);
    case "html":
      return StreamLanguage.define(html);
    default:
      return [];
  }
}

const theme = EditorView.theme({
  "&": { fontSize: "0.92em", height: "100%" },
  ".cm-scroller": { fontFamily: "var(--mono)", lineHeight: "1.5" },
  ".cm-content": { paddingBottom: "40px" },
});

const readOnlyTheme = EditorView.theme({
  "&": { fontSize: "0.9em" },
  ".cm-scroller": { fontFamily: "var(--mono)", lineHeight: "1.45" },
  ".cm-gutters": { display: "none" },
  ".cm-activeLine": { backgroundColor: "transparent" },
  ".cm-cursor": { display: "none !important" },
});

interface Props {
  value: string;
  lang: Lang | null;
  onChange?: (code: string) => void;
  onRun?: () => void;
  onCheck?: () => void;
  readOnly?: boolean;
}

export function CodeEditor({ value, lang, onChange, onRun, onCheck, readOnly }: Props) {
  const host = useRef<HTMLDivElement>(null);
  const view = useRef<EditorView | null>(null);
  // Callbacks change every render; the editor reads them through refs.
  const cb = useRef({ onChange, onRun, onCheck });
  cb.current = { onChange, onRun, onCheck };

  useEffect(() => {
    const extensions: Extension[] = readOnly
      ? [EditorState.readOnly.of(true), EditorView.editable.of(false), language(lang), oneDark, readOnlyTheme, EditorView.lineWrapping]
      : [
          basicSetup,
          // Highest precedence: the default keymap uses Ctrl+Enter for "insert blank line".
          Prec.highest(
            keymap.of([
              { key: "Mod-Shift-Enter", run: () => ((cb.current.onCheck ?? cb.current.onRun)?.(), true) },
              { key: "Mod-Enter", run: () => (cb.current.onRun?.(), true) },
              { key: "F5", run: () => (cb.current.onRun?.(), true) },
            ]),
          ),
          keymap.of([indentWithTab]),
          indentUnit.of(lang === "javascript" ? "  " : "    "),
          language(lang),
          oneDark,
          theme,
          EditorView.updateListener.of((u) => {
            if (u.docChanged) cb.current.onChange?.(u.state.doc.toString());
          }),
        ];
    const v = new EditorView({ state: EditorState.create({ doc: value, extensions }), parent: host.current! });
    view.current = v;
    return () => v.destroy();
    // The editor is rebuilt only when its language or mode changes; value changes are synced below.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [lang, readOnly]);

  useEffect(() => {
    const v = view.current;
    if (v && v.state.doc.toString() !== value) {
      v.dispatch({ changes: { from: 0, to: v.state.doc.length, insert: value } });
    }
  }, [value]);

  return <div className={readOnly ? "code-static" : "code-editor"} ref={host} />;
}
