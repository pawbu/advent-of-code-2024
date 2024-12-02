use std::fs;


fn part1(pairs: Vec<Vec<u32>>) -> String {
    let mut left_list = pairs.iter().map(|p| p[0]).collect::<Vec<u32>>();
    let mut right_list = pairs.iter().map(|p| p[1]).collect::<Vec<u32>>();

    left_list.sort();
    right_list.sort();

    let mut diffs = Vec::new();
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        if left > right {
            diffs.push(left - right);
        } else {
            diffs.push(right - left);
        }
    }

    diffs
        .iter()
        .sum::<u32>()
        .to_string()
}

fn part2(pairs: Vec<Vec<u32>>) -> String {
    let mut left_list = pairs.iter().map(|p| p[0]).collect::<Vec<u32>>();
    let mut right_list = pairs.iter().map(|p| p[1]).collect::<Vec<u32>>();

    left_list.into_iter()
        .map(|left| left * (right_list.iter().filter(|e| **e == left).count() as u32))
        .sum::<u32>()
        .to_string()
}


pub fn day_one() {
    let contents = fs::read_to_string("input/day01").unwrap();
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    let pairs: Vec<Vec<u32>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<u32>>()
        })
        .collect();

    println!("Part 1: {}", part1(pairs.clone()));
    println!("Part 2: {}", part2(pairs.clone()));
}
