@@lesson xp-algorithm_tracer Algorithm Trace Trainer
@@text
The 基本情報技術者 exam's algorithm questions are about tracing: following variables through loops by hand. Implement the classic algorithms (linear and binary search, bubble, selection and insertion sort, stack and queue operations), make each one print a trace table, and use it to check your hand traces of past-exam style questions.

**You'll practise:** trace tables · searching and sorting · array indexes (1-based in the exam!) · stacks and queues

## Milestones

1. **Trace helper**: a function that prints the variables you choose after every step.
2. **Searches**: linear and binary search with traces.
3. **Sorts**: bubble, selection and insertion sort, counting comparisons and swaps.
4. **Exam style**: write versions with 1-based arrays like the exam's pseudo-code.
5. **Practice**: trace three inputs by hand first, then check with the program.

## You're done when

- Your hand traces match the program on at least five inputs.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Explain how binary search's low and high change on this input step by step."*
- *"Why does insertion sort do fewer swaps on nearly sorted data?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project algorithm_tracer Algorithm Trace Trainer
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Algorithm Trace Trainer

The 基本情報技術者 exam's algorithm questions are about tracing: following variables through loops by hand. Implement the classic algorithms (linear and binary search, bubble, selection and insertion sort, stack and queue operations), make each one print a trace table, and use it to check your hand traces of past-exam style questions.

## Milestones

- [ ] 1. Trace helper: a function that prints the variables you choose after every step.
- [ ] 2. Searches: linear and binary search with traces.
- [ ] 3. Sorts: bubble, selection and insertion sort, counting comparisons and swaps.
- [ ] 4. Exam style: write versions with 1-based arrays like the exam's pseudo-code.
- [ ] 5. Practice: trace three inputs by hand first, then check with the program.

## Done when

- [ ] Your hand traces match the program on at least five inputs.
@@file trace.py
"""Algorithm traces for FE practice: python trace.py"""


def trace(step: int, **vars) -> None:
    print(f"{step:>3}: " + "  ".join(f"{k}={v}" for k, v in vars.items()))


def binary_search(a: list[int], target: int) -> int:
    """1-based like the exam: returns the position of target in sorted a, or 0."""
    low, high, step = 1, len(a), 0
    while low <= high:
        mid = (low + high) // 2
        step += 1
        trace(step, low=low, high=high, mid=mid, value=a[mid - 1])
        if a[mid - 1] == target:
            return mid
        if a[mid - 1] < target:
            low = mid + 1
        else:
            high = mid - 1
    return 0


# TODO: linear search, bubble / selection / insertion sort with traces

if __name__ == "__main__":
    print("found at", binary_search([3, 8, 15, 21, 34, 55, 89], 34))
