# Advent of Rust

My solutions to the puzzles presented during the [Advent of Code][adventofcode] 
2015 and 2016 events, written in [Rust][rust].

As with my [previous attempt][aoc-kotlin] at solving the AoC 2017 
puzzles in Kotlin, these solutions represent my first endeavors with 
the Rust programming language. 

All solutions were compiled with the standard rust compiler, `rustc` 
(`v1.23.0`).


## A Word on Stability

All solutions in this repository can be compiled with using the stable release
channel of Rust and as such are not liable to become dysfunctional with time
([more on this][release-channels]).

It is worth noting, however, that some of these solutions take advantage of
patterns in my puzzle inputs that may not be present in all valid inputs.
Since I cannot guarantee that these shortcuts are universally applicable,
my solutions _might_ be liable to panic when given input different from
theones that they were designed against.

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

Nearly all modules in this repsoitory come fully equipped with unit tests.
Most solutions are packaged with unit tests for both the examples given in 
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
