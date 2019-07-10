# Advent of Rust

My solutions to the [Advent of Code][adventofcode] puzzles, written in [Rust][rust].


As with my [previous attempt][aoc-kotlin] at solving the AoC 2017 
puzzles in Kotlin, these solutions represent my first endeavors with 
the Rust programming language. 

All solutions were compiled with Rust v1.23.0.

## Running a Solution

Solutions may be run using cargo with the following format:

```bash
$ cargo run year day [input-file]
```
For example, to run the solution for the puzzle from Day 2 of 2016, you can write:

```bash
$ cargo run 2016 2
```

If you would like to use your own input file, simply include the path to 
it as the third argument:

```bash
$ cargo run 2016 2 ~/my/input/file
```

It is worth noting that a handful of solutions take advantage of patterns
that exist in my puzzle inputs, but which may not be present in all valid 
inputs. As such, my solutions _might_ be liable to panic when given input 
different from the ones that they were designed against.

If all goes well, you should be met with an output that resembles the 
following:
    
    Solving 2016 day 02 ... OK
    Input: ./resources/y2016/day02.txt
    
    Part 1: `99332` [0.000457333s]
    Part 2: `DD483` [0.000369839s]
    
    Setup, Parsing: 0.000747620s
    Total Elapsed: 0.001574792s



## Running Tests

Nearly all modules in this repository come equipped with unit tests.

Most solutions are packaged with unit tests for both the examples in 
the puzzle descriptions as well as for the actual solutions for my inputs.

If you would like to run these tests, you can use the `cargo test` command,
which is documentation in the [Cargo Book][cargo-book]

As an example, to run all the tests associated with the puzzle from Day 2 of 2016, 
you can write:

```bash
$ cargo test 2016::day02
```

A handful of the included unit tests take a few minutes to run. 
Cargo will ignore these tests by default, but, if you would like to run them,
pass the `--ignored` flag with the _test binary's_ arguments (after a `--`):

```bash
$ cargo test 2016::day02 -- --ignored
```

(Don't forget some popcorn!)

## Copyright & License
Copyright &copy; 2018 Brian Schubert - available under [MIT License][license].

[adventofcode]: https://adventofcode.com/
[rust]: https://www.rust-lang.org/en-US/
[aoc-kotlin]: https://github.com/blueschu/Advent-Of-Code
[license]: https://github.com/blueschu/Advent-Of-Rust/blob/master/LICENSE
[cargo-book]: https://doc.rust-lang.org/cargo/guide/tests.html
