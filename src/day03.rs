use regex::Regex;
use std::fs;

fn part1(contents: String) -> String {
    let re = Regex::new(r"(?<m>(mul\((?<d1>(\d+)),(?<d2>(\d+)))\))").unwrap();
    re.captures_iter(&contents)
        .map(|caps| {
            let d1 = caps.name("d1").unwrap().as_str().parse::<u32>().unwrap();
            let d2 = caps.name("d2").unwrap().as_str().parse::<u32>().unwrap();
            d1 * d2
        })
        .sum::<u32>()
        .to_string()
}

fn part2(contents: String) -> String {
    String::from("TODO")
}

pub fn day_three() {
    let contents = fs::read_to_string("input/day03").unwrap();

    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: {}", part2(contents.clone()));
}
