use std::collections::VecDeque;

use aoc2023::solve_day;

fn main() {
    solve_day(9, part1, part2);
}

fn construct_histories(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|number| number.parse().ok())
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> Result<u64, ()> {
    let histories = construct_histories(input);

    let next_value_sum = histories
        .iter()
        .map(|history| {
            let mut sequences: Vec<Vec<i32>> = vec![history.clone()];

            loop {
                let last = &sequences[sequences.len() - 1];

                let next_sequence: Vec<i32> = last
                    .iter()
                    .enumerate()
                    .map_while(|(index, &element)| {
                        last.get(index + 1)
                            .map(|&next_element| next_element - element)
                    })
                    .collect();

                let finished = next_sequence.iter().all(|&element| element == 0);

                sequences.push(next_sequence);

                if finished {
                    break;
                }
            }

            for index in (0..sequences.len()).rev() {
                let sequence = &sequences[index];
                let sequence_last_element = sequence[sequence.len() - 1];

                let Some(previous_sequence) = index
                    .checked_sub(1)
                    .map(|previous_index| &mut sequences[previous_index])
                else {
                    break;
                };

                let previous_sequence_last_element = previous_sequence[previous_sequence.len() - 1];

                previous_sequence.push(sequence_last_element + previous_sequence_last_element);
            }

            sequences[0][history.len()]
        })
        .sum::<i32>() as u64;

    Ok(next_value_sum)
}

fn part2(input: &str) -> Result<u64, ()> {
    let histories = construct_histories(input);

    let previous_value_sum = histories
        .iter()
        .map(|history| {
            let mut sequences: Vec<VecDeque<i32>> = vec![VecDeque::from(history.clone())];

            loop {
                let last = &sequences[sequences.len() - 1];

                let next_sequence: VecDeque<i32> = last
                    .iter()
                    .enumerate()
                    .map_while(|(index, &element)| {
                        last.get(index + 1)
                            .map(|&next_element| next_element - element)
                    })
                    .collect();

                let finished = next_sequence.iter().all(|&element| element == 0);

                sequences.push(next_sequence);

                if finished {
                    break;
                }
            }

            for index in (0..sequences.len()).rev() {
                let sequence = &sequences[index];
                let sequence_first_element = sequence[0];

                let Some(previous_sequence) = index
                    .checked_sub(1)
                    .map(|previous_index| &mut sequences[previous_index])
                else {
                    break;
                };

                let previous_sequence_first_element = previous_sequence[0];

                previous_sequence
                    .push_front(previous_sequence_first_element - sequence_first_element);
            }

            sequences[0][0]
        })
        .sum::<i32>() as u64;

    Ok(previous_value_sum)
}
