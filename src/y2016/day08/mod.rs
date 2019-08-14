//! Solution for 2016 Day 08.

use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

mod screen {
    use std::str::FromStr;

    use std::{fmt, str};

    #[derive(Debug)]
    /// A display instruction for a mini screen.
    pub enum ScreenInstruction {
        Rect { x: usize, y: usize },
        RotRow { row: usize, offset: RotOffset },
        RotCol { col: usize, offset: RotOffset },
    }

    /// Pixel rotation offset.
    type RotOffset = u8;

    /// A simulated miniature screen.
    pub struct MiniScreen {
        dim_x: usize,
        dim_y: usize,
        pixels: Vec<u64>,
    }

    impl FromStr for ScreenInstruction {
        type Err = &'static str;

        fn from_str(line: &str) ->  Result<Self, Self::Err> {
            let bytes = line.as_bytes();
            match bytes.len() {
                len @ 8 ..= 10 => { // We caught a rectangle instruction!
                    let div = bytes.iter().enumerate()
                        .find(|&(_, &b)| b == b'x')
                        .map(|pair| pair.0)
                        .ok_or("malformed rect instr")?;
                    let row: usize = str::from_utf8(&bytes[div - 2..div])
                        .unwrap()
                        .trim_start()
                        .parse()
                        .map_err(|_| "invalid rect row")?;
                    let col: usize = str::from_utf8(&bytes[div + 1..len])
                        .unwrap()
                        .trim_start()
                        .parse()
                        .map_err(|_| "invalid rect col")?;
                    Ok(ScreenInstruction::Rect { x: row, y: col })
                }
                len if bytes[7] == b'c' => { // Smells like a column rotation
                    let col: usize = str::from_utf8(&bytes[16..18])
                        .unwrap()
                        .trim_end()
                        .parse()
                        .map_err(|_|"invalid col")?;
                    let offset: RotOffset = str::from_utf8(&bytes[21..len])
                        .unwrap()
                        .trim_start()
                        .parse()
                        .map_err(|_| "invalid col rot offset")?;
                    Ok(ScreenInstruction::RotCol { col, offset })
                }
                len if bytes[7] == b'r' => { // Row rot? Row rot.
                    let row: usize = str::from_utf8(&bytes[13..15])
                        .unwrap()
                        .trim_end()
                        .parse()
                        .map_err(|_| "invalid shift row")?;
                    let offset: RotOffset = str::from_utf8(&bytes[18..len])
                        .unwrap()
                        .trim_start()
                        .parse()
                        .map_err(|_| "invalid row rot offset")?;
                    Ok(ScreenInstruction::RotRow { row, offset })
                }
                _ => Err("Fishy screen instruction!")
            }
        }
    }

    impl MiniScreen {
        /// Builds a new mini screen with the specified dimensions.
        pub fn new(dim_x: usize, dim_y: usize) -> Self {
            debug_assert!(dim_x <= 64);
            MiniScreen {
                dim_x,
                dim_y,
                pixels: vec![0; dim_y],
            }
        }

        /// Returns the total number of lit "pixels" on this screen.
        pub fn pixel_count(&self) -> u32 {
            self.pixels.iter().map(|r| r.count_ones()).sum()
        }

        /// Updates this screen's pixels according to the specified instruction.
        pub fn process_instr(&mut self, instr: &ScreenInstruction) {
            use self::ScreenInstruction as Instr;
            match *instr {
                Instr::Rect { x, y } => self.run_rect(x, y),
                Instr::RotRow { row, offset } => self.run_rot_row(row, offset),
                Instr::RotCol { col, offset } => self.run_rot_col(col, offset),
            }
        }

        fn run_rect(&mut self, x: usize, y: usize) {
            debug_assert!(x <= 64);
            let payload = !make_mask(64 - x as u8);

            for row in 0..y {
                self.pixels[row] |= payload
            }
        }

        fn run_rot_row(&mut self, row: usize, offset: RotOffset) {
            let offset = offset % 64;
            let dim_offset = 64 - self.dim_x as u8;
            let mut bits = self.pixels[row];

            let shifted_off = bits & (make_mask(offset) << dim_offset);

            // Shift row and wipe any bits outside of the screens dimensions
            bits = (bits >> offset) & !make_mask(dim_offset);
            // Insert the bits that we shifted off the right
            bits |= shifted_off << (self.dim_x as u8 - offset);

            self.pixels[row] = bits;
        }

        fn run_rot_col(&mut self, col: usize, offset: RotOffset) {
            debug_assert!(col <= 64);
            let split = self.dim_y - (offset as usize % self.dim_y);
            let col_offset = 63 - col;
            let selector = 1_u64 << col_offset;

            // Get the state of the pixels in the column
            let mut col_states: Vec<bool> = self.pixels.iter()
                .map(|r| (r & selector) != 0).collect();

            // Rotate the pixels
            col_states[split..].reverse();
            col_states[..split].reverse();
            col_states.reverse();

            // Write new states to the pixels
            for (i, row) in self.pixels.iter_mut().enumerate() {
                *row = (*row & !selector) | ((col_states[i] as u64) << col_offset);
            }
        }
    }

    impl fmt::Debug for MiniScreen {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("MiniScreen")
                .field("dim_x", &self.dim_x)
                .field("dim_y", &self.dim_y)
                .field("pixels", &self.pixels.iter()
                    .map(|r| format!("{:064b}", r))
                    .collect::<Vec<String>>())
                .finish()
        }
    }

    impl fmt::Display for MiniScreen {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "")?;
            for row in self.pixels.iter() {
                writeln!(
                    f, "\"{}\"",
                    &format!("{:064b}", row)[0..self.dim_x]
                        .replace('0', " ")
                        .replace('1', "#")
                )?;
            }
            write!(f, "")
        }
    }

    #[inline]
    fn make_mask(set_count: u8) -> u64 {
        (0..set_count).fold(0_u64, |acc, i| acc | 1 << i)
    }
}

use self::screen::*;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input: Vec<_> = pio::fetch_lines(puzzle)?
        .into_iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let mut screen = MiniScreen::new(50, 6);

    solve_parts! {
        1 => {
            for instr in &input {
                screen.process_instr(instr);
            }
            screen.pixel_count()
        },
        2 => format!("{}", screen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_TWO_SOLUTION: &'static str = r######"
" ##  #### ###  #  # ###  #### ###    ## ###   ### "
"#  # #    #  # #  # #  #    # #  #    # #  # #    "
"#  # ###  ###  #  # #  #   #  ###     # #  # #    "
"#### #    #  # #  # ###   #   #  #    # ###   ##  "
"#  # #    #  # #  # #    #    #  # #  # #       # "
"#  # #    ###   ##  #    #### ###   ##  #    ###  "
"######;

    #[test]
    fn solution() {
        assert_solution! {
            123,
            PART_TWO_SOLUTION,
            Pz::new(2016, 8)
        };
    }

    #[test]
    fn ex_both() {
        let input = [
            "rect 3x2",
            "rotate column x=1 by 1",
            "rotate row y=0 by 4",
            "rotate column x=1 by 1"
        ];

        let mut screen = MiniScreen::new(7, 3);

        for instr in input.iter().map(|l| l.parse().unwrap()) {
            screen.process_instr(&instr);
        }

        assert_eq!(6, screen.pixel_count());
        assert_eq!(
            "\n\" #  # #\"\n\"# #    \"\n\" #     \"\n",
            format!("{}", screen)
        );
    }

    #[test]
    fn parse_screen_instr() {
        use self::ScreenInstruction as SI;

        if let Ok(SI::Rect { x, y }) = "rect 50x11".parse() {
            assert_eq!(50, x);
            assert_eq!(11, y);
        } else {
            panic!("Wrong instruction: expected Rect")
        }
        if let Ok(SI::RotCol { col, offset }) = "rotate column x=1 by 30".parse() {
            assert_eq!(1, col);
            assert_eq!(30, offset);
        } else {
            panic!("Wrong instruction: expected RotCol")
        }
        if let Ok(SI::RotRow { row, offset }) = "rotate row y=40 by 42".parse() {
            assert_eq!(40, row);
            assert_eq!(42, offset);
        } else {
            panic!("Wrong instruction: expected RotRow")
        }
    }
}
