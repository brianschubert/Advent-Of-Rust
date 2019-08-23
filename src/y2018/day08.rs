//! Solution for Advent of Code [2018 Day 08](https://adventofcode.com/2018/day/8).

use crate::common::puzzle;

/// Module for working with "license file trees".
mod tree {
    /// The integral type used to represent data values in a tree.
    pub type Data = u32;

    #[derive(Debug, Eq, PartialEq)]
    /// A node a tree.
    pub struct Node {
        children: Vec<Node>,
        metadata: Vec<Data>,
    }

    impl Node {
        /// Constructs a tree from the given list of numbers.
        pub fn tree_from_number_list(desc: &[Data]) -> Node {
            let root_child_count = desc[0] as usize;
            let root_meta_count = desc[1] as usize;
            let (root_children, unused) = Self::children_from_list(&desc[2..], root_child_count);
            if unused.len() != root_meta_count {
                panic!(format!("root node has too much meta data: expected {} number, but {} numbers are left after parsing children", root_meta_count, unused.len()));
            }

            Self {
                children: root_children,
                metadata: Vec::from(unused),
            }
        }

        /// Parses `n=children_left` `Node`s from the given list of numbers
        /// and returns 1) the parsed nodes and 2) the numbers left after
        /// parsing `n` nodes.
        fn children_from_list(mut desc: &[Data], mut children_left: usize) -> (Vec<Node>, &[Data]) {
            // 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
            let mut nodes = Vec::new();

            while children_left != 0 {
                // Look at the next child in the description
                let child_count = desc[0] as usize;
                let meta_count = desc[1] as usize;

                if child_count == 0 {
                    let (metadata, rest) = desc[2..].split_at(meta_count);
                    nodes.push(Node {
                        children: Vec::new(),
                        metadata: Vec::from(metadata),
                    });
                    desc = rest;
                } else {
                    let (children, rest) = Self::children_from_list(&desc[2..], child_count);
                    let (metadata, unused) = rest.split_at(meta_count);
                    nodes.push(Node {
                        children,
                        metadata: Vec::from(metadata),
                    });
                    desc = unused
                }
                children_left -= 1;
            }
            (nodes, desc)
        }

        /// Returns the sum of all of the metadata entries contained
        /// in this `Node`'s branch.
        pub fn branch_metadata_sum(&self) -> Data {
            self.metadata.iter().sum::<Data>()
                + self
                    .children
                    .iter()
                    .map(Node::branch_metadata_sum)
                    .sum::<Data>()
        }

        /// Computes the value of this `Node`.
        pub fn compute_node_value(&self) -> Data {
            if self.children.is_empty() {
                self.metadata.iter().sum::<Data>()
            } else {
                self.metadata
                    .iter()
                    .map(|&index| {
                        let index = index as usize;
                        if index == 0 || index > self.children.len() {
                            0
                        } else {
                            // Note - children that are referenced multiple times
                            // will have their value computed each time they are
                            // referenced
                            self.children[index - 1].compute_node_value()
                        }
                    })
                    .sum()
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use super::super::tests::EXAMPLE_TREE_DESC;

        #[test]
        fn parse_tree_from_number_list() {
            let root_node = Node::tree_from_number_list(EXAMPLE_TREE_DESC);
            let excepted_tree = Node {
                children: vec![
                    Node {
                        children: vec![],
                        metadata: vec![10, 11, 12],
                    },
                    Node {
                        children: vec![Node {
                            children: vec![],
                            metadata: vec![99],
                        }],
                        metadata: vec![2],
                    },
                ],
                metadata: vec![1, 1, 2],
            };
            assert_eq!(root_node, excepted_tree);
        }
    }
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input: Vec<tree::Data> = puzzle::fetch_string(puzzle)?
        .trim_end()
        .split(' ')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let root_node = tree::Node::tree_from_number_list(&input);

    solve_parts!(
        1 => root_node.branch_metadata_sum(),
        2 => root_node.compute_node_value(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE_TREE_DESC: &[tree::Data] =
        &[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];

    #[test]
    fn solution() {
        assert_solution!(48260, 25981, puzzle::Selection::new(2018, 8))
    }

    #[test]
    fn ex1() {
        let root_node = tree::Node::tree_from_number_list(EXAMPLE_TREE_DESC);
        assert_eq!(138, root_node.branch_metadata_sum());
    }

    #[test]
    fn ex2() {
        let root_node = tree::Node::tree_from_number_list(EXAMPLE_TREE_DESC);
        assert_eq!(66, root_node.compute_node_value());
    }
}
