# aoc_2022
My solutions for Advent of Code 2022, all written in Rust. 

See https://adventofcode.com/2022/ for more information on the competition itself.

## Days
01. [day01.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day01.rs) - There's not much to say here. It's a simple case of reading a text file, converting strings to integers, and performing basic arithmetic. The unit tests verify the examples given in the problem description.
02. [day02.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day02.rs) -  This was a fun [challenge](https://adventofcode.com/2022/day/2) based around Rock Paper Scissors. My first solution spelled out each permutation, but my [revamped solution](https://github.com/bsinglet/aoc_2022/blob/master/src/day02_alt_solution.rs) used a more elegant solution.
03. [day03.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day03.rs) - Another beginner-level challenge. Rust's [HashSets](https://doc.rust-lang.org/std/collections/struct.HashSet.html) made quick work of this.
04. [day04.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day04.rs) - This was a very easy challenge, but I initially misunderstood Part 2 here and solved a much more difficult task than it was describing! I left the harder solution in a comment block in the code. 
05. [day05.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day05.rs) - This was the first day I had to implement extensive unit tests for. The [challenge](https://adventofcode.com/2022/day/5) itself is pretty simple: keeping track of stacks of boxes as you move elements from one to stack to another. However, the puzzle input format and number of operations involved introduced many potential off-by-one errors that couldn't be easily isolated. The unit tests here step through operations ensuring all internal states match up with what we expect.
06. [day06.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day06.rs) - This was a fun little challenge, only requiring about nine lines of code for Part 1, and another nine for Part 2. The unit tests cover the many different examples covered on the official page.
07. [day07.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day07.rs) - This was the first challenge that made me wish I was using Python instead of Rust. The intuitive solution for a problem like this is to represent each file/directory as a map (or in Python terminology, a dict). Rust's [HashMaps](https://doc.rust-lang.org/std/collections/struct.HashMap.html), however, do not allow varying data types among its values, nor is there a good way for a HashMap to contain the child HashMaps. Instead, I created a Struct to hold the attributes of a file/directory, and used two vectors to hold them, one for files and one for directories. This made it surprisingly easy to traverse the directory structure, rolling file sizes up to higher levels. Also, the [regex crate](https://docs.rs/regex/1.7.0/regex/) made it very easy to parse the input intuitively, while unit tests were invaluable for making sure helper functions worked correctly.
08. [day08.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day08.rs)
09. [day09.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day09.rs)
10. [day10.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day10.rs)
11. [day11.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day11.rs)
12. [day12.rs](https://github.com/bsinglet/aoc_2022/blob/master/src/day12.rs)
