use std::fs;

const XMAS: &str = "XMAS";
const XMAS_REVERSED: &str = "SAMX";

fn get_column(letters: Vec<String>, col_index: usize) -> String {
    letters
        .iter()
        .map(|line| line.chars().nth(col_index).unwrap())
        .collect()
}

fn part1(letters: Vec<String>) -> String {
    let mut total = 0;
    //horizontal
    for line in letters.clone() {
        total += line.match_indices(XMAS).count();
        total += line.match_indices(XMAS_REVERSED).count();
    }

    //vertical
    for col_index in 0..letters[0].len() {
        let column = get_column(letters.clone(), col_index);
        total += column.match_indices(XMAS).count();
        total += column.match_indices(XMAS_REVERSED).count();
    }

    //diagonal back
    let rows = letters.len();
    let cols = letters[0].len();

    for row in 0..rows - XMAS.len() + 1 {
        for col in 0..cols - XMAS.len() +1 {
            let mut diagonal1 = String::new();
                for i in 0..4 {
                    diagonal1.push(letters[row+i].chars().nth(col+i).unwrap());
                }
            total += diagonal1.match_indices(XMAS).count();
            total += diagonal1.match_indices(XMAS_REVERSED).count();
        }
    }

    //diagonal forward
    for row in 0 + XMAS.len()-1..rows {
        for col in 0..cols - XMAS.len() +1 {
            let mut diagonal1 = String::new();
            for i in 0..4 {
                diagonal1.push(letters[row-i].chars().nth(col+i).unwrap());
            }
            total += diagonal1.match_indices(XMAS).count();
            total += diagonal1.match_indices(XMAS_REVERSED).count();
        }
    }

    total.to_string()
}

fn part2(letters: Vec<String>) -> String {
    String::from("TODO")
}

pub fn day_four() {
    let contents = fs::read_to_string("input/day04").unwrap();
    let lines: Vec<String> = contents.lines().map(String::from).collect();

    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}
