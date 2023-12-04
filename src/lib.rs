use std::fs;

const INITIALS: [&str; 4] = ["xmp", "jwc", "scb", "tmf"];

fn read_input_file(day: u8, initials: &str) -> String {
    fs::read_to_string(format!("input/day{day}/{initials}.txt"))
        .expect("You typo'd a filename (or sutin).")
}

pub fn solve_day(day: u8, part1: fn(&str) -> u32, part2: fn(&str) -> u32) {
    println!("===== Begin Part 1 =====");
    INITIALS.iter().for_each(|&initials| {
        let input = read_input_file(day, initials);

        println!("{}: {}", initials.to_uppercase(), part1(&input),);
    });
    println!("=====  End Part 1  =====");

    println!();

    println!("===== Begin Part 2 =====");
    INITIALS.iter().for_each(|&initials| {
        let input = read_input_file(day, initials);

        println!("{}: {}", initials.to_uppercase(), part2(&input),);
    });
    println!("=====  End Part 2  =====");
}
