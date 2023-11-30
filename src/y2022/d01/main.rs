use aoc_2023::read_lines_split_on_empty_line_as_i32;

fn main() {
    let megavec = read_lines_split_on_empty_line_as_i32(2022, 1);
    println!(
        "part1:{}\npart2:{}",
        part1(megavec.clone()),
        part2(megavec.clone())
    );
}

fn part1(vecvec: Vec<Vec<i32>>) -> i32 {
    let mut highest_comb_cal = 0;
    for vec in vecvec {
        let sum = vec.iter().sum();
        if sum > highest_comb_cal {
            highest_comb_cal = sum;
        }
    }
    return highest_comb_cal;
}
fn part2(vecvec: Vec<Vec<i32>>) -> i32 {
    let mut top_three = vec![0, 0, 0];
    vecvec.iter().for_each(|elf| {
        if elf.iter().sum::<i32>() > *top_three.iter().min().unwrap() {
            top_three.swap_remove(
                top_three
                    .iter()
                    .position(|e| e == top_three.iter().min().unwrap())
                    .unwrap(),
            );
            top_three.push(elf.iter().sum::<i32>())
        }
    });
    return top_three.iter().sum::<i32>();
}
