use std::str::FromStr;

use aoc2023::solve_day;

fn main() {
    solve_day(6, part1, part2);
}

struct BoatRaces {
    distances: Vec<u64>,
    times: Vec<u64>,
}

impl FromStr for BoatRaces {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut race_descriptors = s.lines().filter_map(|line| {
            line.split_once(':').map(|(_, values)| {
                values
                    .split(' ')
                    .filter_map(|value| value.parse().ok())
                    .collect()
            })
        });

        Ok(Self {
            times: race_descriptors
                .next()
                .expect("The input was missing the line describing race times."),
            distances: race_descriptors
                .next()
                .expect("The input was missing the line describing race distances."),
        })
    }
}

impl BoatRaces {
    fn iter(&self) -> BoatRacesIter {
        BoatRacesIter {
            boat_races: self,
            current_index: 0,
        }
    }
}

struct BoatRacesIter<'a> {
    boat_races: &'a BoatRaces,
    current_index: usize,
}

impl Iterator for BoatRacesIter<'_> {
    type Item = BoatRace;

    fn next(&mut self) -> Option<Self::Item> {
        let boat_race = self
            .boat_races
            .distances
            .get(self.current_index)
            .zip(self.boat_races.times.get(self.current_index))
            .map(|(&distance, &time)| BoatRace { distance, time });

        self.current_index += 1;

        boat_race
    }
}

struct BoatRace {
    distance: u64,
    time: u64,
}

impl FromStr for BoatRace {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut race_descriptor = s.lines().filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, values)| values.replace(' ', "").parse().ok())
        });

        Ok(Self {
            time: race_descriptor
                .next()
                .expect("The input was missing the line describing the race time."),
            distance: race_descriptor
                .next()
                .expect("The input was missing the line describing the race distance."),
        })
    }
}

impl BoatRace {
    fn first_winning_time(&self) -> u64 {
        (1..self.time)
            .find(|&button_holding_time| {
                button_holding_time * (self.time - button_holding_time) > self.distance
            })
            .expect("A race record should always be beatable.")
    }
}
fn part1(input: &str) -> Result<u64, ()> {
    let boat_races: BoatRaces = input.parse().expect("Badly formatted input was provided.");

    Ok(boat_races
        .iter()
        .map(|race| (race.time + 1) - 2 * race.first_winning_time())
        .product())
}

fn part2(input: &str) -> Result<u64, ()> {
    let race: BoatRace = input.parse().expect("Badly formatted input was provided.");

    Ok((race.time + 1) - 2 * race.first_winning_time())
}
