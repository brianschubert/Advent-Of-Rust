use common::{input as pio, Pt, PuzzleSelection as Pz, Solution};

pub fn solve(puzzle: Pz) -> Solution {
    let input = pio::fetch_string(puzzle).unwrap();

    solve_parts! {
        both => {
            let (end, intersect) = walk_blocks(&input);
            (end, intersect.expect("Instructions never intersect"))
        }
    }
}

fn walk_blocks(instr: &str) -> (i16, Option<i16>) {
    let instructions: Vec<String> = instr
        .split(", ")
        .map(|s: &str| s.to_owned())
        .collect();

    let mut pos = Pt::origin();
    let mut dir: Pt<i8> = Pt::n();

    let mut previous = Vec::new();
    let mut intersect: Option<Pt<i16>> = None;

    for instr in instructions.iter() {
        let (turn, mag) = instr.split_at(1);
        dir = match turn {
            "R" => dir.rot90r(),
            "L" => dir.rot90l(),
            _ => panic!("Bad turn token")
        };

        for _ in 0..mag.parse().unwrap() {
            pos += dir;
            if intersect.is_none() {
                if previous.contains(&pos) {
                    intersect = Some(pos);
                } else {
                    previous.push(pos);
                }
            }
        }
    }

    (
        pos.dist_manh(&Pt::origin()),
        intersect.and_then(|p| Some(p.dist_manh(&Pt::origin())))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution! {
            332,
            166,
            Pz::of(2016,1)
        }
    }

    #[test]
    fn ex1() {
        assert_eq!(5, walk_blocks("R2, L3").0);
        assert_eq!(2, walk_blocks("R2, R2, R2").0);
        assert_eq!(12, walk_blocks("R5, L5, R5, R3").0);
    }

    #[test]
    fn ex2() {
        assert_eq!(4, walk_blocks("R8, R4, R4, R8").1.unwrap())
    }
}
