// Import lines
use core::str::Lines;
use std::{collections::HashSet};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <a/b> <input>", args[0]);
        std::process::exit(1);
    }
    let part = args[1].as_str();
    let input_file = args[2].as_str();
    let contents = std::fs::read_to_string(input_file).unwrap();
    let lines = contents.lines();
    let score = match part {
        "a" => do_a(lines),
        "b" => do_b(lines),
        _ => panic!("Invalid part"),
    };
    println!("Your score is {}", score.unwrap());
}

fn do_a(lines: Lines) -> Result<i32, String> {
    let mut total_score = 0;
    // Split each line into two parts.
    for line in lines {
        let halfway_point = line.len() / 2;
        let (first_half, second_half) = line.split_at(halfway_point);
        // Build a hash set from the letters in the first half.
        let first_rucksack: HashSet<char> = first_half.chars().collect();
        let second_rucksack: HashSet<char> = second_half.chars().collect();
        // Find the elements in common
        let common: HashSet<_> = first_rucksack.intersection(&second_rucksack).collect();
        if common.len() != 1 {
            return Err(String::from("There can only be one common element!"));
        };
        let common_element = match common.iter().next() {
            Some(common_element) => common_element,
            None => return Err(String::from("There must be one common element!")),
        };
        let current_score = match score_letter(common_element) {
            Ok(current_score) => current_score,
            Err(e) => return Err(e),
        };
        // Add the current score to the total score.
        total_score += current_score;
    }

    Ok(total_score)
}

fn score_letter(c: &char) -> Result<i32, String> {
    // Get the ascii integer value of the character.
    let raw_value = *c as i32;

    if *c >= 'a' && *c <= 'z' {
        Ok(raw_value - 96)
    } else if *c >= 'A' && *c <= 'Z' {
        Ok(raw_value - 64 + 26)
    } else {
        Err(String::from("Invalid letter"))
    }
}

fn do_b(lines: Lines) -> Result<i32, String> {
    let lines: Vec::<_> = lines.collect();
    if (lines.len() % 3) != 0 {
        return Err(String::from("Invalid number of lines"));
    }
    // Split the lines into groups of three.
    let groups: Vec<&[&str]> = lines.chunks(3).collect();
    let mut total_score = 0;
    for group in groups {
        let group = group.to_vec();
        let group_score = match score_group(group) {
            Ok(group_score) => group_score,
            Err(e) => return Err(e),
        };
        total_score += group_score;
    }
    Ok(total_score)
}

fn score_group(group: Vec<&str>) -> Result<i32, String> {
    let rucksacks: Vec<HashSet<char>> = group
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    let common = rucksacks
        .iter()
        .fold(rucksacks[0].clone(), |acc, x| {
            acc.intersection(x).cloned().collect()
        }
    );
    if common.len() != 1 {
        return Err(String::from("There can only be one common element!"));
    }
    let common_letter = match common.iter().next() {
        Some(common_letter) => common_letter,
        None => return Err(String::from("There must be one common element!")),
    };
    score_letter(common_letter)
}