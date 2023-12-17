use std::{collections::HashMap, str::FromStr};

use aoc2023::solve_day;

fn main() {
    solve_day(2, part1, part2);
}

struct Game {
    id: u64,
    hands: Vec<Hand>,
}

struct Hand {
    cube_counts: HashMap<Color, u64>,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Color {
    Blue,
    Green,
    Red,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, game) = s
            .split_once(':')
            .expect("Game did not have a `:` character.");

        let id = id.replace("Game ", "");
        let id: u64 = id.parse().expect("Game provided a non-numeric ID.");

        let hands = game.split(';');

        let hands = hands.map(|hand| {
            hand.split(',').map(|cube_count| {
                let (count, color) = cube_count
                    .trim_start()
                    .split_once(' ')
                    .expect("A cube count was missing a space.");

                let count: u64 = count.parse().expect("A non numeric count was given.");
                let color: Color = color.parse().expect("An unrecognised color was found.");

                (color, count)
            })
        });

        let hands = hands.map(|hand| hand.collect::<Hand>());

        Ok(Game {
            id,
            hands: hands.collect(),
        })
    }
}

impl FromIterator<(Color, u64)> for Hand {
    fn from_iter<T: IntoIterator<Item = (Color, u64)>>(iter: T) -> Self {
        Self {
            cube_counts: iter.into_iter().collect(),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            "red" => Ok(Self::Red),
            _ => Err(()),
        }
    }
}

fn make_games(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(|line| {
        line.parse::<Game>()
            .expect("Game either panics or returns Ok.")
    })
}

fn part1(input: &str) -> Result<u64, ()> {
    Ok(make_games(input)
        .filter_map(|Game { id, hands }| {
            hands
                .iter()
                .all(|Hand { cube_counts }| {
                    cube_counts.get(&Color::Blue).copied().unwrap_or_default() <= 14
                        && cube_counts.get(&Color::Green).copied().unwrap_or_default() <= 13
                        && cube_counts.get(&Color::Red).copied().unwrap_or_default() <= 12
                })
                .then_some(id)
        })
        .sum())
}

fn part2(input: &str) -> Result<u64, ()> {
    Ok(make_games(input)
        .map(|Game { hands, .. }| {
            let maxima = hands.iter().fold(
                HashMap::from([(Color::Blue, 0), (Color::Green, 0), (Color::Red, 0)]),
                |mut maxima, Hand { cube_counts }| {
                    [Color::Blue, Color::Green, Color::Red]
                        .into_iter()
                        .for_each(|color| {
                            maxima.insert(
                                color,
                                maxima[&color]
                                    .max(cube_counts.get(&color).copied().unwrap_or_default()),
                            );
                        });

                    maxima
                },
            );

            maxima.values().product::<u64>()
        })
        .sum())
}
