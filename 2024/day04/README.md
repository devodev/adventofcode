# Day 04 - Ceres Search

> [!NOTE]
> Input link: <https://adventofcode.com/2024/day/4/input>

## Part 1

> [!NOTE]
> Problem link: <https://adventofcode.com/2024/day/4>

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt;
she'd like to know if you could help her with her word search (your puzzle input). She only has to
find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even
overlapping other words. It's a little unusual, though, as you don't merely need to find one
instance of XMAS - you need to find all of them.

Take a look at the little Elf's word search. How many times does XMAS appear?

### Part 1 Solution

```bash
$ cargo run --release -- --input 2024/day04/input.txt 2024 04 part1
2573
```

## Part 2

> [!NOTE]
> Problem link: <https://adventofcode.com/2024/day/4#part2>

Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS
puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way
to achieve that is like this:

```txt
M.S
.A.
M.S
```

Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS
can be written forwards or backwards.

Flip the word search from the instructions back over to the word search side and try again. How many
times does an X-MAS appear?

### Part 2 Solution

```bash
$ cargo run --release -- --input 2024/day04/input.txt 2024 04 part2
1850
```
