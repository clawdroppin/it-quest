@@lesson xp-log_tail Multi-file Log Follower
@@text
Follow several log files at the same time, like `tail -f`, prefixing each line with its file name in its own colour, and showing only lines that match a regular expression. One goroutine per file, one channel for all lines.

**You'll practise:** goroutines and channels · bufio.Scanner · regexp · os.Signal for clean shutdown · flag package

## Milestones

1. **Flags**: `-grep REGEX` and a list of files.
2. **Follow**: a goroutine per file that seeks to the end and polls for new lines.
3. **Merge**: all goroutines send `Line{File, Text}` to one channel; main prints them.
4. **Colour**: pick a colour per file from a small palette.
5. **Shutdown**: Ctrl+C stops every goroutine cleanly with a context.

## You're done when

- Appending to two files in another terminal shows their lines live, filtered and coloured.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I detect that a log file was rotated (truncated or replaced)?"*
- *"Is my use of context for cancellation correct?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project log_tail Multi-file Log Follower
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Multi-file Log Follower

Follow several log files at the same time, like `tail -f`, prefixing each line with its file name in its own colour, and showing only lines that match a regular expression. One goroutine per file, one channel for all lines.

## Milestones

- [ ] 1. Flags: `-grep REGEX` and a list of files.
- [ ] 2. Follow: a goroutine per file that seeks to the end and polls for new lines.
- [ ] 3. Merge: all goroutines send `Line{File, Text}` to one channel; main prints them.
- [ ] 4. Colour: pick a colour per file from a small palette.
- [ ] 5. Shutdown: Ctrl+C stops every goroutine cleanly with a context.

## Done when

- [ ] Appending to two files in another terminal shows their lines live, filtered and coloured.
@@file go.mod
module logtail

go 1.22
@@file main.go
// logtail follows several files at once: go run . -grep ERROR app.log web.log
package main

import (
	"flag"
	"fmt"
	"os"
)

type Line struct {
	File string
	Text string
}

func main() {
	grep := flag.String("grep", "", "only show lines matching this regular expression")
	flag.Parse()
	if flag.NArg() == 0 {
		fmt.Fprintln(os.Stderr, "usage: logtail [-grep REGEX] FILE...")
		os.Exit(2)
	}
	lines := make(chan Line)
	// TODO: start a follower goroutine per file that sends to lines
	_ = grep
	for l := range lines {
		fmt.Printf("%s: %s\n", l.File, l.Text)
	}
}
