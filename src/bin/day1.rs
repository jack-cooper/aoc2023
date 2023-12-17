use std::str::FromStr;

use aoc2023::solve_day;

fn main() {
    solve_day(1, part1, part2);
}

fn part1(input: &str) -> Result<u64, ()> {
    Ok(input
        .trim()
        .lines()
        .map(|line| {
            let (first, last) = (
                line.matches(char::is_numeric)
                    .next()
                    .expect("Line had no numeric character."),
                line.rmatches(char::is_numeric)
                    .next()
                    .expect("Line had no numeric character."),
            );

            format!("{first}{last}")
                .parse::<u64>()
                .expect("We checked `char::is_numeric` above.")
        })
        .sum())
}

enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one" => Ok(Self::One),
            "two" => Ok(Self::Two),
            "three" => Ok(Self::Three),
            "four" => Ok(Self::Four),
            "five" => Ok(Self::Five),
            "six" => Ok(Self::Six),
            "seven" => Ok(Self::Seven),
            "eight" => Ok(Self::Eight),
            "nine" => Ok(Self::Nine),
            _ => Err(()),
        }
    }
}

impl From<Digit> for u64 {
    fn from(value: Digit) -> Self {
        match value {
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}

fn part2(input: &str) -> Result<u64, ()> {
    let valid_str_patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let lines = input.trim().lines();

    Ok(lines
        .map(|line| {
            let first_str_matches = valid_str_patterns
                .into_iter()
                .flat_map(|pattern| line.match_indices(pattern).next());
            let last_str_matches = valid_str_patterns
                .into_iter()
                .flat_map(|pattern| line.rmatch_indices(pattern).next());

            let first_digit_match = line.match_indices(char::is_numeric).next();
            let last_digit_match = line.rmatch_indices(char::is_numeric).next();

            let first_match = first_str_matches
                .chain(first_digit_match)
                .min_by_key(|(index, _)| *index)
                .map(|(_, str)| match str.parse::<u64>() {
                    Ok(integer) => integer,
                    Err(_) => str.parse::<Digit>().unwrap().into(),
                })
                .expect("Badly formatted line had no matches!");

            let last_match = last_str_matches
                .chain(last_digit_match)
                .max_by_key(|(index, _)| *index)
                .map(|(_, str)| match str.parse::<u64>() {
                    Ok(integer) => integer,
                    Err(_) => str.parse::<Digit>().unwrap().into(),
                })
                .expect("Badly formatted line had no matches!");

            first_match * 10 + last_match
        })
        .sum())
}
