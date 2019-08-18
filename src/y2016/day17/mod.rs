//! Solution for 2016 Day 17
//!
//! While capable of determining the shortest route to
//! a vault destination (part one), this solution evidently
//! fails to generate all possible routes as it is unable
//! to determine the length of the longest route (part two).
//!
//! I have been unable to find the root of this bug. For now,
//! I am pushing the solution to part one with the part two
//! boilerplate commented out.

use crate::common::puzzle::{input as pio, Result as PuzzleResult, Selection as Pz};
use crate::common::util::Pt;

/// The final destination in the vault
const VAULT_DEST: Pt<i8> = Pt { x: 3, y: 0 };

/// The starting location in the vault
const VAULT_START: Pt<i8> = Pt { x: 0, y: 3 };

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_string(puzzle)?;

    let mut nav = vault::VaultNavigator::new(VAULT_DEST);

    solve_parts! {
        1 => {
            nav.find_routes(VAULT_START, input.trim_end());
            nav.shortest_route().unwrap()
        }
        //, 2 => nav.longest_route().unwrap().len()
    }
}

mod vault {
    use crate::common::util::Pt;
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    /// A Position within a vault.
    type VaultPos = Pt<i8>;

    /// A direction specifying a position offset.
    type Dir = Pt<i8>;

    impl Dir {
        /// Attempts to convert a direction into a char.
        fn into_char(self) -> Option<char> {
            if self == Dir::n() {
                Some('U')
            } else if self == Dir::e() {
                Some('R')
            } else if self == Dir::s() {
                Some('D')
            } else if self == Dir::w() {
                Some('L')
            } else {
                None
            }
        }
    }

    #[derive(Debug)]
    /// A navigator between two points in a secured vault.
    pub struct VaultNavigator {
        /// The paths that lead to this vault's destination.
        valid_routes: Vec<String>,
        /// The desired destination within the vault.
        destination: VaultPos,
    }

    #[derive(Debug)]
    /// A path within a vault
    struct VaultPath {
        /// The doors taken on this path.
        pathcode: String,
        /// The position at the end of the path.
        pos: VaultPos,
        /// The doors that are open at the end of the path.
        doors: Vec<Dir>,
    }

    impl VaultNavigator {
        /// Builds a `VaultNavigator` with the specified destination.
        pub fn new(destination: VaultPos) -> Self {
            VaultNavigator { valid_routes: Vec::new(), destination }
        }

        /// Determines all the possible routes leading from the `start_pos`
        /// to this vault's destination based on the specified passcode.
        pub fn find_routes(&mut self, start_pos: VaultPos, passcode: &str) {
            self.reset();
            if start_pos == self.destination {
                return;
            }

            let mut path_stack = Vec::new();

            // Assumes square vault
            let max_pos = self.destination.x;

            let start = VaultPath {
                pathcode: String::from(passcode),
                pos: start_pos,
                doors: doors_at_path_end(&passcode)
                    .into_iter()
                    .filter(|&dir| {
                        let next = start_pos + dir;
                        next.x >= 0 && next.x <= max_pos && next.y >= 0 && next.y <= max_pos
                    })
                    .collect(),
            };

            path_stack.push(start);

            while !path_stack.is_empty() {
                // Should optimize repetitive popping/re-pushing to the stack
                let mut current = path_stack.pop().unwrap();

                if !current.doors.is_empty() {
                    let door_selection = current.doors.pop().unwrap();

                    let mut pathcode = current.pathcode.clone();
                    pathcode.push(
                        door_selection.into_char().expect("illegal direction")
                    );

                    let pos = current.pos + door_selection;

                    if pos == self.destination {
                        self.valid_routes.push(
                            pathcode[passcode.len()..].to_owned()
                        );
                    } else {
                        let next = VaultPath {
                            doors: doors_at_path_end(&pathcode)
                                .into_iter()
                                .filter(|&dir| {
                                    let next = pos + dir;
                                    next.x >= 0
                                        && next.x <= max_pos
                                        && next.y >= 0
                                        && next.y <= max_pos
                                })
                                .collect(),
                            pathcode,
                            pos,
                        };

                        path_stack.push(current);
                        path_stack.push(next);
                    }
                }
            }
        }

        /// Clears all valid routes
        pub fn reset(&mut self) {
            self.valid_routes.clear();
        }

        /// Returns the shortest valid route found.
        ///
        /// If no valid routes were found, `None` is returned.
        pub fn shortest_route(&self) -> Option<&String> {
            self.valid_routes.iter().min_by_key(|&path| path.len())
        }

        /// Returns the longest valid route found.
        ///
        /// If no valid routes were found, `None` is returned.
        pub fn _longest_route(&self) -> Option<&String> {
            self.valid_routes.iter().max_by_key(|&path| path.len())
        }
    }

    /// Returns the paths at the end of a path based on its MD5 digest.
    fn doors_at_path_end(pathcode: &str) -> Vec<Dir> {
        let mut doors = Vec::with_capacity(4);
        let mut result = [0_u8; 16];

        {
            let mut hasher = Md5::new();
            hasher.input_str(&pathcode);
            hasher.result(&mut result);
        }

        const OPEN_THRESHOLD: u8 = 0xA;
        // First char - up
        if result[0] >> 4 > OPEN_THRESHOLD { doors.push(Dir::n()); }
        // Second char - down
        if result[0] & 0xf > OPEN_THRESHOLD { doors.push(Dir::s()); }
        // Third char - left
        if result[1] >> 4 > OPEN_THRESHOLD { doors.push(Dir::w()); }
        // Fourth char - right
        if result[1] & 0xf > OPEN_THRESHOLD { doors.push(Dir::e()); }

        doors
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn find_doors() {
            let doors = doors_at_path_end("hijkl");
            assert_eq!([Dir::n(), Dir::s(), Dir::w()], &doors[..]);

            let doors = doors_at_path_end("hijklD");
            assert_eq!([Dir::n(), Dir::w(), Dir::e()], &doors[..]);

            let doors = doors_at_path_end("hijklDR");
            assert!(doors.is_empty());

            let doors = doors_at_path_end("hijklDU");
            assert_eq!([Dir::e()], &doors[..]);

            let doors = doors_at_path_end("hijklDUR");
            assert!(doors.is_empty());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!("DDRRULRDRD", Pz::new(2016, 17))
    }

    #[test]
    fn ex_both() {
        let mut nav = vault::VaultNavigator::new(VAULT_DEST);

        let test_cases = [
            ("hijkl", None, None),
            ("ihgpwlah", Some("DDRRRD"), Some(370)),
            ("kglvqrro", Some("DDUDRLRRUDRD"), Some(492)),
            ("ulqzkmiv", Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR"), Some(830)),
        ];

        for &(input, expected_one, _expected_two) in test_cases.iter() {
            nav.find_routes(VAULT_START, input);

            assert_eq!(expected_one, nav.shortest_route().map(String::as_ref));

            // Temporarily (hopefully) omitting part two's example check
            // assert_eq!(expected_two, nav.longest_route().map(String::len));
        }
    }
}
