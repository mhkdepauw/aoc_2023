use regex::Regex;
use std::fs::read_to_string;

pub fn read_string(year: i32, day: i32) -> String {
    let path = format!("src/y{}/d{:0>2}/input.txt", year, day);
    println!("{}", path);
    return read_to_string(path)
        .expect("Couldn't read file")
        .trim()
        .to_string();
}

pub fn read_lines(year: i32, day: i32) -> Vec<String> {
    return read_string(year, day)
        .lines()
        .map(|l| l.trim().to_string())
        .collect();
}

pub fn read_lines_as_i32(year: i32, day: i32) -> Vec<i32> {
    return read_lines(year, day)
        .into_iter()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();
}

pub fn read_split_on_empty_line(year: i32, day: i32) -> Vec<String> {
    return read_string(year, day)
        .split("\n\n")
        .map(|l| l.trim().to_string())
        .collect();
}

pub fn read_lines_split_on_empty_line(year: i32, day: i32) -> Vec<Vec<String>> {
    return read_split_on_empty_line(year, day)
        .iter()
        .map(|g| g.lines().map(|l| l.trim().to_string()).collect())
        .collect();
}

pub fn read_lines_split_on_empty_line_as_i32(year: i32, day: i32) -> Vec<Vec<i32>> {
    return read_lines_split_on_empty_line(year, day)
        .iter()
        .map(|g| g.iter().map(|l| l.parse::<i32>().unwrap()).collect())
        .collect();
}

pub fn read_split_on_comma(year: i32, day: i32) -> Vec<String> {
    return read_string(year, day)
        .split(",")
        .map(|l| l.trim().to_string())
        .collect();
}

pub fn read_split_on_comma_as_i32(year: i32, day: i32) -> Vec<i32> {
    return read_split_on_comma(year, day)
        .into_iter()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();
}

pub fn regex_parse(s: &str, regex_s: &str) -> Vec<String> {
    let re = Regex::new(regex_s).expect("Invalid RegEx");
    let caps = re.captures(s).unwrap();

    let mut res: Vec<String> = Vec::new();

    for i in 1..caps.len() {
        res.push(caps.get(i).unwrap().as_str().to_string());
    }

    return res;
}

pub fn regex_parse_as_i32(s: &str, regex_s: &str) -> Vec<i32> {
    return regex_parse(s, regex_s)
        .iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect();
}
/*
pub fn regex_parse_matches_i32(s: String, regex_s: &str) -> Vec<i32> {
    let re = Regex::new(regex_s).expect("Invalid RegEx");
    let mut vec:Vec<i32> = vec![];
    for (_, [first, second]) in re.captures_iter(s.as_str()).map(|c| c.extract()) {
        vec.push(second.parse::<i32>().expect("Parse error"));
    }
    return vec;
}

 */
/*
pub fn regex_parse_matches_as_i32(s: String, regex_s: &str) -> Vec<i32> {
    return regex_parse_matches(s, regex_s)
        .iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect();
}

 */


//credit to https://github.com/JonasssC/AoC-Rust/blob/main/src/lib.rs
