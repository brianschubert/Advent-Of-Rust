//! Solution for 2016 Day 04.

use std::collections::BTreeMap; // Orders chars alphabetically
use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

const PART_TWO_NEEDLE: &'static str = "northpole object storage";

#[derive(Debug)]
struct RoomListing<'a> {
    name: &'a str,
    expected_checksum: &'a str,
    sector: u16,
    name_freq: BTreeMap<char, u8>,
}

impl<'a> RoomListing<'a> {
    /// Returns true if the room matches its checksum.
    fn is_real(&self) -> bool {
        let mut elems: Vec<(&char, &u8)> = self.name_freq.iter().collect();
        elems.sort_by(|a, b| b.1.cmp(a.1));
        let check: String = elems.iter().take(5).map(|e| e.0).collect();
        &check[..] == self.expected_checksum
    }

    /// Builds a room from a string slice.
    ///
    /// Panics if the room string contains a malformed sector.
    fn parse_unstable(line: &'a str) -> Self {
        let (name, sec_and_check) = line.split_at(line.len() - 11);
        let mut name_freq = BTreeMap::new();

        for letter in name.chars() {
            if letter == '-' { continue }
            *name_freq.entry(letter).or_insert(0u8) += 1
        }

        RoomListing {
            name,
            expected_checksum: &sec_and_check[5..10],
            sector: sec_and_check[1..4].parse().expect("malformed sector"),
            name_freq,
        }
    }

    /// Returns this room's decrypted name. Calculated on each call.
    fn decrypted_name(&self) -> String {
        self.name.bytes().map(|l| {
            if l == b'-' { return ' '; }
            (((((l - b'a') as u16) + self.sector) % 26) as u8 + b'a') as char
        }).collect()
    }
}

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_lines(puzzle).expect("input file could not be read");

    let rooms = parse_input(&input[..]);

    solve_parts! {
        1 => part_one(&rooms),
        2 => part_two(&rooms)
    }
}

fn parse_input<'a>(lines: &'a [String]) -> Vec<RoomListing<'a>> {
    lines
        .iter()
        .map(|line| RoomListing::parse_unstable(&line[..]))
        .filter(|room| room.is_real())
        .collect()
}

fn part_one(rooms: &[RoomListing]) -> u32 {
    rooms.iter().map(|room| room.sector as u32).sum()
}

fn part_two(rooms: &[RoomListing]) -> u16 {
    let decrypted: Vec<(String, u16)> = rooms.iter().map(|room| {
        (room.decrypted_name(), room.sector)
    }).collect();

    decrypted
        .iter()
        .find(|n| n.0 == PART_TWO_NEEDLE)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            409147,
            991,
            Pz::new(2016, 4)
        )
    }

    #[test]
    fn ex1() {
        let rooms = [
            "aaaaa-bbb-z-y-x-123[abxyz]".to_owned(),
            "a-b-c-d-e-f-g-h-987[abcde]".to_owned(),
            "not-a-real-room-404[oarel]".to_owned(),
            "totally-real-room-200[decoy]".to_owned()
        ];

        let sol = part_one(&parse_input(&rooms[..]));

        assert_eq!(1514, sol);
    }

    #[test]
    fn ex2() {
        let room = RoomListing::parse_unstable("qzmt-zixmtkozy-ivhz-343[abcde]");
        assert_eq!("very encrypted name", room.decrypted_name());
    }

    #[test]
    fn parse_room_listing() {
        let room = RoomListing::parse_unstable("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(123, room.sector);
        assert_eq!("aaaaa-bbb-z-y-x", room.name);
        assert_eq!("abxyz", room.expected_checksum);
        assert_eq!(5, room.name_freq.len());
        assert_eq!(5, *room.name_freq.get(&'a').unwrap());
        assert_eq!(3, *room.name_freq.get(&'b').unwrap());
        assert_eq!(1, *room.name_freq.get(&'y').unwrap());
        assert_eq!(1, *room.name_freq.get(&'x').unwrap());
        assert_eq!(1, *room.name_freq.get(&'x').unwrap());
    }

    #[test]
    fn room_is_real() {
        let real =
            RoomListing::parse_unstable("aaaaa-bbb-z-y-x-123[abxyz]");
        let decoy =
            RoomListing::parse_unstable("totally-real-room-200[decoy]");

        assert!(real.is_real());
        assert!(!decoy.is_real());
    }
}
