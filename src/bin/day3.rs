use aoc2023::solve_day;
use grid::Grid;

fn main() {
    solve_day(3, part1, part2);
}

fn part1(input: &str) -> u64 {
    let grid: Grid = input.parse().expect("Grid parsing never fails.");

    let mut part_number_sum = 0;

    let mut current_number = String::from("");
    let mut current_number_index: Option<(usize, usize)> = None;

    grid.iter().for_each(|((x, y), char)| {
        if char.is_ascii_digit() {
            current_number.push(char);
            current_number_index = current_number_index.or(Some((x, y)));
        }

        if !current_number.is_empty() && (!char.is_ascii_digit() || x == grid.width() - 1) {
            let (x, y) = current_number_index
                .take()
                .expect("`current_number_index` will always have been set.");

            if grid
                .neighbors(x, y, current_number.len())
                .into_iter()
                .any(|(x, y)| {
                    let char = grid.char_at(x, y);

                    !(char.is_ascii_digit() || char == '.')
                })
            {
                part_number_sum += current_number
                    .parse::<u64>()
                    .expect("We only add ascii digits to `current_number`.");
            }

            current_number.clear();
        }
    });

    part_number_sum
}

fn part2(input: &str) -> u64 {
    let grid: Grid = input.parse().expect("Grid parsing never fails.");

    let gear_ratio_sum: u64 = grid
        .iter()
        .filter(|(_, char)| *char == '*')
        .map(|((x, y), _)| {
            let mut neighbors = grid.neighbors(x, y, 1);
            neighbors.retain(|&(x, y)| grid.char_at(x, y).is_ascii_digit());

            let mut part_number_coordinates = Vec::with_capacity(8);

            let mut coordinates = (x, y.wrapping_sub(1));
            if neighbors.contains(&coordinates) {
                part_number_coordinates.push(coordinates);
            } else {
                coordinates = (x.wrapping_sub(1), y.wrapping_sub(1));
                if neighbors.contains(&coordinates) {
                    part_number_coordinates.push(coordinates);
                }

                coordinates = (x + 1, y.wrapping_sub(1));
                if neighbors.contains(&coordinates) {
                    part_number_coordinates.push(coordinates);
                }
            }

            coordinates = (x, y + 1);
            if neighbors.contains(&coordinates) {
                part_number_coordinates.push(coordinates);
            } else {
                coordinates = (x.wrapping_sub(1), y + 1);
                if neighbors.contains(&coordinates) {
                    part_number_coordinates.push(coordinates);
                }

                coordinates = (x + 1, y + 1);
                if neighbors.contains(&coordinates) {
                    part_number_coordinates.push(coordinates);
                }
            }

            coordinates = (x.wrapping_sub(1), y);
            if neighbors.contains(&coordinates) {
                part_number_coordinates.push(coordinates);
            }
            coordinates = (x + 1, y);
            if neighbors.contains(&coordinates) {
                part_number_coordinates.push(coordinates);
            }

            if part_number_coordinates.len() != 2 {
                return 0;
            }

            let gear_ratio: u64 = part_number_coordinates
                .into_iter()
                .map(|(mut x, y)| {
                    let mut part_number = String::from(grid.char_at(x, y));

                    let start_x = x;

                    while x > 0 && grid.char_at(x - 1, y).is_ascii_digit() {
                        let char = grid.char_at(x - 1, y);
                        part_number = format!("{char}{part_number}");

                        x -= 1;
                    }

                    x = start_x;

                    while x < grid.width() - 1 && grid.char_at(x + 1, y).is_ascii_digit() {
                        let char = grid.char_at(x + 1, y);
                        part_number.push(char);

                        x += 1;
                    }

                    part_number
                        .parse::<u64>()
                        .expect("We only constructed `part_number` from ascii digits.")
                })
                .product();

            gear_ratio
        })
        .sum();

    gear_ratio_sum
}

mod grid {
    use std::{collections::BTreeSet, iter, str::FromStr};

    pub(crate) struct Grid {
        rows: Vec<Vec<char>>,
        width: usize,
    }

    impl FromStr for Grid {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let rows: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

            let width = rows[0].len();

            assert!(rows.iter().all(|row| row.len() == width));

            Ok(Self { rows, width })
        }
    }

    impl Grid {
        pub(crate) fn char_at(&self, x: usize, y: usize) -> char {
            let row = &self.rows()[y];
            row[x]
        }

        pub(crate) fn height(&self) -> usize {
            self.rows.len()
        }

        /// Returns an iterator giving out ((x, y), cell value) items.
        pub(crate) fn iter(&self) -> impl Iterator<Item = ((usize, usize), char)> + '_ {
            self.rows.iter().enumerate().flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(column_index, &cell)| ((column_index, row_index), cell))
            })
        }

        pub(crate) fn neighbors(
            &self,
            x: usize,
            y: usize,
            length: usize,
        ) -> BTreeSet<(usize, usize)> {
            let mut neighbors = BTreeSet::default();

            let first_column = x == 0;
            let first_row = y == 0;
            let last_column = x + length == self.width;
            let last_row = y == self.height() - 1;

            if !(first_column || first_row) {
                neighbors.insert((x - 1, y - 1));
            }

            if !(first_column || last_row) {
                neighbors.insert((x - 1, y + 1));
            }

            if !(last_column || first_row) {
                neighbors.insert((x + length, y - 1));
            }
            if !(last_column || last_row) {
                neighbors.insert((x + length, y + 1));
            }

            if !first_column {
                neighbors.insert((x - 1, y));
            }

            if !last_column {
                neighbors.insert((x + length, y));
            }

            if !first_row {
                neighbors.extend((x..(x + length)).zip(iter::repeat(y - 1)));
            }

            if !last_row {
                neighbors.extend((x..(x + length)).zip(iter::repeat(y + 1)));
            }

            neighbors
        }

        pub(crate) fn rows(&self) -> &[Vec<char>] {
            &self.rows
        }

        pub(crate) fn width(&self) -> usize {
            self.width
        }
    }
}
