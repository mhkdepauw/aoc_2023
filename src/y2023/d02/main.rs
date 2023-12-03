use std::collections::HashMap;
use std::iter::Map;
use std::ops::Index;
use std::process::id;
use regex::Regex;
use aoc_2023::{read_lines, regex_parse, regex_parse_as_i32};

fn main() {
    let megavec: Vec<String> = read_lines(2023, 2);
    println!(
        "part1:{}\npart2:{}",
        part1(megavec.clone()),
        part2(megavec.clone())
    );
}

fn part1(vec: Vec<String>) -> i32 {
    let mut id_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let possible_cubes = HashMap::from([("red",12),("green",13),("blue",14)]);
    let re = Regex::new(r"(\d+) (red|green|blue);?").expect("Invalid RegEx");
    for line in &vec {
        let pulls: Vec<Vec<(i32, &str)>> = re
            .captures_iter(&*line)
            .map(|c| c.extract())
            .fold(vec![vec![]], |mut res, (group, [num, colour])| {
                res.last_mut().unwrap().push((num.parse().unwrap(), colour));
                if group.ends_with(";") {
                    res.push(vec![]);
                }
                return res;
            });
        let mut possible:bool = true;
        for set in &pulls {
            for cube in set{
                if cube.0> *possible_cubes.get(cube.1).unwrap() {
                    possible = false;
                    break
                }
            }
        }
        if !possible {
            id_vec.remove(vec.iter().position(| s| s.contains(&*line) ).unwrap());
        }
        println!("{pulls:?}");
    }
    return id_vec.iter().sum();
}

fn part2(vec: Vec<String>) -> i32 {
    return 0;
}

/*
let mut red: Vec<Vec<i32>> = vec![vec![0]];
let mut green: Vec<Vec<i32>> = vec![vec![0]];
let mut blue: Vec<Vec<i32>> = vec![vec![0]];
for game in 0..vec.len() {
    red.push(regex_parse_as_i32(vec.iter().nth(game).unwrap().as_str(), r"(\d+) red"));
    green.push(regex_parse_as_i32(vec.iter().nth(game).unwrap().as_str(), r"(\d+) blue"));
    blue.push(regex_parse_as_i32(vec.iter().nth(game).unwrap().as_str(), r"(\d+) green"));
}
red.remove(0);
green.remove(0);
blue.remove(0);
println!("red:{:?}\ngreen:{:?}\nblue:{:?}",red,green,blue);

 */