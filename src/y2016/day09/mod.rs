//! Solution for 2016 Day 09.

use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_string(puzzle)
        .expect("input file could not be read");

    solve_parts! {
        1 => decompress(&input[..]).len(),
        2 => decompressed_len(&input[..])
    }
}

/// Decompresses a string according to its repetition markers.
fn decompress(input: &str) -> String {
    let input_len = input.len();
    let mut buf_out = String::new();
    let mut pos = 0_usize;

    while pos < input_len {
        let remaining = &input[pos..];

        if let (Some(marker_s), Some(marker_e)) = (
            remaining.find('('),
            remaining.find(')')
        ) {
            if marker_s != 0 {
                buf_out.push_str(&remaining[..marker_s])
            }

            let marker = &remaining[marker_s + 1..marker_e];
            let parts = marker.split_at(
                marker.find('x').expect("malformed marker")
            );

            let payload_range: usize = parts.0.parse()
                .expect("malformed marker range");

            let rep_count: u8 = (&parts.1[1..]).parse()
                .expect("malformed marker repetition");

            let payload = &remaining[marker_e + 1..marker_e + 1 + payload_range];

            for _ in 0..rep_count {
                buf_out.push_str(payload);
            }

            pos += marker_e + payload_range + 1;
        } else {
            buf_out.push_str(remaining);
            pos = input_len;
        }
    }
    buf_out
}

/// Calculates the length of a string one it has been recursively decompressed
/// according to its repetition markers.
///
/// Decompression is *not* preformed.
fn decompressed_len(input: &str) -> usize {
    let input_len = input.len();
    let mut out_len = 0_usize;
    let mut pos = 0_usize;

    while pos < input_len {
        let remaining = &input[pos..];

        if let (Some(marker_s), Some(marker_e)) = (
            remaining.find('('),
            remaining.find(')')
        ) {
            out_len += marker_s;

            let marker = &remaining[marker_s + 1..marker_e];
            let parts = marker.split_at(
                marker.find('x').expect("malformed marker")
            );

            let payload_range: usize = parts.0.parse()
                .expect("malformed marker range");

            let rep_count: u8 = (&parts.1[1..]).parse()
                .expect("malformed marker repetition");

            let payload = &remaining[marker_e + 1..marker_e + 1 + payload_range];

            out_len += decompressed_len(payload) * rep_count as usize;

            pos += marker_e + payload_range + 1;
        } else {
            out_len += remaining.len();
            pos = input_len;
        }
    }
    out_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            74_532_usize,
            11_558_231_665_usize,
            Pz::new(2016, 9)
        )
    }

    #[test]
    fn ex1() {
        assert_eq!("ADVENT", decompress("ADVENT"));
        assert_eq!("ABBBBBC", decompress("A(1x5)BC"));
        assert_eq!("XYZXYZXYZ", decompress("(3x3)XYZ"));
        assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG"));
        assert_eq!("(1x3)A", decompress("(6x1)(1x3)A"));
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY"));
    }


    #[test]
    fn ex2() {
        assert_eq!(9, decompressed_len("(3x3)XYZ"));
        assert_eq!(20, decompressed_len("X(8x2)(3x3)ABCY"));
        assert_eq!(241_920, decompressed_len(
            "(27x12)(20x12)(13x14)(7x10)(1x12)A"
        ));
        assert_eq!(445, decompressed_len(
            "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"
        ));
    }
}
