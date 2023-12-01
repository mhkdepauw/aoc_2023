use aoc_2023::{read_lines, read_lines_split_on_empty_line_as_i32, read_split_on_comma};
use regex::Match;

fn main() {
    let megavec = read_lines(2023, 1);
    println!(
        "part1:{}\npart2:{}",
        part1(megavec.clone()),
        part2(megavec.clone())
    );
}

fn part1(cords: Vec<String>) -> u32 {
    let mut totalsum: u32 = 0;
    for cord in cords {
        let mut cord_vec: Vec<u32> = cord.chars().filter_map(|c| c.to_digit(10)).collect();
        totalsum += cord_vec.last().unwrap();
        if cord_vec.len() > 1 {
            totalsum += cord_vec.first().unwrap() * 10;
        } else {
            totalsum += cord_vec.last().unwrap() * 10;
        }
    }
    return totalsum;
}

fn part2(cords: Vec<String>) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut totalsum: u32 = 0;
    for cord in cords {
        let mut first_digit = None;
        let mut last_digit = None;
        for char in 0..cord.len() {
            let mut digit = None;
            if cord.chars().nth(char).unwrap().is_digit(10) {
                digit = cord.chars().nth(char).unwrap().to_digit(10)
            } else {
                for word_end in char..cord.len() {
                    if numbers.contains(&&(cord[char..=word_end])) {
                        let number = numbers
                            .iter()
                            .position(|n| n == &&cord[char..=word_end])
                            .unwrap() as u32;
                        digit = Some(number + 1);
                    }
                }
            }
            match digit {
                Some(x) => {
                    if first_digit.is_none() {
                        first_digit = digit;
                    }
                    last_digit = digit;
                }

                _ => {}
            }
        }
        totalsum += (first_digit.unwrap() * 10) + last_digit.unwrap();
    }
    return totalsum;
}

// strong inspiration from https://github.com/Yendric/aoc_2023/blob/main/day-01/src/bin/part2.rs because I'm too rusty for this one

/*
cords
    .iter()
    .for_each(|string| string.chars().filter_map(|c| c.to_digit(10)).collect());
    */
