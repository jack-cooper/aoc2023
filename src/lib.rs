use std::fs;

const INITIALS: [&str; 4] = ["xmp", "jwc", "scb", "tmf"];

fn read_input_file(day: u8, initials: &str) -> String {
    fs::read_to_string(format!("input/day{day}/{initials}.txt"))
        .expect("You typo'd a filename (or sutin).")
}

pub fn solve_day(day: u8, part1: fn(&str) -> Result<u64, ()>, part2: fn(&str) -> Result<u64, ()>) {
    println!("===== Begin Part 1 =====");
    let _ = INITIALS.iter().try_for_each(|&initials| {
        let input = read_input_file(day, initials);

        let answer = part1(&input)?;
        println!("{}: {}", initials.to_uppercase(), answer);

        Ok::<(), ()>(())
    });
    println!("=====  End Part 1  =====");

    println!();

    println!("===== Begin Part 2 =====");
    let _ = INITIALS.iter().try_for_each(|&initials| {
        let input = read_input_file(day, initials);

        let answer = part2(&input)?;

        println!("{}: {}", initials.to_uppercase(), answer);

        Ok::<(), ()>(())
    });
    println!("=====  End Part 2  =====");
}
