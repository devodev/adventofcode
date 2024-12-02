# Day 02 - Red-Nosed Reports

> [!NOTE]
> Input link: <https://adventofcode.com/2024/day/2/input>

## Part 1

> [!NOTE]
> Problem link: <https://adventofcode.com/2024/day/2>

The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a
list of numbers called levels that are separated by spaces. For example:

```txt
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
```

This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems
can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report
only counts as safe if both of the following are true:

- The levels are either all increasing or all decreasing.
- Any two adjacent levels differ by at least one and at most three.

Analyze the unusual data from the engineers. How many reports are safe?

### Part 1 Solution

```bash
$ cargo run --release -- --input 2024/day02/input.txt 2024 02 part1
252
```

## Part 2

> [!NOTE]
> Problem link: <https://adventofcode.com/2024/day/2#part2>

Now, the same rules apply as before, except if removing a single level from an unsafe report would
make it safe, the report instead counts as safe.

### Part 2 Solution

```bash
$ cargo run --release -- --input 2024/day02/input.txt 2024 02 part2
324
```
