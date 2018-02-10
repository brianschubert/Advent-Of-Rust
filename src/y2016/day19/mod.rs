//! Solution for 2016 Day 19
//!
//! I spent a few hours (unsuccessfully) trying to solve part one with
//! pure math. I eventually stumbled across a [very similar problem](josephus)
//! which served as the basis for my solution.
//!
//! After a similar hassle with part two, I resolved to simply brute
//! force it. My first attempt involved simply simulating the problem
//! by remove and shifting elements in a vector. As one would expect,
//! this implementation, while feasible for small "elf" groups, proved
//! far too inefficient to solve for the puzzle's input. It did,
//! however, allow me to generate a table mapping group size to the
//! index of the "winning member," which revealed the relationship
//! that I exploited in my final solution.
//!
//! [josephus]: https://en.wikipedia.org/wiki/Josephus_problem

use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_string(puzzle)?;
    let input = input.trim_right().parse()?;

    solve_parts! {
        1 => part_one(input),
        2 => part_two(input)
    }
}

/// Returns the greatest power of `base` less than or equal to `cap`.
fn greatest_power(base: u32, cap: u32) -> u32 {
    if cap < base { return 0; }
    let mut acc = base;
    loop {
        let next = acc * base;
        if next > cap {
            break acc;
        }
        acc = next;
    }
}

/// Return the least power of `base` greater than or equal to `shoe`.
fn least_power(base: u32, shoe: u32) -> u32 {
    let mut acc = base;
    loop {
        if acc >= shoe {
            break acc;
        }
        acc *= base;
    }
}

/// Returns the elf who will obtain all the presents in part one.
fn part_one(elf_count: u32) -> u32 {
    debug_assert_ne!(0, elf_count);
    if 1 == elf_count { return 1; }

    let origin = greatest_power(2, elf_count);
    let offset = elf_count - origin;
    2 * offset + 1
}

/// Returns the elf who will obtain all the presents in part two.
///
/// From what I gather, this function resets every `3^m` values.
/// After each reset, the first `3^(m-1)` values are follow a slope
/// of `1`. The following `3^(m-1)` values, beginning at `3^m - 3^(m-1)`,
/// continue with a slope of `2`, mapping solely to odd values until
/// eventually reaching `3^m`, which maps to itself.
///
/// ### A visual example:
///
/// | Group Size | Last Elf | Note         |
/// |:----------:|:--------:|--------------|
/// | ...        |          |              |
/// | 26         | 25       |              |
/// | 27         | 27       |              |
/// | 28         | 1        | Resets here  |
/// | 29         | 2        | Inc. by 1    |
/// | 30         | 3        |              |
/// | ...        |          |              |
/// | 53         | 26       |              |
/// | 54         | 27       | Reach `3^(m-1)` |
/// | 55         | 29       | Inc. by 2    |
/// | 56         | 31       |              |
/// | ...        |          |              |
/// | 79         | 77       |              |
/// | 80         | 79       |              |
/// | 81         | 81       | Reach `3^m`  |
/// | 82         | 1        | Resets again |
///
/// The complete data may be found [here](./relationship_data.txt).
fn part_two(elf_count: u32) -> u32 {
    debug_assert_ne!(0, elf_count);
    if 1 == elf_count { return 1; }

    let pow_above = least_power(3, elf_count);
    let pow_below = pow_above / 3;
    let mid = pow_above - pow_below;

    if elf_count > mid {
        2 * elf_count - pow_above
    } else {
        pow_below - (mid - elf_count)
    }
}

#[deprecated]
#[allow(dead_code)]
/// Returns the elf who will obtain all the presents in part two.
///
/// Used to generate relationship table for cleaner solution.
///
/// This implementation is outstandingly inefficient in both memory
/// and time due to shifting/copying elements during each "elf" removal.
fn part_two_brute_force(elf_count: usize) -> usize {
    debug_assert_ne!(0, elf_count);
    let mut elves: Vec<usize> = (0..elf_count).into_iter().collect();
    let mut finger = 0_usize;

    while elves.len() != 1 {
        let len = elves.len();
        let offset = (finger + (len / 2)) % len;
        elves.remove(offset);
        if offset > finger {
            finger += 1;
        }
        finger %= len - 1;
    }

    *elves.get(0).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            1808357,
            1407007,
            Pz::new(2016, 19)
        )
    }

    #[test]
    fn ex1() {
        assert_eq!(3, part_one(5));
        assert_eq!(5, part_one(10));
        assert_eq!(9, part_one(20));
    }

    #[test]
    fn ex2() {
        assert_eq!(2, part_two(5));
    }

    #[test]
    fn greatest_power_() {
        assert_eq!(4, greatest_power(2, 5));
        assert_eq!(8, greatest_power(2, 15));
        assert_eq!(16, greatest_power(2, 16));
        assert_eq!(16, greatest_power(2, 17));
        assert_eq!(0, greatest_power(2, 0));
        assert_eq!(2, greatest_power(2, 2));
    }

    #[test]
    fn least_power_() {
        assert_eq!(3, least_power(3, 0));
        assert_eq!(3, least_power(3, 2));
        assert_eq!(3, least_power(3, 3));
        assert_eq!(9, least_power(3, 4));
        assert_eq!(27, least_power(3, 15));
        assert_eq!(81, least_power(3, 80));
    }
}
