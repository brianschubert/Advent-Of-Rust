//! Solution for 2016 Day 04.

use std::collections::BTreeMap; // Orders chars alphabetically
use crate::common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

/// The decrypted room name to be searched for during part two.
const PART_TWO_NEEDLE: &str = "northpole object storage";

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
    fn parse_str(line: &'a str) -> Result<Self, &'static str> {
        let (name, sec_and_check) = line.split_at(line.len() - 11);
        let mut name_freq = BTreeMap::new();

        for letter in name.chars() {
            if letter == '-' { continue }
            *name_freq.entry(letter).or_insert(0u8) += 1
        }

        Ok(RoomListing {
            name,
            expected_checksum: &sec_and_check[5..10],
            sector: sec_and_check[1..4].parse().map_err(|_|"malformed sector")?,
            name_freq,
        })
    }

    /// Returns this room's decrypted name. Calculated on each call.
    fn decrypted_name(&self) -> String {
        self.name.bytes().map(|l| {
            if l == b'-' { return ' '; }
            (((u16::from(l - b'a') + self.sector) % 26) as u8 + b'a') as char
        }).collect()
    }
}

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_lines(puzzle)?;

    let rooms = parse_input(&input[..])?;

    solve_parts! {
        1 => part_one(&rooms),
        2 => part_two(&rooms, PART_TWO_NEEDLE)
            .ok_or(format!("no room found with name `{}`", PART_TWO_NEEDLE))?
    }
}

fn parse_input<'a, S: AsRef<str>>(
    lines: &'a [S]
) -> Result<Vec<RoomListing<'a>>, &'static str> {
    lines
        .iter()
        .map(|line| RoomListing::parse_str(line.as_ref()))
        .collect()
}

/// Returns the sum of the sector ids of the specified rooms that are
/// real.
fn part_one(rooms: &[RoomListing]) -> u32 {
    rooms
        .iter()
        .filter(|&r| r.is_real())
        .map(|room| u32::from(room.sector))
        .sum()
}

/// Returns the sector id of the room whose decrypted name matches the
/// specified needle or None if no such room is found.
fn part_two(rooms: &[RoomListing], needle: &str) -> Option<u16> {
    let decrypted: Vec<(String, u16)> = rooms.iter().map(|room| {
        (room.decrypted_name(), room.sector)
    }).collect();

    decrypted
        .iter()
        .find(|n| n.0 == needle)
        .map(|pair| pair.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            409_147,
            991,
            Pz::new(2016, 4)
        )
    }

    #[test]
    fn ex1() {
        let rooms = [
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
        ];

        let sol = part_one(&parse_input(&rooms[..]).unwrap());

        assert_eq!(1514, sol);
    }

    #[test]
    fn ex2() {
        let room = RoomListing::parse_str("qzmt-zixmtkozy-ivhz-343[abcde]").unwrap();
        assert_eq!("very encrypted name", room.decrypted_name());
    }

    #[test]
    fn parse_room_listing() {
        let room = RoomListing::parse_str("aaaaa-bbb-z-y-x-123[abxyz]")
            .expect("failed to parse room listing");
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
        let real = RoomListing::parse_str("aaaaa-bbb-z-y-x-123[abxyz]")
            .expect("failed to parse real room");
        let decoy = RoomListing::parse_str("totally-real-room-200[decoy]")
            .expect("failed to parse decoy room");

        assert!(real.is_real());
        assert!(!decoy.is_real());
    }
}
