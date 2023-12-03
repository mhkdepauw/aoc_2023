use aoc_2023::{read_lines, regex_parse, regex_parse_as_i32};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let megavec: Vec<String> = read_lines(2023, 2);
    println!(
        "part1:{}\npart2:{}",
        part1(megavec.clone()),
        part2(megavec.clone())
    );
}

fn part1(vec: Vec<String>) -> i32 {
    let mut id_vec: Vec<i32> = (1..(vec.len() as i32 + 1)).collect();
    let possible_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let re = Regex::new(r"(\d+) (red|green|blue);?").expect("Invalid RegEx");
    for line in &vec {
        let pulls: Vec<Vec<(i32, &str)>> = re.captures_iter(&*line).map(|c| c.extract()).fold(
            vec![vec![]],
            |mut res, (group, [num, colour])| {
                res.last_mut().unwrap().push((num.parse().unwrap(), colour));
                if group.ends_with(";") {
                    res.push(vec![]);
                }
                return res;
            },
        );
        let mut possible: bool = true;
        for set in &pulls {
            for cube in set {
                if cube.0 > *possible_cubes.get(cube.1).unwrap() {
                    possible = false;
                }
            }
        }
        if !possible {
            id_vec[vec.iter().position(|s| s.contains(&*line)).unwrap()] = 0;
        }
        println!("{pulls:?}");
    }
    println!("{:?}", id_vec);
    return id_vec.iter().sum();
}

fn part2(vec: Vec<String>) -> i32 {
    let mut games_cube_val = vec![1];
    let re = Regex::new(r"(\d+) (red|green|blue);?").expect("Invalid RegEx");
    for line in &vec {
        let pulls: Vec<Vec<(i32, &str)>> = re.captures_iter(&*line).map(|c| c.extract()).fold(
            vec![vec![]],
            |mut res, (group, [num, colour])| {
                res.last_mut().unwrap().push((num.parse().unwrap(), colour));
                if group.ends_with(";") {
                    res.push(vec![]);
                }
                return res;
            },
        );
        let mut min_cube_size = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for set in &pulls {
            for cube in set {
                if *min_cube_size.get(cube.1).unwrap() < cube.0 {
                    *min_cube_size.get_mut(cube.1).unwrap() = cube.0;
                }
            }
        }
        let mut multipl = 1;
        min_cube_size.values().for_each(|&num| (multipl *= num));
        games_cube_val.push(multipl);
        println!("{pulls:?}");
    }
    games_cube_val.remove(0);
    println!("{:?}", games_cube_val);
    return games_cube_val.iter().sum();
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
