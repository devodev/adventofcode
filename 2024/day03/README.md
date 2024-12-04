# Day 03 - Mull It Over

> [!NOTE]
> Input link: <https://adventofcode.com/2024/day/3/input>

## Part 1

> [!NOTE]
> Problem link: <https://adventofcode.com/2024/day/3>

The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted.
All of the instructions have been jumbled up!

It seems like the goal of the program is just to multiply some numbers. It does that with
instructions like `mul(X,Y)`, where X and Y are each 1-3 digit numbers. For instance, `mul(44,46)`
multiplies 44 by 46 to get a result of 2024. Similarly, `mul(123,4)` would multiply 123 by 4.

However, because the program's memory has been corrupted, there are also many invalid characters
that should be ignored, even if they look like part of a mul instruction. Sequences like
`mul(4*, mul(6,9!, ?(12,34)`, or `mul ( 2 , 4 )` do nothing.

Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the
results of the multiplications?

### Part 1 Solution

```bash
$ cargo run --release -- --input 2024/day03/input.txt 2024 03 part1
170068701
```

## Part 2

> [!NOTE]
> Problem link: <https://adventofcode.com/2024/day/3#part2>

There are two new instructions you'll need to handle:

- The `do()` instruction enables future mul instructions.
- The `don't()` instruction disables future mul instructions.

Only the most recent `do()` or `don't()` instruction applies. At the beginning of the program, mul
instructions are enabled.

Handle the new instructions; what do you get if you add up all of the results of just the enabled
multiplications?

### Part 2 Solution

```bash
$ cargo run --release -- --input 2024/day03/input.txt 2024 03 part2
78683433
```
