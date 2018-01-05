use common::{input as pio, PuzzleSelection as Pz, Pt};

const KEYPAD_ONE: [&'static [char]; 3] = [
    &['1', '2', '3'],
    &['4', '5', '6'],
    &['7', '8', '9'],
];

const START_ONE: Pt<i8> = Pt { x: 1, y: 1};

const KEYPAD_TWO: [&'static [char]; 5] = [
    &['x', 'x', '1', 'x', 'x'],
    &['x', '2', '3', '4', 'x'],
    &['5', '6', '7', '8', '9'],
    &['x', 'A', 'B', 'C', 'x'],
    &['x', 'x', 'D', 'x', 'x'],
];

const START_TWO: Pt<i8> = Pt { x: 0, y: 2 };

pub fn solve(puzzle: Pz) {
    let input = pio::fetch_lines(puzzle).unwrap();

    println!("Part 1: {}", press_keycode(&KEYPAD_ONE, &input, START_ONE));
    println!("Part 2: {}", press_keycode(&KEYPAD_TWO, &input, START_TWO));
}

fn press_keycode(keypad: &[&[char]], instr: &[String], start: Pt<i8>) -> String {
    // keypad assumed to be a square
    let dim = keypad.len() - 1;
    let mut finger = start.clone();

    instr.iter().map(|line| {
        for byte in line.as_bytes() {
            let next = finger + match *byte as char {
                'U' => Pt::<i8>::n(),
                'R' => Pt::e(),
                'D' => Pt::s(),
                'L' => Pt::w(),
                _ => panic!(format!("Bad direction: {}", &byte))
            };
            {
                let (x, y) = next.parts();
                if (x >= 0 && x <= dim as i8)
                        && (y >= 0 && y <= dim as i8)
                        && keypad[dim - y as usize][x as usize] != 'x' {
                    finger = next;
                }
            }
        }
        keypad[dim - finger.y as usize][finger.x as usize]
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        let instr = pio::fetch_lines(Pz::of(2016, 2)).unwrap();
        assert_eq!("99332", press_keycode(&KEYPAD_ONE, &instr, START_ONE));
        assert_eq!("DD483", press_keycode(&KEYPAD_TWO, &instr, START_TWO));
    }

    #[test]
    fn ex1() {
        let instr = [
            "ULL".to_owned(),
            "RRDDD".to_owned(),
            "LURDL".to_owned(),
            "UUUUD".to_owned()
        ];

        assert_eq!("1985", press_keycode(&KEYPAD_ONE, &instr, START_TWO));
    }

    #[test]
    fn ex2() {
        let instr = [
            "ULL".to_owned(),
            "RRDDD".to_owned(),
            "LURDL".to_owned(),
            "UUUUD".to_owned()
        ];

        assert_eq!("5DB3", press_keycode(&KEYPAD_TWO, &instr, START_TWO));
    }
}

