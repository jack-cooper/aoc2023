use std::{collections::HashSet, str::FromStr};

use aoc2023::solve_day;

fn main() {
    solve_day(4, part1, part2);
}

struct Scratchcard {
    player_numbers: HashSet<u8>,
    winning_numbers: HashSet<u8>,
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, scratchcard) = s
            .split_once(':')
            .expect("A scratchcard should contain the character `:`.");

        let (winning_numbers, player_numbers) = scratchcard
            .split_once('|')
            .expect("A scratchcard should contain the character `|`.");

        Ok(Self {
            player_numbers: player_numbers
                .split_whitespace()
                .flat_map(|number| number.parse::<u8>())
                .collect(),
            winning_numbers: winning_numbers
                .split_whitespace()
                .flat_map(|number| number.parse::<u8>())
                .collect(),
        })
    }
}

impl Scratchcard {
    fn score(&self) -> u32 {
        let winning_number_count = self
            .player_numbers
            .intersection(&self.winning_numbers)
            .count() as u32;

        if winning_number_count == 0 {
            0
        } else {
            2_u32.pow(winning_number_count - 1)
        }
    }
}
fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|scratchcard| scratchcard.parse::<Scratchcard>())
        .map(|scratchcard| scratchcard.score())
        .sum()
}

fn part2(input: &str) -> u32 {
    0
}
