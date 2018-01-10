use std::collections::HashMap;
use common::{input as pio, PuzzleSelection as Pz, Solution};

pub fn solve(puzzle: Pz) -> Solution {
    let input: Vec<_> = pio::fetch_lines(puzzle)
        .expect("input file could not be read");

    solve_parts! {
        both => repetition_correct(&input[..])
    }
}

/// "Error corrects" the specified transmission by locating the most frequent
/// (part one) and least frequent (part two) character at each message position.
fn repetition_correct(transmissions: &[String]) -> (String, String) {
    let msg_width = transmissions.first().unwrap().len();
    let mut msg_one: Vec<u8> = Vec::with_capacity(msg_width);
    let mut msg_two: Vec<u8> = Vec::with_capacity(msg_width);
    let mut freq_buf = HashMap::new();

    for col in 0..msg_width {
        for letter in transmissions.iter().map(|line| line.as_bytes()[col]) {
            *freq_buf.entry(letter).or_insert(0u16) += 1
        }
        let mut elems: Vec<(u8, u16)> = freq_buf.drain().collect();
        elems.sort_by(|a, b| b.1.cmp(&a.1));

        msg_one.push(elems.first().unwrap().0);
        msg_two.push(elems.last().unwrap().0);
    }
    (
        String::from_utf8(msg_one).expect("message contains invalid uft8"),
        String::from_utf8(msg_two).expect("message contains invalid uft8")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            "afwlyyyq",
            "bhkzekao",
            Pz::of(2016, 6)
        )
    }

    #[test]
    fn ex_both() {
        let transmission = [
            "eedadn".to_owned(),
            "drvtee".to_owned(),
            "eandsr".to_owned(),
            "raavrd".to_owned(),
            "atevrs".to_owned(),
            "tsrnev".to_owned(),
            "sdttsa".to_owned(),
            "rasrtv".to_owned(),
            "nssdts".to_owned(),
            "ntnada".to_owned(),
            "svetve".to_owned(),
            "tesnvt".to_owned(),
            "vntsnd".to_owned(),
            "vrdear".to_owned(),
            "dvrsen".to_owned(),
            "enarar".to_owned()
        ];

        let (one, two) = repetition_correct(&transmission);
        assert_eq!("easter", one);
        assert_eq!("advent", two);
    }
}
