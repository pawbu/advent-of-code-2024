use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Ops {
    Add,
    Mul,
}

static OPS: &[Ops] = &[Ops::Add, Ops::Mul];

fn part1(calibrations: Vec<Vec<u64>>) -> String {
    calibrations
        .iter()
        .filter_map(|line| {
            let (test_value, numbers) = line.split_at(1);
            for combo in generate_combinations_copilot(numbers.len()) {
                let mut total = numbers[0];
                for i in 1..numbers.len() {
                    let operation = combo.get(i).unwrap();
                    match operation {
                        Ops::Add => total += numbers[i],
                        Ops::Mul => total *= numbers[i],
                    }
                }
                if total == test_value[0] {
                    return Some(test_value[0]);
                }
            }
            None
        })
        .sum::<u64>()
        .to_string()
}

// works too!
// fn generate_combinations(size: usize) -> Vec<Vec<Ops>> {
//     if size == 0 {
//         return vec![vec![]];
//     }
//
//     let mut result = Vec::new();
//     let elements = [Ops::Add, Ops::Mul];
//
//     // Generate binary numbers from 0 to 2^size - 1
//     for i in 0..(1 << size) {
//         let mut combination = Vec::with_capacity(size);
//
//         // Convert binary number to combination
//         for j in 0..size {
//             if (i >> j) & 1 == 1 {
//                 combination.push(Ops::Add);
//             } else {
//                 combination.push(Ops::Mul);
//             }
//         }
//
//         result.push(combination);
//     }
//
//     result
// }

fn generate_combinations_copilot(x: usize) -> Vec<Vec<Ops>> {
    fn helper(current: &mut Vec<Ops>, x: usize, combinations: &mut Vec<Vec<Ops>>) {
        if current.len() == x {
            combinations.push(current.clone());
            return;
        }

        for &op in &[Ops::Add, Ops::Mul] {
            current.push(op);
            helper(current, x, combinations);
            current.pop();
        }
    }

    let mut combinations = Vec::new();
    let mut current = Vec::new();
    helper(&mut current, x, &mut combinations);
    combinations
}


pub fn day_seven() {
    let contents = fs::read_to_string("input/day07").unwrap();
    let calibrations: Vec<Vec<u64>> = contents
        .lines()
        .map(|line| {
            line.split(|c: char| c == ':' || c.is_whitespace())
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect();

    println!("Part 1: {}", part1(calibrations.clone()));
}
