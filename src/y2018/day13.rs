//! Solution for Advent of Code [2018 Day 13](https://adventofcode.com/2018/day/13).

use crate::common::puzzle;

mod cart {
    use crate::common::util::{Grid, IntoPoint, Pt};
    use std::ops::Index;
    use std::str::FromStr;

    /// Signed integer type used to represent the components of a point.
    type PointScalar = i16;
    /// Point type used to represent coordinates on the cart track.
    pub type Point = Pt<i16>;
    /// Integer type used to represent ticks of time.
    pub type TimeTick = u32;

    #[derive(Copy, Clone, Debug)]
    /// A way that a chart may choose to turn at an intersection.
    enum Turn {
        Left,
        Right,
        Straight,
    }

    #[derive(Copy, Clone, Debug)]
    /// Cardinal directions in 2D space.
    enum Direction {
        North,
        East,
        South,
        West,
    }

    #[derive(Copy, Clone, Debug)]
    /// The map tiles that comprise a cart map.
    enum MapTile {
        // "space"
        Blank,
        // "/"
        ConnectEast,
        // "\"
        ConnectWest,
        // "-" or "|" or an initial cart position
        Straight,
        // "+"
        Intersection,
    }

    #[derive(Debug)]
    /// A cart positioned on a cart map.
    struct Cart {
        turn_preference: Turn,
        pos: Point,
        facing: Direction,
        collided: bool,
    }

    /// A grid in 2D space containing paths along which carts travel.
    struct CartTrack {
        grid: Grid<i16>,
        path: Vec<Vec<MapTile>>,
    }

    /// A simulator for predicting the behavior of a sequence of carts on a
    /// cart map.
    pub struct TrackSimulator {
        track: CartTrack,
        carts: Vec<Cart>,
        tick: TimeTick,
    }

    impl Direction {
        fn as_pt(&self) -> Point {
            // Note: "North" (i.e. up) in the input map is associated with a
            // decreasing y coordinate, so "north" and "south" displacements
            // are swapped to maintain consistency with the puzzle's conventions.
            match *self {
                Direction::North => Point::s(),
                Direction::East => Point::e(),
                Direction::South => Point::n(),
                Direction::West => Point::w(),
            }
        }
    }

    impl Cart {
        /// Cycles this carts turning preferences for intersections according
        /// to the rules specified in the puzzle.
        fn cycle_turn_preference(&mut self) {
            self.turn_preference = match self.turn_preference {
                Turn::Left => Turn::Straight,
                Turn::Straight => Turn::Right,
                Turn::Right => Turn::Left,
            }
        }

        /// Advances this carts position by one tile in the direction that
        /// it is facing.
        fn advance(&mut self) {
            self.pos += self.facing.as_pt();
        }

        /// Updates the direction that this cart is facing according to the
        /// provided tile that it just advanced to.
        ///
        /// The cart's turn preference will also be updated.
        fn apply_turn(&mut self, tile: MapTile) {
            use Direction::*;
            use Turn::*;

            self.facing = match tile {
                MapTile::ConnectWest => match self.facing {
                    North => West,
                    East => South,
                    West => North,
                    South => East,
                },
                MapTile::ConnectEast => match self.facing {
                    South => West,
                    West => South,
                    North => East,
                    East => North,
                },
                MapTile::Intersection => {
                    let turn_preference = self.turn_preference;
                    self.cycle_turn_preference();
                    match (self.facing, turn_preference) {
                        (_, Turn::Straight) => self.facing,
                        (South, Left) | (North, Right) => East,
                        (South, Right) | (North, Left) => West,
                        (East, Left) | (West, Right) => North,
                        (East, Right) | (West, Left) => South,
                    }
                }
                _ => self.facing,
            }
        }
    }

    impl FromStr for CartTrack {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let path: Vec<Vec<MapTile>> = s
                .lines()
                .map(|row_data| {
                    row_data
                        .bytes()
                        .map(|tile| match tile {
                            b' ' => Ok(MapTile::Blank),
                            b'-' | b'|' | b'<' | b'^' | b'>' | b'v' => Ok(MapTile::Straight),
                            b'+' => Ok(MapTile::Intersection),
                            b'\\' => Ok(MapTile::ConnectWest),
                            b'/' => Ok(MapTile::ConnectEast),
                            _ => Err("invalid map tile found"),
                        })
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?;

            let row_width = path.iter().map(Vec::len).max().unwrap() as PointScalar;
            let grid = Grid::from_corners(
                (0, 0).into_pt(),
                (row_width - 1, path.len() as PointScalar - 1).into_pt(),
            );

            Ok(Self { grid, path })
        }
    }

    impl Index<Point> for CartTrack {
        type Output = MapTile;

        fn index(&self, index: Point) -> &Self::Output {
            if !self.grid.contains(index) {
                panic!("point not on grid!")
            }
            let origin = self.grid.bottom_left();
            &self.path[(index.y - origin.y) as usize][(index.x - origin.x) as usize]
        }
    }

    impl TrackSimulator {
        /// Creates a new `TrackSimulator` to analyze the given puzzle input.
        pub fn from_puzzle_input(input: &str) -> Result<Self, &'static str> {
            let track: CartTrack = input.parse()?;
            let carts = find_carts(input);
            Ok(Self {
                track,
                carts,
                tick: 0,
            })
        }

        /// Advance the track simulation until a collision between two carts occurs.
        ///
        /// If multiple collisions occur during the same simulation tick, all
        /// of the collisions will be processed and the point of the collision that
        /// occurred first (i.e. the upper-left most collision) will be returned.
        ///
        /// Solves Part 1 of the puzzle.
        pub fn run_until_next_collision(&mut self) -> Result<(TimeTick, Point), &'static str> {
            loop {
                if let Some(point) = self.run_tick()?.first() {
                    return Ok((self.tick, *point));
                }
            }
        }

        /// Advances the track simulation until there is only a single cart left.
        ///
        /// Note: this function assumes that the carts *will* eventually collide.
        /// Bad puzzle inputs may result in an infinite loop.
        pub fn run_until_last_cart(&mut self) -> Result<(TimeTick, Point), &'static str> {
            while self.active_carts().len() > 1 {
                self.run_tick()?;
            }

            self.active_carts()
                .first()
                .ok_or("no final cart - all remaining carts collided during the same tick")
                .map(|cart| (self.tick, cart.pos))
        }

        /// Advances this simulation by a single time tick.
        ///
        /// The positions of all cart collisions that occur during this time
        /// tick are returned. This collection will be empty if no carts
        /// collide.
        pub fn run_tick(&mut self) -> Result<Vec<Point>, &'static str> {
            let mut collisions = Vec::new();
            self.tick += 1;

            // Loop via index since we may need to mutate arbitrary elements
            // during a loop.
            for i in 0..self.carts.len() {
                if self.carts[i].collided {
                    continue;
                }

                self.carts[i].advance();

                let current_tile = self.track[self.carts[i].pos];
                self.carts[i].apply_turn(current_tile);

                // Iterate over all of the carts (ignoring the one that just moved)
                // to check for a collision.
                for j in 0..self.carts.len() {
                    if i == j || self.carts[j].collided {
                        // Skip the reference cart, since it is always "in a
                        // collision" with itself.
                        // Also ignore carts that have already collided.
                        continue;
                    }
                    if self.carts[i].pos == self.carts[j].pos {
                        // A new collision has occurred.
                        // Mark both of the carts involved as having collided
                        // and add the point of collisions to the collection of
                        // collisions that occured during this tic.
                        self.carts[i].collided = true;
                        self.carts[j].collided = true;

                        collisions.push(self.carts[i].pos);
                    }
                }
            }

            // Sort the carts so that the upper-left most carts will move first
            // during the next tick.
            self.carts
                .sort_unstable_by_key(|&Cart { pos, .. }| (pos.y, pos.x));

            Ok(collisions)
        }

        /// Returns a collection of references to all of the carts in this
        /// simulation that have not yet collided.
        fn active_carts(&self) -> Vec<&Cart> {
            self.carts.iter().filter(|&c| !c.collided).collect()
        }
    }

    /// Returns a sequence of `Cart` instances representing the positions of
    /// carts in the given puzzle input.
    fn find_carts(s: &str) -> Vec<Cart> {
        let base_cart = Cart {
            turn_preference: Turn::Left,
            pos: Pt::origin(),
            facing: Direction::North,
            collided: false,
        };
        s.lines()
            .enumerate()
            // Map each (row number, line) pair into a Vector of points, which
            // are flattened into one vector
            .flat_map(|(row, line)| {
                // Map each (col number, tile) pair into a Cart if the tile
                // represents a cart. Otherwise, ignore it.
                line.bytes()
                    .enumerate()
                    .filter_map(|(col, tile)| {
                        let point = (col as PointScalar, row as PointScalar).into_pt();
                        match tile {
                            b'^' => Some(Cart {
                                pos: point,
                                facing: Direction::North,
                                ..base_cart
                            }),
                            b'>' => Some(Cart {
                                pos: point,
                                facing: Direction::East,
                                ..base_cart
                            }),
                            b'v' => Some(Cart {
                                pos: point,
                                facing: Direction::South,
                                ..base_cart
                            }),
                            b'<' => Some(Cart {
                                pos: point,
                                facing: Direction::West,
                                ..base_cart
                            }),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input = puzzle::fetch_string(puzzle)?;
    let mut sim = cart::TrackSimulator::from_puzzle_input(&input)?;

    solve_parts!(
        1 => {
            let (_, collision) = sim.run_until_next_collision()?;
            format!("{},{}", collision.x, collision.y)
        },
        2 => {
            let (_, last_cart) = sim.run_until_last_cart()?;
            format!("{},{}", last_cart.x, last_cart.y)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::util::IntoPoint;

    #[test]
    fn solution() {
        assert_solution!("33,69", "135,9", puzzle::Selection::new(2018, 13),)
    }

    #[test]
    fn ex1() {
        const INPUT_1: &str = "|\n\
                               v\n\
                               |\n\
                               |\n\
                               |\n\
                               ^\n\
                               |";

        let (tick_count, collision) = cart::TrackSimulator::from_puzzle_input(INPUT_1)
            .unwrap()
            .run_until_next_collision()
            .unwrap();
        assert_eq!(2, tick_count);
        assert_eq!((0, 3).into_pt(), collision);

        const INPUT_2: &str = "/->-\\\n\
                               |   |  /----\\\n\
                               | /-+--+-\\  |\n\
                               | | |  | v  |\n\
                               \\-+-/  \\-+--/\n\
                               \\------/ ";
        let (tick_count, collision) = cart::TrackSimulator::from_puzzle_input(INPUT_2)
            .unwrap()
            .run_until_next_collision()
            .unwrap();
        assert_eq!(14, tick_count);
        assert_eq!((7, 3).into_pt(), collision);
    }

    #[test]
    fn ex2() {
        const INPUT: &str = "/>-<\\
|   |
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/";
        let (tick_count, final_cart) = cart::TrackSimulator::from_puzzle_input(&INPUT)
            .unwrap()
            .run_until_last_cart()
            .unwrap();
        assert_eq!(3, tick_count);
        assert_eq!((6, 4).into_pt(), final_cart);
    }
}
