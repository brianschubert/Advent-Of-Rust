//! Solution for Advent of Code [2018 Day 02](https://adventofcode.com/2018/day/2).

use crate::common::puzzle::{input as pio, Result as PuzzleResult, Selection as Pz};

// The ID of a warehouse box;
#[derive(Debug, Clone, PartialEq, Eq)]
struct BoxId(String);

impl BoxId {
    /// Check for repeated letters in this BoxId and return a tuple
    /// containing 1) whether any letter appears exactly twice and 2)
    /// whether any letter appears exactly thrice.
    ///
    /// This function will panic if the BoxId contains anything but lowercase
    /// ascii letters.
    fn check_repeats(&self) -> (bool, bool) {
        let mut count_cells = [0; 26];
        for &byte in self.0.as_bytes().iter() {
            if !(byte >= b'a' && byte <= b'z') {
                panic!("BoxId letter MUST BE a valid lowercase ascci letter");
            }
            count_cells[(byte - b'a') as usize] += 1;
        }

        let mut two_repeats = false;
        let mut three_repeats = false;
        for &letter_count in count_cells.iter() {
            if letter_count == 2 {
                two_repeats = true;
            } else if letter_count == 3 {
                three_repeats = true;
            }
        }
        (two_repeats, three_repeats)
    }

    /// Return the index at which this this BoxId and the other BoxId differ,
    /// assuming they differ at *exactly* one position, otherwise ``None``.
    fn find_differing_position(&self, other: &Self) -> Option<usize> {
        if self.0.len() != other.0.len() {
            return None;
        }

        // Create a Vec the indices at which the two IDs differ
        let differing_position: Vec<usize> = self.0.as_bytes()
            .iter()
            .zip(other.0.as_bytes())
            .enumerate()
            .filter(|(_, (&left, &right))| left != right)
            .map(|(index, _)| index)
            .collect();

        if differing_position.len() == 1 {
            Some(differing_position[0])
        } else {
            None
        }
    }
}

impl ToString for BoxId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input: Vec<_> = pio::fetch_lines(puzzle)?.into_iter().map(BoxId).collect();
    solve_parts!(
        1 => compute_box_list_checksum(&input),
        2 => {
            let (similar_pair, differing_pos) = find_similar_box_ids(&input)
                .ok_or("input contains no IDs that differ at exactly one position")?;
            let mut matching_part = similar_pair.0.to_string();
            matching_part.remove(differing_pos);
            matching_part
        }
    )
}

/// Count the number of BoxIDs with exactly two repeated digits and the number
/// of BoxIDs with exactly three repeated digits. Return the product of these
/// two numbers.
fn compute_box_list_checksum(box_ids: &[BoxId]) -> u32 {
    let (two_count, three_count) = box_ids
        .iter()
        .map(BoxId::check_repeats)
        .map(|(two, three)| (two as u32, three as u32))
        .fold((0, 0), |acc, value| (acc.0 + value.0, acc.1 + value.1));
    two_count * three_count
}

/// Search the list of BoxIDs for two IDs that differ at exactly one position.
/// If such a pair of IDs is found, return a tuple containing 1) the pair of
/// similar IDS, and 2) the position at which the two IDs differ.
fn find_similar_box_ids(box_ids: &[BoxId]) -> Option<((&BoxId, &BoxId), usize)> {
    for (index, finger) in box_ids.iter().enumerate() {
        // Search all BoxID that come after the finger for similarity to the finger.
        if let Some((similar_id, differing_pos)) = box_ids[index + 1..].iter()
            // Filter the sequence of BoxIDs by whether an ID is similar to the finger.
            // Map each similar ID into a tuple of 1) the ID and 2) the position
            // at which the ID differs from the finger.
            .filter_map(|box_id|
                finger
                    .find_differing_position(box_id)
                    .map(|pos| (box_id, pos))
            ).next() {
            return Some(((finger, similar_id), differing_pos));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(6422, "qcslyvphgkrmdawljuefotxbh", Pz::new(2018, 2))
    }

    #[test]
    fn ex1() {
        let test_cases = [
            (BoxId("abcdef".to_owned()), (false, false)),
            (BoxId("bababc".to_owned()), (true, true)),
            (BoxId("abbcde".to_owned()), (true, false)),
            (BoxId("abcccd".to_owned()), (false, true)),
            (BoxId("aabcdd".to_owned()), (true, false)),
            (BoxId("abcdee".to_owned()), (true, false)),
            (BoxId("ababab".to_owned()), (false, true)),
        ];
        for (input, expected) in test_cases.iter() {
            assert_eq!(input.check_repeats(), *expected)
        }
        let box_ids: Vec<_> = test_cases.into_iter()
            .map(|(box_id, _)| box_id.clone())
            .collect();
        assert_eq!(compute_box_list_checksum(&box_ids), 12,)
    }

    #[test]
    fn find_differing_position() {
        let box_id = BoxId("abcde".to_owned());
        assert!(
            box_id.find_differing_position(&BoxId("abcde".to_owned())).is_none()
        );
        assert!(
            box_id.find_differing_position(&BoxId("ABcde".to_owned())).is_none()
        );
        assert_eq!(
            box_id.find_differing_position(&BoxId("Abcde".to_owned())),
            Some(0),
        );
        assert_eq!(
            box_id.find_differing_position(&BoxId("aBcde".to_owned())),
            Some(1),
        );
        assert_eq!(
            box_id.find_differing_position(&BoxId("abcdE".to_owned())),
            Some(4),
        );
    }

    #[test]
    fn ex2() {
        let box_ids = [
            BoxId("abcde".to_owned()),
            BoxId("fghij".to_owned()),
            BoxId("klmno".to_owned()),
            BoxId("pqrst".to_owned()),
            BoxId("fguij".to_owned()),
            BoxId("axcye".to_owned()),
            BoxId("vxyz]".to_owned()),
        ];

        let (similar_pair, differing_pos) = find_similar_box_ids(&box_ids).unwrap();
        assert_eq!(*similar_pair.0, box_ids[1]);
        assert_eq!(*similar_pair.1, box_ids[4]);

        let mut matching_parts = similar_pair.0.to_string();
        matching_parts.remove(differing_pos);
        assert_eq!(&matching_parts, "fgij")
    }
}
