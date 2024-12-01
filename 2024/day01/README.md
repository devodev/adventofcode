# Day 01 - Historian Hysteria

> [!NOTE]
> Input link: <https://adventofcode.com/2024/day/1/input>

## Part 1

> [!NOTE]
> Problem link: <https://adventofcode.com/2024/day/1>

There's just one problem: by holding the two lists up side by side (your puzzle input), it quickly
becomes clear that the lists aren't very similar. Maybe you can help The Historians reconcile their
lists?

For example:

```txt
3   4
4   3
2   5
1   3
3   9
3   3
```

Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far
apart they are. Pair up the smallest number in the left list with the smallest number in the right
list, then the second-smallest left number with the second-smallest right number, and so on.

Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those
distances. For example, if you pair up a 3 from the left list with a 7 from the right list, the
distance apart is 4; if you pair up a 9 with a 3, the distance apart is 6.

To find the total distance between the left list and the right list, add up the distances between
all of the pairs you found.

Your actual left and right lists contain many location IDs. What is the total distance between your
lists?

### Part 1 Solution

> [!NOTE]
> Input link: <https://adventofcode.com/2024/day/1/input>

```bash
# cargo run --release -- --input 2024/day01/input.txt 2024 01 part1-binary-heap
$ cargo run --release -- --input 2024/day01/input.txt 2024 01 part1-sort
1110981
```

## Part 2

> [!NOTE]
> Problem link: <https://adventofcode.com/2024/day/1#part2>

This time, you'll need to figure out exactly how often each number from the left list appears in the
right list. Calculate a total similarity score by adding up each number in the left list after
multiplying it by the number of times that number appears in the right list.

Once again consider your left and right lists. What is their similarity score?

### Part 2 Solution

```bash
$ cargo run --release -- --input 2024/day01/input.txt 2024 01 part2
24869388
```
