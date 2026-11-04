@@lesson ru4-text-lists Text Wrapping and Virtual Lists: Fast UIs with Lots of Content
@@concepts lists, strings, numbers
@@lang rust
@@text
Two things make GUIs slow as content grows: laying out lots of text, and creating widgets for thousands of rows that aren't even visible. This lesson explains how text layout works (shaping, line breaking, Unicode details that matter for Japanese), and how **virtualised lists** render only what's on screen. The exercises implement greedy word wrapping and the visible-range maths of a virtual list.

## Text layout in brief

1. **Shaping** turns a string into positioned glyphs with a font (HarfBuzz; in Rust `rustybuzz`, used by `cosmic-text`, `parley` and others). Ligatures, kerning and complex scripts happen here.
2. **Line breaking** decides where lines can wrap: Unicode **UAX #14** defines break opportunities. Spaces in English; between almost any characters in Japanese (except before 。、」 or after 「: kinsoku rules); never inside a word unless it doesn't fit at all.
3. **Bidi** (UAX #9) orders mixed right-to-left and left-to-right text.
4. Measure in **grapheme clusters** (what users see as one character: `é` can be two code points, a flag emoji is two, a family emoji many). Truncating in the middle of a cluster draws garbage; the `unicode-segmentation` crate splits them.
- egui uses its own `epaint` text layout (galleys) and caches them per frame; iced and Slint use `cosmic-text`/`parley`-style shaping. Cache layouts: re-shaping unchanged text every frame wastes time.

## Greedy wrapping

The common algorithm: add words to the current line while they fit the width; otherwise start a new line. A word longer than the line is broken at character boundaries. (TeX's Knuth-Plass optimises the whole paragraph for even spacing but costs more.)

## Virtualised lists

Rendering 100,000 rows as widgets is slow; the user sees maybe 20. A **virtual list** computes which rows intersect the viewport and lays out only those, plus a few extra (**overscan**) to avoid blank flashes while scrolling:
- With a fixed row height: `first = floor(offset / h)`, `last = ceil((offset + viewport) / h) − 1`, clamped and widened by overscan. The scroll area's total height is `rows × h`.
- With variable heights: keep a prefix sum of heights (estimated for rows not yet measured) and binary-search the offset.
- egui: `ScrollArea::show_rows(ui, row_height, total_rows, |ui, range| ...)`. iced has lazy widgets; Slint `ListView` virtualises automatically.
- Keep **keys** stable (row IDs, not indexes) so selection and animations survive inserts.

@@quiz
Why truncate text at grapheme cluster boundaries?
* A single visible character can be several code points, and cutting inside it breaks rendering
- It's faster than byte boundaries
- Rust strings require it
- Fonts can only draw clusters
? Accents, flags and family emoji.

@@quiz
Where may Japanese text usually wrap?
- Only at spaces
* Between most characters, except kinsoku cases like before 。 or after 「
- Never
- Only after kanji
? UAX #14 plus kinsoku rules.

@@quiz
Rows are 24 px tall, the viewport is 480 px and the offset is 1000 px. Which row is first visible?
- 40
* 41
- 42
- 20
? floor(1000 / 24) = 41.

@@quiz
What is overscan in a virtual list?
- Scrolling past the end
* Rendering a few extra rows beyond the visible ones
- A bigger font
- Hidden scroll bars
? Prevents blank flashes while scrolling.

@@quiz
What makes variable-height virtual lists harder?
- Nothing
* Finding the row at an offset needs prefix sums of (estimated) heights
- They can't scroll
- They require GPU rendering
? Binary-search the cumulative heights.

@@flashcards
shaping :: string to positioned glyphs
HarfBuzz / rustybuzz :: shaping engines
UAX #14 :: line break opportunities
kinsoku :: Japanese line-break rules
UAX #9 :: bidirectional text
grapheme cluster :: user-perceived character
unicode-segmentation :: split graphemes
greedy wrap :: fill lines word by word
Knuth-Plass :: whole-paragraph line breaking
galley :: egui laid-out text
virtual list :: render visible rows only
overscan :: extra rows above and below
show_rows :: egui virtual scrolling
prefix sum :: variable height lookup
stable keys :: IDs, not indexes

@@drill
Which Unicode annex defines line-breaking opportunities?
= UAX #14
= UAX 14
= uax14
? UAX #9 is bidi.

@@drill
What is the name for rendering a few extra rows outside the viewport?
= overscan
? Smooths fast scrolling.

@@exercise Greedy word wrap
Implement `wrap(text, width)` for a monospace display (every `char` is 1 column): split on whitespace, fill lines greedily with single spaces between words, and break a word longer than `width` into chunks. `width` 0 returns no lines. Then `truncate(text, width)` that shortens text to fit with `…` (which takes 1 column) when it doesn't fit.
@@starter
fn wrap(text: &str, width: usize) -> Vec<String> {
    todo!()
}

fn truncate(text: &str, width: usize) -> String {
    todo!()
}

fn main() {
    for line in wrap("The quick brown fox jumps over the extraordinarily lazy dog", 12) {
        println!("|{line:<12}|");
    }
    println!("{}", truncate("Settings and preferences", 10));
}
@@tests
#[test]
fn wraps_words() {
    assert_eq!(wrap("the quick brown fox", 10), vec!["the quick", "brown fox"]);
    assert_eq!(wrap("  spaced   out  ", 20), vec!["spaced out"]);
    assert_eq!(wrap("", 10), Vec::<String>::new());
    assert_eq!(wrap("anything", 0), Vec::<String>::new());
}

#[test]
fn exact_fit() {
    assert_eq!(wrap("abcd efgh", 9), vec!["abcd efgh"]);
    assert_eq!(wrap("abcd efgh", 8), vec!["abcd", "efgh"]);
}

#[test]
fn long_words_are_split() {
    assert_eq!(wrap("a extraordinarily b", 6), vec!["a", "extrao", "rdinar", "ily b"]);
}

#[test]
fn counts_chars_not_bytes() {
    assert_eq!(wrap("日本語 テキスト", 4), vec!["日本語", "テキスト"]);
}

#[test]
fn truncates() {
    assert_eq!(truncate("Settings", 10), "Settings");
    assert_eq!(truncate("Settings and preferences", 10), "Settings …");
    assert_eq!(truncate("日本語のテキスト", 5), "日本語の…");
    assert_eq!(truncate("abc", 0), "");
}
@@hint
Keep a `current: String` and its length in chars. For each word, if it's longer than `width`, first push `current` (if not empty), then push full chunks (`chars().collect::<Vec<_>>().chunks(width)`), keeping the last partial chunk as the new `current`. Otherwise, if `current_len + 1 + word_len > width`, push and start over. For `truncate`, take `width - 1` chars and add `…`.
@@solution
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    if width == 0 {
        return lines;
    }
    let mut current = String::new();
    let mut len = 0;
    for word in text.split_whitespace() {
        let chars: Vec<char> = word.chars().collect();
        let mut pieces: Vec<String> = chars.chunks(width).map(|c| c.iter().collect()).collect();
        if pieces.len() > 1 {
            if len > 0 {
                lines.push(std::mem::take(&mut current));
            }
            let last = pieces.pop().unwrap();
            lines.extend(pieces);
            len = last.chars().count();
            current = last;
            continue;
        }
        let wlen = chars.len();
        if len == 0 {
            current = word.to_string();
            len = wlen;
        } else if len + 1 + wlen <= width {
            current.push(' ');
            current.push_str(word);
            len += 1 + wlen;
        } else {
            lines.push(std::mem::replace(&mut current, word.to_string()));
            len = wlen;
        }
    }
    if len > 0 {
        lines.push(current);
    }
    lines
}

fn truncate(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_string();
    }
    if width == 0 {
        return String::new();
    }
    let mut out: String = text.chars().take(width - 1).collect();
    out.push('…');
    out
}

fn main() {
    for line in wrap("The quick brown fox jumps over the extraordinarily lazy dog", 12) {
        println!("|{line:<12}|");
    }
    println!("{}", truncate("Settings and preferences", 10));
}

@@exercise Visible rows of a virtual list
Implement `visible_range(total, row_h, viewport_h, offset, overscan)` for fixed-height rows, returning the half-open range of rows to render (clamped to `0..total`), and `VariableList` for rows of different heights with a prefix sum and binary search: `row_at(offset)` and `range(offset, viewport_h)`.
@@starter
use std::ops::Range;

fn visible_range(total: usize, row_h: f32, viewport_h: f32, offset: f32, overscan: usize) -> Range<usize> {
    todo!()
}

struct VariableList { starts: Vec<f32> } // starts[i] = top of row i; last entry = total height

impl VariableList {
    fn new(heights: &[f32]) -> Self {
        todo!()
    }
    fn row_at(&self, offset: f32) -> usize {
        todo!()
    }
    fn range(&self, offset: f32, viewport_h: f32) -> Range<usize> {
        todo!()
    }
}

fn main() {
    println!("{:?}", visible_range(100_000, 24.0, 480.0, 1000.0, 3));
    let list = VariableList::new(&[40.0, 24.0, 24.0, 120.0, 24.0, 24.0, 60.0]);
    println!("{:?}", list.range(50.0, 100.0));
}
@@tests
#[test]
fn fixed_rows() {
    assert_eq!(visible_range(100_000, 24.0, 480.0, 1000.0, 0), 41..62);
    assert_eq!(visible_range(100_000, 24.0, 480.0, 1000.0, 3), 38..65);
    assert_eq!(visible_range(100_000, 24.0, 480.0, 0.0, 3), 0..23);
    assert_eq!(visible_range(10, 24.0, 480.0, 0.0, 3), 0..10, "clamped to total");
    assert_eq!(visible_range(0, 24.0, 480.0, 0.0, 3), 0..0);
    assert_eq!(visible_range(100, 20.0, 100.0, 100.0, 0), 5..10, "rows ending exactly at the edge are not included");
}

#[test]
fn variable_rows() {
    let list = VariableList::new(&[40.0, 24.0, 24.0, 120.0, 24.0, 24.0, 60.0]);
    assert_eq!(list.starts, vec![0.0, 40.0, 64.0, 88.0, 208.0, 232.0, 256.0, 316.0]);
    assert_eq!(list.row_at(0.0), 0);
    assert_eq!(list.row_at(39.9), 0);
    assert_eq!(list.row_at(40.0), 1);
    assert_eq!(list.row_at(150.0), 3);
    assert_eq!(list.row_at(10_000.0), 6, "past the end clamps to the last row");
    assert_eq!(list.range(50.0, 100.0), 1..4);
    assert_eq!(list.range(200.0, 56.0), 3..6);
    assert_eq!(list.range(200.0, 60.0), 3..7, "row 6 starts at 256, inside the viewport");
}
@@hint
Fixed: `first = (offset / row_h).floor()`, `end = ((offset + viewport_h) / row_h).ceil()`; subtract/add overscan with `saturating_sub` and `min(total)`. Variable: `starts` is a running sum beginning with 0. `row_at` is the last index whose start is `<= offset`: `self.starts.partition_point(|&s| s <= offset) - 1`, clamped to the last row. The range ends after the row containing `offset + viewport_h` minus a tiny epsilon.
@@solution
use std::ops::Range;

fn visible_range(total: usize, row_h: f32, viewport_h: f32, offset: f32, overscan: usize) -> Range<usize> {
    if total == 0 {
        return 0..0;
    }
    let first = (offset / row_h).floor() as usize;
    let end = ((offset + viewport_h) / row_h).ceil() as usize;
    first.saturating_sub(overscan).min(total)..(end + overscan).min(total)
}

struct VariableList { starts: Vec<f32> } // starts[i] = top of row i; last entry = total height

impl VariableList {
    fn new(heights: &[f32]) -> Self {
        let mut starts = vec![0.0];
        for h in heights {
            let last = *starts.last().unwrap();
            starts.push(last + h);
        }
        VariableList { starts }
    }
    fn row_at(&self, offset: f32) -> usize {
        let rows = self.starts.len() - 1;
        let i = self.starts.partition_point(|&s| s <= offset).saturating_sub(1);
        i.min(rows.saturating_sub(1))
    }
    fn range(&self, offset: f32, viewport_h: f32) -> Range<usize> {
        let first = self.row_at(offset);
        let last = self.row_at(offset + viewport_h - 0.001);
        first..last + 1
    }
}

fn main() {
    println!("{:?}", visible_range(100_000, 24.0, 480.0, 1000.0, 3));
    let list = VariableList::new(&[40.0, 24.0, 24.0, 120.0, 24.0, 24.0, 60.0]);
    println!("{:?}", list.range(50.0, 100.0));
}
