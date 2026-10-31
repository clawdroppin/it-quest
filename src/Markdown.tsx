// Markdown for lessons and AI answers. Links open in the learner's browser.
import { useMemo } from "react";
import { marked } from "marked";
import DOMPurify from "dompurify";
import { api } from "./api";

marked.setOptions({ gfm: true, breaks: false });

export function renderMarkdown(md: string): string {
  return DOMPurify.sanitize(marked.parse(md, { async: false }) as string);
}

export function Markdown({ text, className }: { text: string; className?: string }) {
  const html = useMemo(() => renderMarkdown(text), [text]);
  return (
    <div
      className={"md " + (className ?? "")}
      dangerouslySetInnerHTML={{ __html: html }}
      onClick={(e) => {
        const a = (e.target as HTMLElement).closest("a");
        if (a) {
          e.preventDefault();
          const href = a.getAttribute("href") ?? "";
          if (/^https?:\/\//.test(href)) api.openUrl(href);
        }
      }}
    />
  );
}
