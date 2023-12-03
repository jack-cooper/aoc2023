use aoc2023::solve_day;
use grid::Grid;

fn main() {
    solve_day(3, part1, part2);
}

fn part1(input: &str) -> u32 {
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
                    let row = &grid.rows()[y];
                    let char = row[x];

                    !(char.is_ascii_digit() || char == '.')
                })
            {
                part_number_sum += current_number
                    .parse::<u32>()
                    .expect("We only add ascii digits to `current_number`.");
            }

            current_number.clear();
        }
    });

    part_number_sum
}

fn part2(input: &str) -> u32 {
    0
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
