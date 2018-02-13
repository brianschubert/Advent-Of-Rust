# Advent of Rust

My solutions to the puzzles presented during the [Advent of Code][adventofcode] 
2016 event, written in [Rust][rust].

As with my [previous attempt][aoc-kotlin] at solving the AoC 2017 
puzzles in Kotlin, these solutions represent my first endeavors with 
the Rust language. 

All solutions were compiled with the standard rust compiler, `rustc` 
(`v1.23.0`), inside the community edition of [IntelliJ IDEA][intellij] 
with the Rust plugin.

## Goals

Quite simply, I am using these puzzles learn Rust. 

This repository presents neither the most concise nor the most 
efficient solution to the AoC puzzles. I have tried to write
these solutions to be as idiomatic as possible. Doubtlessly, however,
they are littered with flawed designs and misapplications of 
language or library features.

As I become aware of better practices, I will judiciously refactor
previous solutions. In the majority of cases, however, toiling over
the entire repository simply is not feasible. As such, most "code 
stink" is simply left as-is, serving as a record of my gradual growth
with the Rust language.

## A Word on Stability

Stability takes on a double meaning here. In terms of _rust_ stability, 
these solutions are absolutely stable. That is, all solutions may be 
compiled with a toolchain from the stable release channel 
([more on this][release-channels]), meaning that they are not subject 
to break with time. However, in terms of business logic,
no such guarantee can been made. Numerous solutions take advantage of 
shortcuts that may only be applicable to some puzzle inputs. Moreover, 
many solutions have a nasty habit of panicing rather than returning 
errors values whenever something goes wrong - especially the early 
2016 solutions.

## Running a Solution

A minimal interface is provided for running individual solutions. 

Solutions may be run with cargo via the following format.

```bash
$ cargo run (year) (day) [input-file]
```
For example, to run the solution for the puzzle from 2016 day 1, you 
might execute:

```bash
$ cargo run 2016 1
```

Should you want to use your own input file, simply include the path to 
it as the third argument:

```bash
$ cargo run 2016 1 ./aoc/my/input/file
```

If all goes well, you should be greeted by an output that resembles the 
following (the precise format is subject to change):
    
    Solving 2016 day 02 ... OK
    Input: ./resources/y2016/day02.txt
    
    Part 1: `99332` [0.000457333s]
    Part 2: `DD483` [0.000369839s]
    
    Setup, Parsing: 0.000747620s
    Total Elapsed: 0.001574792s

## Running Tests

Blimey, you really are interested.

Anywho, nearly all modules come fully equipped with unit tests. Most 
solutions are packaged with unit tests for both the examples given in 
the puzzle descriptions as well as for the actual solutions given the 
default input. If you would like to run these, simply run `cargo test` 
on a pattern matching the desired module. For example, in order run all 
the tests associated with the solution for the puzzle from 2016 day 1, 
you might execute:

```bash
$ cargo test --lib 2016::day01
```

Please note, a handful of the included unit tests are rather taxing and 
run for a few minutes. Cargo will ignore these tests by default. If you
insist on running them, simply pass the `--ingnored` flag with the 
executable arguments:

```bash
$ cargo test --lib 2016::day01 -- --ignored
```

Don't forget some popcorn.

## Copyright & License
Copyright &copy; 2018 Brian Schubert - available under [MIT License][license].

[adventofcode]: https://adventofcode.com/
[rust]: https://www.rust-lang.org/en-US/
[aoc-kotlin]: https://github.com/blueschu/Advent-Of-Code
[license]: https://github.com/blueschu/Advent-Of-Rust/blob/master/LICENSE
[intellij]: https://www.jetbrains.com/idea/
[release-channels]: https://doc.rust-lang.org/book/first-edition/release-channels.html
