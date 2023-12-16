use std::{collections::HashMap, iter, ops::ControlFlow, str::FromStr};

use aoc2023::solve_day;

fn main() {
    solve_day(8, part1, part2)
}

struct Map {
    instructions: Vec<Instruction>,
    network: HashMap<Node, (Node, Node)>,
}

enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Node(String);

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let instructions = lines
            .next()
            .expect("The instruction line was not provided in the input.");

        let instructions: Result<Vec<Instruction>, ()> =
            instructions.chars().map(Instruction::try_from).collect();

        let instructions = instructions?;

        // Skip the empty line
        lines.next();

        let network: HashMap<Node, (Node, Node)> = lines
            .map(|line| {
                let chars: Vec<char> = line.chars().collect();

                (
                    Node(chars[0..3].iter().collect()),
                    (
                        Node(chars[7..10].iter().collect()),
                        Node(chars[12..15].iter().collect()),
                    ),
                )
            })
            .collect();

        Ok(Self {
            instructions,
            network,
        })
    }
}

#[derive(Default)]
struct Cycle {
    start_index: Option<u64>,
    end_index: Option<u64>,
}

impl Map {
    fn cycles(&self) -> impl Iterator<Item = Cycle> + '_ {
        let mut current_nodes: Vec<&Node> = self
            .network
            .keys()
            .filter(|Node(identifier)| identifier.ends_with('A'))
            .collect();

        let mut ending_nodes: HashMap<&Node, Cycle> = self
            .network
            .keys()
            .filter(|Node(identifier)| identifier.ends_with('Z'))
            .zip(iter::repeat_with(Cycle::default))
            .collect();

        let instruction_len = self.instructions.len() as u64;

        (1_u64..)
            .zip(self.instructions.iter().cycle())
            .try_for_each(|(index, instruction)| {
                current_nodes.iter_mut().for_each(|node_ref| {
                    let (left, right) = &self.network[node_ref];

                    *node_ref = match instruction {
                        Instruction::Left => left,
                        Instruction::Right => right,
                    };

                    if let Some(cycle) = ending_nodes.get_mut(*node_ref) {
                        if let Some(start_index) = cycle.start_index {
                            if cycle.end_index.is_none()
                                && (index % instruction_len == start_index % instruction_len)
                            {
                                cycle.end_index = Some(index);
                            }
                        } else {
                            cycle.start_index = Some(index);
                        }
                    }
                });

                if ending_nodes
                    .values()
                    .all(|Cycle { end_index, .. }| end_index.is_some())
                {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });

        ending_nodes.into_values()
    }

    fn steps_to_end(&self) -> u64 {
        let ControlFlow::Break((step_count, _)) = self.instructions.iter().cycle().try_fold(
            (0, &Node(String::from("AAA"))),
            |(step_count, current_node), instruction| {
                if current_node == &Node(String::from("ZZZ")) {
                    return ControlFlow::Break((step_count, current_node));
                }

                let (left, right) = &self.network[&current_node];

                ControlFlow::Continue(match instruction {
                    Instruction::Left => (step_count + 1, left),
                    Instruction::Right => (step_count + 1, right),
                })
            },
        ) else {
            unreachable!("A cycling iterator will never finish without breaking.")
        };

        step_count
    }
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

fn part1(input: &str) -> u64 {
    let map: Map = input.parse().expect("Badly formatted input was provided.");

    map.steps_to_end()
}

fn part2(input: &str) -> u64 {
    let map: Map = input.parse().expect("Badly formatted input was provided.");

    let start_node_count = map
        .network
        .keys()
        .filter(|Node(identifier)| identifier.ends_with('A'))
        .count();

    let cycles: Vec<Cycle> = map.cycles().collect();
    cycles.iter().for_each(|cycle| {
        let Some((start_index, end_index)) = cycle.start_index.zip(cycle.end_index) else {
            unreachable!();
        };

        assert_eq!(end_index % start_index, 0);
    });

    let start_indices: Vec<u64> = map.cycles().filter_map(|cycle| cycle.start_index).collect();
    assert_eq!(start_indices.len(), start_node_count);

    start_indices
        .iter()
        .copied()
        .reduce(|lcm, start_index| {
            let a = lcm.max(start_index);
            let b = lcm.min(start_index);

            a * (b / gcd_euclidean(a, b))
        })
        .expect("`start_indices` has a guaranteed non-zero length.")
}

fn gcd_euclidean(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_euclidean(b, a % b)
    }
}
