use common::{input as pio, Pt, PuzzleSelection as Pz};

pub fn solve(puzzle: Pz) {
    let input = pio::fetch_string(puzzle).unwrap();

    println!("Part 1: {}", part1(&input)); // 332

    println!("Part 2: {}", part2(&input)); // 166
}

fn part1(input: &str) -> isize {
    let instructions: Vec<String> = input
        .split(", ")
        .map(|s: &str| s.to_owned())
        .collect();

    let mut pos = Pt::origin();
    let mut dir = Pt::n();

    for instr in instructions.iter() {
        let (turn, mag) = instr.split_at(1);
        dir = match turn {
            "R" => dir.rot90r(),
            "L" => dir.rot90l(),
            _ => panic!("Bad turn token")
        };

        for _ in 0..mag.parse().unwrap() {
            pos += dir;
        }
    }

    pos.dist_manh(&Pt::origin())
}

fn part2(input: &str) -> isize {
    let instructions: Vec<String> = input.split(", ").map(|s: &str| s.to_owned()).collect();

    let mut pos = Pt::origin();
    let mut dir = Pt::n();

    let mut previous = Vec::new();

    for instr in instructions.iter() {
        let (turn, mag) = instr.split_at(1);
        dir = match turn {
            "R" => dir.rot90r(),
            "L" => dir.rot90l(),
            _ => panic!("Bad turn token")
        };

        for _ in 0..mag.parse().unwrap() {
            pos += dir;
            if previous.contains(&pos) {
                return pos.dist_manh(&Pt::origin());
            }
            previous.push(pos)
        }
    }
    panic!("No recreated location found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        let input = pio::fetch_string(Pz::of(2016, 1)).unwrap();
        assert_eq!(332, part1(&input));
        assert_eq!(166, part2(&input));
    }

    #[test]
    fn ex1() {
        assert_eq!(5, part1("R2, L3"));
        assert_eq!(2, part1("R2, R2, R2"));
        assert_eq!(12, part1("R5, L5, R5, R3"));
    }

    #[test]
    fn ex2() {
        assert_eq!(4, part2("R8, R4, R4, R8"))
    }
}
