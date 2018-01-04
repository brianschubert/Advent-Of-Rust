# Advent of Rust

My solutions to the puzzles presented during the [Advent of Code][adventofcode] 2016 event, written in [Rust][rust].

As with my [previous attempt][aoc-kotlin] at solving the AoC 2017 puzzles in Kotlin, these soltuiosn represent my first endeavors with the Rust language. 

All solutions were compiled with the standard rust compiler, `rustc`, inside the community edition of [IntelliJ IDEA][intellij] with the Rust plugin,   

## Running a Solution

Solutions may be run with cargo via the following format.
```bash
$ cargo run (year) (day)
```
For example, to run the solution for AoC 2016 day 1, you might execute:
```bash
$ cargo run 2016 1
```

Most solutions are packaged with unit tests for the puzzles examples. If you would like to run these, simply run `cargo test` on pattern matching the desired module. For example, in order to test the above referenced solution, you might execute:
```bash
$ cargo test --lib 2016::day01
```

## Copyright & License
Copyright &copy; 2018 Brian Schubert - available under [MIT License][license].

[adventofcode]: https://adventofcode.com/
[rust]: https://www.rust-lang.org/en-US/
[aoc-kotlin]: https://github.com/blueschu/Advent-Of-Code
[license]: https://github.com/blueschu/Advent-Of-Rust/blob/master/LICENSE
[intellij]: https://www.jetbrains.com/idea/
