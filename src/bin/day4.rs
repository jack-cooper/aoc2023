use std::{collections::HashSet, str::FromStr};

use aoc2023::solve_day;

fn main() {
    solve_day(4, part1, part2);
}

struct Scratchcard {
    id: u8,
    player_numbers: HashSet<u8>,
    winning_numbers: HashSet<u8>,
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_id, scratchcard) = s
            .split_once(':')
            .expect("A scratchcard should contain the character `:`.");

        let (_, card_id) = card_id
            .split_once(' ')
            .expect("A card ID should contain a space.");

        let id: u8 = card_id
            .trim_start()
            .parse()
            .expect("A card ID should be a valid u8.");

        let (winning_numbers, player_numbers) = scratchcard
            .split_once('|')
            .expect("A scratchcard should contain the character `|`.");

        Ok(Self {
            id,
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

impl Eq for Scratchcard {}

impl Ord for Scratchcard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Scratchcard {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Scratchcard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Scratchcard {
    fn score(&self) -> u64 {
        let winning_number_count = self.winning_number_count() as u32;

        if winning_number_count == 0 {
            0
        } else {
            2_u64.pow(winning_number_count - 1)
        }
    }

    fn winning_number_count(&self) -> usize {
        self.player_numbers
            .intersection(&self.winning_numbers)
            .count()
    }
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|scratchcard| scratchcard.parse())
        .map(|scratchcard: Scratchcard| scratchcard.score())
        .sum()
}

fn part2(input: &str) -> u64 {
    let scratchcards: Vec<Scratchcard> = input
        .lines()
        .flat_map(|scratchcard| scratchcard.parse())
        .collect();

    let mut counts: Vec<u64> = vec![1; scratchcards.len()];

    for (index, scratchcard) in scratchcards.into_iter().enumerate() {
        let winning_number_count = scratchcard.winning_number_count();

        let [scratchcard_count, next_scratchcard_counts @ ..] =
            &mut counts[index..(index + 1 + winning_number_count)]
        else {
            unreachable!("Cards will never make you copy a card past the end of the table.");
        };

        next_scratchcard_counts
            .iter_mut()
            .for_each(|next_scratchcard_count| {
                *next_scratchcard_count += *scratchcard_count;
            });
    }

    counts.into_iter().sum()
}
