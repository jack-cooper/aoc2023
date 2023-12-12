use std::{collections::HashMap, str::FromStr};

use aoc2023::solve_day;

fn main() {
    solve_day(7, part1, part2);
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u64,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Card {
    Two, // m Fwenwick
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, // COOPER
    Queen,
    King,
    Ace,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        let cards: Vec<Card> = chars[0..5]
            .iter()
            .filter_map(|&char| Card::try_from(char).ok())
            .collect();

        assert_eq!(cards.len(), 5);
        let cards: [Card; 5] = cards.try_into().expect("We asserted len was 5 above.");

        let card_counts = cards.iter().copied().fold(
            HashMap::new(),
            |mut card_counts: HashMap<Card, u8>, card| {
                *card_counts.entry(card).or_default() += 1;

                card_counts
            },
        );

        let hand_type = match card_counts.values().copied().max().expect(
            "Card counts was constructed from a 5 element array, it will always have some entries.",
        ) {
            1 => HandType::HighCard,
            2 => {
                if card_counts.values().filter(|&&count| count == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            3 => {
                if card_counts.values().any(|&count| count == 2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => unreachable!(),
        };

        let bid: String = chars[6..].iter().collect();
        let bid: u64 = bid.parse().expect("A bid should be a valid u64.");

        Ok(Hand {
            hand_type,
            cards,
            bid,
        })
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(()),
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input.lines().filter_map(|line| line.parse().ok()).collect();

    hands.sort_unstable();

    hands
        .into_iter()
        .enumerate()
        .fold(0_u64, |total_winnings, (index, Hand { bid, .. })| {
            let rank = index as u64 + 1;

            total_winnings + bid * rank
        })
}

fn part2(input: &str) -> u64 {
    0
}
