//! Shared utility functions
use std::{fs::File, io::Read};
use std::collections::HashMap;

/// Read a file to a string.
pub fn read_file(filename: &str) -> String {
    let mut input = String::new();
    let mut file = File::open(format!("input/{}", filename)).unwrap();
    file.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    println!("Hello, world!");
    let mut left_list = vec![];
    let mut right_list = vec![];

    // Load file "input/day1-test" line by line. Parse it with the regex "^(\d+) +(\d+)$".
    let file = read_file("aoc1-in2.txt");
    for line in file.lines() {
        use regex::Regex;
        let re = Regex::new(r"^(\d+) +(\d+)$").unwrap();

        let caps = re.captures(line).unwrap();
        let left : i128 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right : i128 = caps.get(2).unwrap().as_str().parse().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    // Sort both lists
    left_list.sort();
    right_list.sort();

    // Find the total distance
    let mut total_distance = 0;
    for i in 0..left_list.len() {
        total_distance += (left_list[i] - right_list[i]).abs();
    }
    println!("Total distance: {}", total_distance);
    // assert!(total_distance == 1222801);

    // Part 2: Similarity score
    // Figure out how often each number appears in the right list.
    let mut map : HashMap<i128, i128> = HashMap::new();
    for &x in right_list.iter() {
        *map.entry(x).or_insert(0) += 1;
    }

    // Find the similarity score
    let mut similarity_score = 0;
    for i in 0..left_list.len() {
        similarity_score += map.get(&left_list[i]).cloned().unwrap_or( 0 ) * left_list[i];
    }
    println!("Similarity score: {}", similarity_score);
}
