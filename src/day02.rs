use std::fs;


fn is_increasing(vec: &Vec<u32>) -> bool {
    vec.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
}

fn is_decreasing(vec: &Vec<u32>) -> bool {
    vec.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
}

fn part1(numbers: Vec<Vec<u32>>) -> String {
    numbers.into_iter()
        .filter_map(|vec: Vec<u32>| {
            if is_increasing(&vec) || is_decreasing(&vec) {
                return Some(true);
            }
            return None;
        })
        .collect::<Vec<bool>>()
        .len()
        .to_string()
}


fn part2(numbers: Vec<Vec<u32>>) -> String {
    String::from("TODO")
}


pub fn day_two() {
    let contents = fs::read_to_string("input/day02").unwrap();
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    let numbers: Vec<Vec<u32>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<u32>>()
        })
        .collect();

    println!("Part 1: {}", part1(numbers.clone()));
    println!("Part 2: {}", part2(numbers.clone()));
}
